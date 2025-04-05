use crate::polymart::{generate_polymart_verify_url, verify_user};
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn generate(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let data = generate_polymart_verify_url().await?;

    if !data.success{
        ctx.say("Failed to get verify URL.").await?;
        return Ok(());
    }

    let response = format!("You can verify your polymart account with [this link]({}).\nOnce done you can run ``/verify``.", data.url.unwrap());

    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn verify(
    ctx: Context<'_>,
    code: String,
) -> Result<(), Error> {
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

    ctx.say(response).await?;

    Ok(())
}