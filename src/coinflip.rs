use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn coinflip(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Flipping a coin...").await?;
    let result = if rand::random() {
        "heads"
    } else {
        "tails"
    };

    let response = format!("You got {} :3",  result);

    ctx.say(response).await?;

    Ok(())
}