use poise::serenity_prelude as serenity;

use crate::{Context, Error};

/// 🕰️| Descubra quando sua conta foi criada!
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{} foi criado em {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}