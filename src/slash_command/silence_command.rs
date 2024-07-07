use std::collections::HashMap;

use poise::serenity_prelude as serenity;
use serenity::all::{Permissions, UserId};

use crate::{Context, Error};
use crate::model::membro::Membro;
use crate::redis_connection::redis_con::{get_membros_redis, set_membros_redis};

/// silencie alguem. 🔇🔇🔇
#[poise::command(slash_command, prefix_command)]
pub async fn silence_someone(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: serenity::User,
) -> Result<(), Error> {
    let m = ctx.author_member().await.unwrap().clone();
    if m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        let guild_id = ctx.author_member().await.unwrap().guild_id;
        let mut membros: HashMap<UserId, Membro> = get_membros_redis(guild_id).await;

        let membro = membros.get_mut(&user.id).unwrap();

        membro.set_silence(true);

        set_membros_redis(guild_id, membros).await.unwrap();

        let msg = format!("**{}** silenciado com sucesso!", user.name);
        ctx.say(msg).await.unwrap();
    } else {
        ctx.say("vc nao tem permissao para usar este comando!").await.expect("TODO: panic message");
    }

    Ok(())
}

/// remova o silence de alguem. 📢
#[poise::command(slash_command, prefix_command)]
pub async fn remove_silence_someone(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: serenity::User,
) -> Result<(), Error> {
    let m = ctx.author_member().await.unwrap().clone();
    if m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        let guild_id = ctx.author_member().await.unwrap().guild_id;
        let mut membros: HashMap<UserId, Membro> = get_membros_redis(guild_id).await;

        let membro = membros.get_mut(&user.id).unwrap();

        membro.set_silence(false);

        set_membros_redis(guild_id, membros).await.unwrap();

        let msg = format!("Silence do **{}** removido com sucesso!", user.name);
        ctx.say(msg).await.unwrap();
    } else {
        ctx.say("vc nao tem permissao para usar este comando!").await.expect("TODO: panic message");
    }

    Ok(())
}
