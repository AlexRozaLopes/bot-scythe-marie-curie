use std::collections::HashMap;

use chrono::{Datelike, DateTime, Utc};
use poise::serenity_prelude as serenity;
use serenity::all::{GuildId, Message, Role, Timestamp, UserId, VoiceState};

use crate::{Data, Error};
use crate::model::membro::Membro;

pub async fn death_handler(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    new_message: &Message,
) -> Result<(), Error> {
    if new_message.guild_id.is_none() { Ok(()) } else {
        death(ctx, new_message.guild_id.unwrap(), new_message.clone().author.id, data).await?;

        Ok(())
    }
}

async fn death(ctx: &serenity::Context, guild_id: GuildId, author_id: UserId, data: &Data) -> Result<(), Error> {
    let roles_guild = { ctx.http.get_guild_roles(guild_id).await? };

    let membros_quit: HashMap<UserId, Membro> = {
        let mut membros = data.membros.lock().unwrap();
        let option_m = membros.get(&guild_id);
        if option_m.is_none() {
            let mut membros_guild = HashMap::new();
            let hash_map = &ctx.cache.guild(&guild_id).unwrap().members;
            hash_map.iter().for_each(|(id, m)| {
                membros_guild.insert(*id, Membro::new(m.clone()));
            });
            membros.insert(guild_id, membros_guild);
        }
        let mut membros_guild_atual = membros.get(&guild_id).unwrap().clone();
        membros_guild_atual.entry(author_id).and_modify(|m| m.set_ativo(true));

        let membros_offline: HashMap<UserId, Membro> = membros_guild_atual.iter().filter(|(_, m)| !m.ativo())
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect();

        let data_hoje = Utc::now();

        membros_offline.iter().filter(|(_, m)| months_diff(data_hoje, m.membro().joined_at.unwrap(), data.data_criacao))
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect()
    };

    for (_, m) in membros_quit {
        if !m.membro().user.bot || !is_imune(m.clone(), roles_guild.clone()) {
            let reason = format!("{} {}.", "Seu tempo aqui terminou; chegou a hora de partir", m.membro().clone().user.name);
            match m.membro().kick_with_reason(ctx, &*reason).await {
                Ok(()) => println!("membro {} excluido com sucesso", m.membro().clone().user.name),
                _ => {}
            }
        }
    }

    Ok(())
}

fn is_imune(membro: Membro, roles_guild: Vec<Role>) -> bool {
    let id = roles_guild.iter().find(|r| r.name == "Imunidade");
    match id {
        Some(x) => membro.membro().roles.contains(&x.id),
        None => false
    }
}

fn months_diff(atual: DateTime<Utc>, antigo: Timestamp, data_bot: DateTime<Utc>) -> bool {
    let data_comp = atual.max(data_bot);
    let months_diff = data_comp.year() * 12 + data_comp.month() as i32 - (antigo.year() * 12 + antigo.month() as i32);
    months_diff > 1
}

pub async fn death_handle_voice(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    new: &VoiceState,
) -> Result<(), Error> {
    death(ctx, new.guild_id.unwrap(), new.user_id, data).await?;

    Ok(())
}