use std::collections::HashMap;

use chrono::{Datelike, DateTime, Utc};
use poise::serenity_prelude as serenity;
use serenity::all::{Message, Role, Timestamp, UserId};

use crate::{Data, Error};
use crate::model::membro::Membro;

pub async fn death_handler(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    new_message: &Message,
) -> Result<(), Error> {
    let roles_guild = ctx.http.get_guild_roles(new_message.guild_id.unwrap()).await?;

    let membros_quit: HashMap<UserId, Membro> = {
        let mut membros = data.membros.lock().unwrap();
        let option_m = membros.get(&new_message.guild_id.unwrap());
        if option_m.is_none() {
            let mut membros_guild = HashMap::new();
            let hash_map = &ctx.cache.guild(&new_message.guild_id.unwrap()).unwrap().members;
            hash_map.iter().for_each(|(id, m)| {
                membros_guild.insert(*id, Membro::new(m.clone()));
            });
            membros.insert(new_message.guild_id.unwrap(),membros_guild);
        }
        let mut membros_guild_atual = membros.get(&new_message.guild_id.unwrap()).unwrap().clone();
        membros_guild_atual.entry(new_message.clone().author.id).and_modify(|m| m.set_ativo(true));

        let membros_offline: HashMap<UserId, Membro> = membros_guild_atual.iter().filter(|(_, m)| !m.ativo())
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect();

        let data_hoje = Utc::now();

        membros_offline.iter().filter(|(_, m)| months_diff(data_hoje,m.membro().joined_at.unwrap(),data.data_criacao))
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

fn months_diff(atual:DateTime<Utc>, antigo: Timestamp,data_bot: DateTime<Utc>) -> bool {
    let data_comp = atual.max(data_bot);
    let months_diff = data_comp.year() * 12 + data_comp.month() as i32 - (antigo.year() * 12 + antigo.month() as i32);
    months_diff > 1
}