use std::collections::HashMap;

use chrono::{Datelike, DateTime, Utc};
use poise::serenity_prelude as serenity;
use serenity::all::{Message, Timestamp, UserId};

use crate::{Data, Error};
use crate::model::membro::Membro;

pub async fn death_handler(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    new_message: &Message,
) -> Result<(), Error> {
    let membros_quit: HashMap<UserId, Membro> = {
        let mut membros = data.membros.lock().unwrap();
        if membros.is_empty() {
            let hash_map = &ctx.cache.guild(&new_message.guild_id.unwrap()).unwrap().members;
            hash_map.iter().for_each(|(id, m)| {
                membros.insert(*id, Membro::new(m.clone()));
            })
        }
        membros.entry(new_message.clone().author.id).and_modify(|m| m.set_ativo(true));

        let membros_offline: HashMap<UserId, Membro> = membros.iter().filter(|(_, m)| !m.ativo())
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect();

        let data_hoje = Utc::now();

        membros_offline.iter().filter(|(_, m)| months_diff(data_hoje,m.membro().joined_at.unwrap()))
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect()
    };

    for (_, m) in membros_quit {
        if !m.membro().user.bot {
            let reason = format!("{} {}.", "Seu tempo aqui terminou; chegou a hora de partir", m.membro().clone().user.name);
            match m.membro().kick_with_reason(ctx, &*reason).await {
                Ok(()) => println!("membro {} excluido com sucesso", m.membro().clone().user.name),
                _ => {}
            }
        }
    }

    Ok(())
}

fn months_diff(atual:DateTime<Utc>, antigo: Timestamp) -> bool {
    let months_diff = atual.year() * 12 + atual.month() as i32 - (antigo.year() * 12 + antigo.month() as i32);
    months_diff > 3
}