use std::collections::HashMap;

use poise::serenity_prelude as serenity;
use serenity::all::{Permissions, UserId};
use serenity::builder::{CreateEmbed, CreateMessage};

use crate::{Context, Error};
use crate::event_handle::silence_handle::get_silence_membros;
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

        let msg = format!("**{}** silenciado com sucesso!", user.name);
        let info = format!("Procure o **{}** para mais informacoes. (GUILD: **{}**)", ctx.author(), ctx.guild().unwrap().name);
        membro.membro().user.direct_message(ctx, CreateMessage::new().embed(CreateEmbed::new().title("VC FOI SILENCIADO")
            .description(info))).await.unwrap();
        ctx.say(msg).await.unwrap();

        set_membros_redis(guild_id, membros).await.unwrap();
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


/// lista todos os membros silenciados atualmente!
#[poise::command(slash_command, prefix_command)]
pub async fn list_silence_people(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.author_member().await.unwrap().guild_id;
    let membros: HashMap<UserId, Membro> = get_membros_redis(guild_id).await;

    let silence_membros = get_silence_membros(membros);

    let mut lista = "### LISTA DE MEMBROS SILENCIADOS".to_string();

    silence_membros.iter().for_each(|(_, m)| {
        lista.push_str("\n");
        let msg = format!("- {}", m.membro().user.name);
        lista.push_str(&*msg)
    });

    if silence_membros.is_empty() {
        ctx.say("Nenhum membro foi silenciado!").await.unwrap();
    } else {
        ctx.say(lista).await.unwrap();
    }


    Ok(())
}
