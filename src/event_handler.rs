use crate::{Data, Error};
use poise::serenity_prelude::ReactionType;


use poise::serenity_prelude as serenity;
pub(crate) async fn handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            handle_message(ctx, &new_message).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<(), Error> {
    if message.content == "ping" {
        message.react(
            &ctx.http,
            ReactionType::Unicode("🏓".to_string())
        ).await?;
        message
            .reply(
                ctx,
                "pong",
            ).await?;
    }
    Ok(())
}