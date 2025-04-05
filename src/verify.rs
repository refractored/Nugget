use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, Set};
use sea_orm::sea_query::extension::mysql::IndexHintType::Use;
use crate::polymart::{generate_polymart_verify_url, verify_user};
use crate::{get_config, get_connection, user, Context, Error, CONNECTION};
use crate::user::{ActiveModel, Entity as UserEntity, Model};



#[poise::command(slash_command, prefix_command)]
pub async fn generate(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let connection = get_connection().await.unwrap();

    let result = UserEntity::find()
        .filter(<user::Entity as EntityTrait>::Column::DiscordId.contains(ctx.author().id.to_string()))
        .one(connection)
        .await;

    match result {
        Ok(user) => {
            if user.is_some() {
                ctx.say("You already have a polymart account linked.").await?;
                return Ok(());
            }
        }
        _ => {}
    }


    let data = generate_polymart_verify_url().await?;

    if !data.success{
        ctx.say("Failed to get verify URL.").await?;
        return Ok(());
    }

    let response = format!("You can verify your polymart account with [this link]({}).\nOnce done you can run ``/link``.", data.url.unwrap());

    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn link(
    ctx: Context<'_>,
    code: String,
) -> Result<(), Error> {

    let connection = get_connection().await.unwrap();

    let result = UserEntity::find()
        .filter(<user::Entity as EntityTrait>::Column::DiscordId.contains(ctx.author().id.to_string()))
        .one(connection)
        .await;

    match result {
        Ok(user) => {
            if user.is_some() {
                ctx.say("You already have a polymart account linked.").await?;
                return Ok(());
            }
        }
        _ => {}
    }

    let regex = regex::Regex::new("[A-Za-z0-9]{3}-[A-Za-z0-9]{3}-[A-Za-z0-9]+").unwrap();

    if !regex.is_match(&code){
        ctx.say("Invalid code.").await?;
        return Ok(());
    }

    let data = verify_user(&*code).await?;

    if !data.success{
        let message = data.message
            .unwrap_or_else(|| "Failed to verify.".to_string());
        ctx.say(message).await?;
        return Ok(());
    }

    let response = "it worked";

    let connection = get_connection().await.unwrap();

    let new_user = ActiveModel {
        discord_id: Set(ctx.author().id.to_string()),
        polymart_id: Set(data.user.unwrap()),
        ..Default::default()
    };

    new_user.insert(connection).await?;

    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn unlink(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let connection = get_connection().await.unwrap();

    let result = UserEntity::find()
        .filter(<user::Entity as EntityTrait>::Column::DiscordId.contains(ctx.author().id.to_string()))
        .one(connection)
        .await;

    if result.is_err() {
        ctx.say(result.err().unwrap().to_string()).await?;
        return Ok(());
    }

    let user = result.unwrap();

    if user.is_none() {
        ctx.say("You don't have a polymart account linked.").await?;
        return Ok(());
    }

    user.unwrap().delete(connection).await?;

    ctx.say("You have unlinked your polymart account.").await?;

    Ok(())
}