use poise::CreateReply;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, Set};
use sea_orm::sea_query::extension::mysql::IndexHintType::Use;
use crate::polymart::{generate_polymart_verify_url, get_resource_data, verify_user};
use crate::{get_config, get_connection, user, Context, Error, CONNECTION};
use crate::user::{ActiveModel, Entity as UserEntity, Model};



#[poise::command(slash_command, prefix_command)]
pub async fn generate(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let connection = get_connection().unwrap();

    if get_user_id(connection, ctx.author().id.to_string()).await?.is_some(){
        ctx.send(
            CreateReply::default()
                .content("You already have a polymart account linked.")
                .ephemeral(true)
                .reply(true)
        ).await?;
        return Ok(());
    }
    
    let data = generate_polymart_verify_url().await?;

    if !data.success{
        ctx.send(
            CreateReply::default()
                .content("Failed to get verify URL.")
                .ephemeral(true)
                .reply(true)
        ).await?;
        return Ok(());
    }

    let response = format!("You can verify your polymart account with [this link]({}).\nOnce done you can run ``/link`` with that same code.", data.url.unwrap());
    
    ctx.send(
        CreateReply::default()
            .content(response)
            .ephemeral(true)
            .reply(true)
    ).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn link(
    ctx: Context<'_>,
    code: String,
) -> Result<(), Error> {
    
    let regex = regex::Regex::new("[A-Za-z0-9]{3}-[A-Za-z0-9]{3}-[A-Za-z0-9]+").unwrap();

    if !regex.is_match(&code){
        ctx.send(
            CreateReply::default()
                .content("Invalid Code.")
                .ephemeral(true)
                .reply(true)
        ).await?;
        return Ok(());
    }

    let connection = get_connection().unwrap();

    if get_user_id(connection, ctx.author().id.to_string()).await?.is_some(){
        ctx.send(
            CreateReply::default()
                .content("You already have a polymart account linked.")
                .ephemeral(true)
                .reply(true)
        ).await?;
        return Ok(());
    }

    let data = verify_user(&*code).await?;

    if !data.success{
        let message = data.message
            .unwrap_or_else(|| "Failed to verify.".to_string());
        ctx.send(
            CreateReply::default()
                .content(message)
                .ephemeral(true)
                .reply(true)
        ).await?;
        return Ok(());
    }
    
    let new_user = ActiveModel {
        discord_id: Set(ctx.author().id.to_string()),
        polymart_id: Set(data.user.unwrap()),
        ..Default::default()
    };

    new_user.insert(connection).await?;

    ctx.send(
        CreateReply::default()
            .content("Your polymart account has been linked successfully.")
            .ephemeral(true)
            .reply(true)
    ).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn unlink(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let connection = get_connection().unwrap();

    match get_user_id(connection, ctx.author().id.to_string()).await {
        Ok(None) => {
            ctx.send(
                CreateReply::default()
                    .content("You don't have a polymart account linked.")
                    .ephemeral(true)
                    .reply(true)
            ).await?;
            Ok(())
        }
        Ok(user) => {
            user.unwrap().delete(connection).await?;
            ctx.send(
                CreateReply::default()
                    .content("You polymart account is unlinked.")
                    .ephemeral(true)
                    .reply(true)
            ).await?;
            
            Ok(())
        }
        Err(e) => {
            ctx.say(e.to_string()).await?;
            Ok(())
        }
    }
}

#[poise::command(slash_command, prefix_command)]
pub async fn info(
    ctx: Context<'_>,
    resource: String,
) -> Result<(), Error> {
    let connection = get_connection().unwrap();
    
    let user = get_user_id(connection, ctx.author().id.to_string()).await?;
    
    if user.is_none(){
        ctx.send(
            CreateReply::default()
                .content("You don't have a polymart account linked.")
                .ephemeral(true)
                .reply(true)
        ).await?;
        return Ok(());
    }
    
    let user = user.unwrap();
    
    println!("{}", &user.id.to_string());
    
    let data = get_resource_data(&*resource, &*user.id.to_string()).await?;
    
    ctx.send(
        CreateReply::default()
            .content(data.purchase_valid.unwrap().to_string())
            .ephemeral(true)
            .reply(true)
    ).await?;
    
    Ok(())

}

pub async fn get_user_id(connection: &DatabaseConnection, discord_snowflake: String) -> Result<Option<Model>, Error> {
    let result = UserEntity::find()
        .filter(<user::Entity as EntityTrait>::Column::DiscordId.contains(discord_snowflake))
        .one(connection)
        .await;

    match result {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok((None)),
        Err(e) => Err(e.into()),
    }
}