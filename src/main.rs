
mod age;
mod coinflip;
mod event_handler;

use crate::age::age;
use crate::coinflip::coinflip;
use poise::serenity_prelude::ClientBuilder;
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};
use std::env::var;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    println!("Starting Bot...");

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![age(), coinflip()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::handler(ctx, event, framework, data))
            },// Register the commands
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                  
                })
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    println!("Bot started!");
    
    client.unwrap().start()
        .await.unwrap();
}



