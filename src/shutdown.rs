use std::process::exit;
use poise::CreateReply;
use crate::{get_config, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn shutdown(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !get_config().config.admins.iter().any(|id| id == &ctx.author().id.to_string()) {
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