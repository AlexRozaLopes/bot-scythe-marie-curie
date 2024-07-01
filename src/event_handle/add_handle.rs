use std::collections::HashMap;
use poise::serenity_prelude as serenity;
use redis::AsyncCommands;
use serenity::all::{Colour, Member, Role, UserId};

use crate::{Data, Error};
use crate::model::membro::Membro;
use crate::redis_connection::redis_con::get_redis_connection;

pub async fn add_role_a_new_user(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new_member: &Member,
) -> Result<(), Error> {
    let role_inicial = {
        let roles = &ctx.cache.guild(new_member.guild_id).unwrap().roles.clone();
        roles.iter()
            .find(|(_, m)| m.name.contains("Membro"))
            .map(|(id, _)| *id)
    };

    let mut role = Role::default();
    role.name = "Membro".to_string();
    role.colour = Colour::from_rgb(87, 242, 135) ;
    if !role_inicial.is_some() {
        role = ctx.http.create_role(new_member.guild_id, &role, None).await?;
    }

    let _ = save_membro(new_member).await;

    ctx.http.add_member_role(new_member.guild_id, new_member.user.id, role_inicial.unwrap_or_else(||role.id), None).await?;
    println!("role add com sucesso!!!");
    Ok(())
}

async fn save_membro(membro: &Member) -> Result<(), Error> {
    let mut redis = get_redis_connection().await;
    let m = Membro::new(membro.clone());
    let membros_guild_json :String = redis.get(membro.guild_id.to_string()).await?;
    let mut membros_guild: HashMap<UserId,Membro> = serde_json::from_str(&*membros_guild_json)?;
    membros_guild.insert(membro.user.id,m);
    let smg = serde_json::to_string(&membros_guild)?;
    let _ :() = redis.set(membro.guild_id.to_string(),smg).await.unwrap();
    println!("membro add a lista do redis com sucesso!");
    Ok(())
}
