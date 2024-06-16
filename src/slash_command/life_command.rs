use chrono_tz::Tz;
use poise::serenity_prelude as serenity;
use serenity::all::Timestamp;
use crate::{Context, Error};

/// Descubra quando sua conta foi criada!
#[poise::command(slash_command, prefix_command)]
pub async fn life(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: Option<serenity::User>,
) -> Result<(), Error> {

    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let member = ctx.http().get_member(ctx.guild_id().unwrap(),u.id).await?;

    dbg!(&member);

    let data_formatada = get_data_br(member.joined_at.unwrap());

    let response = format!("{} Esta conosco desde {}", u.name, data_formatada);

    ctx.say(response).await?;

    Ok(())
}

fn get_data_br(data: Timestamp) -> String {

    let tz: Tz = "America/Sao_Paulo".parse().expect("Invalid timezone");
    let date_time = data.with_timezone(&tz);
    let formatted = format!("{}", date_time.format("%d/%m/%Y %H:%M"));

    formatted
}