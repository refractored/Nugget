use std::process::exit;
use poise::CreateReply;
use crate::{get_config, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn shutdown(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !get_config().config.admins.iter().any(|id| id == &ctx.author().id.to_string())
        && !check_user_roles(&ctx).await {

        ctx.send(
            CreateReply::default()
                .content("Only admins can run this command.")
                .ephemeral(true)
                .reply(true)
        ).await?;
        return Ok(());
    }

    ctx.send(
        CreateReply::default()
            .content("Shutting Down... :c")
            .ephemeral(true)
            .reply(true)
    ).await?;
    
    exit(0);
}

async fn check_user_roles(ctx: &Context<'_>) -> bool {

    let guild = if let Some(guild) = ctx.guild_id() {
        guild
    } else {
        return false;
    };

    if get_config().config.guild_id != guild.to_string() {
        return false;
    }

    let member = match guild.member(ctx.http(), ctx.author().id).await {
        Ok(member) => member,
        Err(_) => return false,
    };

    let admin_roles = &get_config().config.admin_roles;
    member.roles.iter().any(|role| {
        admin_roles.iter().any(|admin_role|
            role.to_string() == *admin_role
        )
    })

}