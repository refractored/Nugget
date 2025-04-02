use std::alloc::System;
use std::process::exit;
use poise::CreateReply;
use crate::{get_config, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn shutdown(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !get_config().config.admins.iter().any(|id| id == &ctx.author().id.to_string()) {
        ctx.say("You are not an admin!").await?;
        return Ok(());
    }
    
    ctx.send(
        CreateReply::default()
            .content("Shutting Down... :c")
            .ephemeral(true)
    ).await?;
    
    exit(0);
}