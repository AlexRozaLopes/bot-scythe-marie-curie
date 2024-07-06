use std::collections::HashMap;

use poise::{FrameworkContext, serenity_prelude as serenity};
use serenity::all::{Message, UserId, VoiceState};

use crate::{Data, Error};
use crate::model::membro::Membro;
use crate::redis_connection::redis_con::get_membros_redis;

pub async fn silence_handle_voice(
    ctx: &serenity::Context,
    _framework: FrameworkContext<'_, Data, Error>,
    new: &VoiceState,
) -> Result<(), Error> {
    let membros = get_membros_redis(new.clone().guild_id.unwrap()).await;
    let membros_silenciados: HashMap<UserId, Membro> = membros.iter().filter(|(_, m)| m.silence())
        .map(|(id,m)|(id.clone(),m.clone()))
        .collect();

    let user_id = new.member.clone().unwrap().user.id;
    let option_m = membros_silenciados.get(&user_id);

    match option_m {
        None => {}
        Some(_) => {
            new.member.clone().unwrap().disconnect_from_voice(ctx).await.expect("TODO: panic message");
        }
    }

    Ok(())
}

pub async fn silence_handle(
    ctx: &serenity::Context,
    _framework: FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<(), Error> {

    let membros = get_membros_redis(new_message.guild_id.unwrap()).await;
    let membros_silenciados: HashMap<UserId, Membro> = membros.iter().filter(|(_, m)| m.silence())
        .map(|(id,m)|(id.clone(),m.clone()))
        .collect();

    let user_id = new_message.author.id;
    let option_m = membros_silenciados.get(&user_id);

    match option_m {
        None => {}
        Some(_) => {
            new_message.delete(ctx).await.unwrap();
        }
    }

    Ok(())
}