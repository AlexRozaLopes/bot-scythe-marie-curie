use crate::prelude::*;

/// 🕰️| Descubra quando sua conta foi criada!
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: Option<User>,
) -> Result<()> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{} foi criado em {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
