use chrono_tz::Tz;
use poise::serenity_prelude as serenity;
use redis::AsyncCommands;
use serenity::all::{Permissions, Timestamp};

use crate::{Context, Error};
use crate::redis_connection::redis_con::get_redis_connection;

/// Descubra quando sua conta foi criada!
#[poise::command(slash_command, prefix_command)]
pub async fn life(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let member = ctx.http().get_member(ctx.guild_id().unwrap(), u.id).await?;

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

/// Defina o tempo de vida dos membros. 🪦🪦🪦
#[poise::command(slash_command, prefix_command)]
pub async fn life_time(
    ctx: Context<'_>,
    #[description = "digite o numero de dias para a coleta! 🗡️"] days: i32,
) -> Result<(), Error> {
    let m = ctx.author_member().await.unwrap().clone();
    if m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {

        let mut guild_id = ctx.guild_id().unwrap().to_string();
        guild_id.push_str("life_time");
        let mut redis = get_redis_connection().await;

        let _:() = redis.set(guild_id,days).await.unwrap();
        ctx.say("tempo definido com sucesso!").await.expect("TODO: panic message");
    } else {
        ctx.say("vc nao tem permissao para usar este comando!").await.expect("TODO: panic message");
    }

    Ok(())
}