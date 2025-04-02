
mod age;
mod coinflip;
mod event_handler;
mod shutdown;

use crate::age::age;
use crate::coinflip::coinflip;
use poise::serenity_prelude::ClientBuilder;
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};
use serde_derive::Deserialize;
use std::fs;
use std::sync::OnceLock;
use crate::shutdown::shutdown;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[derive(Deserialize)]
struct ConfigData {
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    token: String,
    admins: Vec<String>,
    admin_roles: Vec<String>,
}

static CONFIG: OnceLock<ConfigData> = OnceLock::new();

fn get_config() -> &'static ConfigData {
    CONFIG.get_or_init(|| {
        read_config().expect("Unable to read config")
    })
}


fn read_config() -> Result<ConfigData, Error> {
    // Variable that holds the filename as a `&str`.
    let filename = "config.toml";

    let contents = match fs::read_to_string(&filename) {
        Ok(contents) => contents,
        Err(_) => {
            // Write `msg` to `stderr`.
            println!("Generating default config file...");

            let default_content = include_str!("default_config.toml");
            fs::write(&filename, default_content)?;

            default_content.to_string()
        }
    };
    
    let data: ConfigData = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            return Err(Error::from("Could not load data"));
        }
    };
    
    Ok(data)
}

#[tokio::main]
async fn main() {
    println!("Reading config...");

    let config_data = get_config();

    if config_data.config.token == "" || config_data.config.token == "DISCORD_TOKEN" {
        println!("Token not found in config.toml");
        return;
    }

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    println!("Starting Bot...");

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![age(), coinflip(), shutdown()],
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

    let client = ClientBuilder::new(&config_data.config.token, intents)
        .framework(framework)
        .await;

    println!("Bot started!");

    client.unwrap().start()
        .await.unwrap();
}



