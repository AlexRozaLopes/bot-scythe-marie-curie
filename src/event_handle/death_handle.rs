use std::collections::HashMap;

use chrono::{Datelike, DateTime, Utc};
use poise::serenity_prelude as serenity;
use redis::AsyncCommands;
use serenity::all::{GuildId, Message, Role, Timestamp, UserId, VoiceState};

use crate::{Data, Error};
use crate::model::membro::Membro;
use crate::redis_connection::redis_con::get_redis_connection;

pub async fn death_handler(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<(), Error> {
    if new_message.guild_id.is_none() { Ok(()) } else {
        death(ctx, new_message.guild_id.unwrap(), new_message.clone().author.id).await?;

        Ok(())
    }
}

async fn death(ctx: &serenity::Context, guild_id: GuildId, author_id: UserId) -> Result<(), Error> {
    let roles_guild = { ctx.http.get_guild_roles(guild_id).await? };

    let membros_quit: HashMap<UserId, Membro> = {
        let mut membros = get_redis_connection().await;
        let option_m: Option<String> = membros.get(&guild_id.to_string()).await?;

        if option_m.is_none() {
            let membros_guild = {
                let mut membros_guild = HashMap::new();
                let hash_map = &ctx.cache.guild(&guild_id).unwrap().members;
                hash_map.iter().for_each(|(id, m)| {
                    membros_guild.insert(*id, Membro::new(m.clone()));
                });
                membros_guild
            };
            let _: () = membros.set(guild_id.to_string(), serde_json::to_string(&membros_guild)?).await?;
        }

        let redis_hash: String = membros.get(&guild_id.to_string()).await?;
        let mut membros_guild_atual: HashMap<UserId, Membro> = serde_json::from_str(&*redis_hash)?;

        let data_marie = membros_guild_atual.iter().find(|(_, m)| m.membro().user.name == "Marie Curie")
            .map(|(_, m)| m.membro().joined_at.unwrap())
            .unwrap();

        membros_guild_atual.entry(author_id).and_modify(|m| {
            m.set_ativo(true);
            m.set_ativo_em(Option::from(Utc::now()))
        });

        let days_redis = get_days_redis(guild_id.clone().to_string()).await;


        membros_guild_atual.iter_mut().for_each(|(_, m)| {
            if !m.ativo_em().is_none() {
                if months_diff(Utc::now(), Timestamp::from(m.ativo_em().unwrap_or(*m.membro().joined_at.unwrap())), m.ativo_em().unwrap_or(*m.membro().joined_at.unwrap()), days_redis) {
                    m.set_ativo(false);
                }
            }
        });

        let json_membros = serde_json::to_string(&membros_guild_atual)?;
        dbg!(&json_membros);
        let _: () = membros.set(guild_id.to_string(), json_membros).await?;

        let membros_offline: HashMap<UserId, Membro> = membros_guild_atual.iter().filter(|(_, m)| !m.ativo())
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect();

        let data_hoje = Utc::now();


        membros_offline.iter().filter(|(_, m)| months_diff(data_hoje, Timestamp::from(m.ativo_em().unwrap_or(*m.membro().joined_at.unwrap())), *data_marie, days_redis))
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect()
    };

    for (_, m) in membros_quit {
        if !m.membro().user.bot && !is_imune(m.clone(), roles_guild.clone()) {
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

fn months_diff(atual: DateTime<Utc>, antigo: Timestamp, data_bot: DateTime<Utc>, days_redis: i32) -> bool {
    let data_comp = antigo.max(Timestamp::from(data_bot));

    let days = (((atual.year() * 12) + atual.month() as i32) * 30) + atual.day() as i32 - ((((data_comp.year() * 12) + data_comp.month() as i32) * 30) + data_comp.day() as i32);

    days > days_redis
}

async fn get_days_redis(mut guild_id: String) -> i32 {
    let mut redis = get_redis_connection().await;
    guild_id.push_str("life_time");
    let days_redis: i32 = redis.get(guild_id).await.unwrap_or(30);
    days_redis
}

pub async fn death_handle_voice(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new: &VoiceState,
) -> Result<(), Error> {
    death(ctx, new.guild_id.unwrap(), new.user_id).await?;

    Ok(())
}