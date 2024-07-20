use crate::prelude::*;

pub async fn death_handler(
    ctx: &SerenityContext,
    _framework: FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<()> {
    if new_message.guild_id.is_none() {
        Ok(())
    } else {
        death(
            ctx,
            new_message.guild_id.unwrap(),
            new_message.clone().author.id,
        )
        .await?;

        Ok(())
    }
}

async fn death(ctx: &SerenityContext, guild_id: GuildId, author_id: UserId) -> Result<()> {
    let roles_guild = { ctx.http.get_guild_roles(guild_id).await? };

    let member_models_quit: HashMap<UserId, MemberModel> = {
        let mut redis = redis_con::get_connection().await;
        let option_m: Option<String> = redis.get(&guild_id.to_string()).await?;

        if option_m.is_none() {
            let member_models_guild = {
                let mut member_models_guild = HashMap::new();
                let hash_map = &ctx.cache.guild(&guild_id).unwrap().members;
                hash_map.iter().for_each(|(id, m)| {
                    member_models_guild.insert(*id, MemberModel::new(m.clone()));
                });
                member_models_guild
            };
            let _: () = redis
                .set(
                    guild_id.to_string(),
                    serde_json::to_string(&member_models_guild)?,
                )
                .await?;
        }

        let redis_hash: String = redis.get(&guild_id.to_string()).await?;
        let mut current_member_models_guild: HashMap<UserId, MemberModel> =
            serde_json::from_str(&*redis_hash)?;

        let data_marie = current_member_models_guild
            .iter()
            .find(|(_, m)| m.member().user.name == "Marie Curie")
            .map(|(_, m)| m.member().joined_at.unwrap())
            .unwrap();

        current_member_models_guild
            .entry(author_id)
            .and_modify(|m| {
                m.set_living(true);
                m.set_last_time_active(Option::from(Utc::now()))
            });

        let days_redis = get_days_redis(guild_id.clone().to_string()).await;

        current_member_models_guild.iter_mut().for_each(|(_, m)| {
            if !m.last_time_active().is_none() {
                if months_diff(
                    Utc::now(),
                    Timestamp::from(
                        *m.last_time_active()
                            .unwrap_or(&*m.member().joined_at.unwrap()),
                    ),
                    *m.last_time_active()
                        .unwrap_or(&*m.member().joined_at.unwrap()),
                    days_redis,
                ) {
                    m.set_living(false);
                }
            }
        });

        let json_membros = serde_json::to_string(&current_member_models_guild)?;
        let _: () = redis.set(guild_id.to_string(), json_membros).await?;

        let member_models_offline: HashMap<UserId, MemberModel> = current_member_models_guild
            .iter()
            .filter(|(_, m)| !m.is_alive())
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect();

        let data_hoje = Utc::now();

        member_models_offline
            .iter()
            .filter(|(_, m)| {
                months_diff(
                    data_hoje,
                    Timestamp::from(
                        *m.last_time_active()
                            .unwrap_or(&*m.member().joined_at.unwrap()),
                    ),
                    *data_marie,
                    days_redis,
                )
            })
            .map(|(id, m)| (id.clone(), m.clone()))
            .collect()
    };

    for (id, m) in member_models_quit {
        if !m.member().user.bot && !is_immune(m.clone(), roles_guild.clone()) {
            let reason = format!(
                "{} {}.",
                "Seu tempo aqui terminou; chegou a hora de partir",
                m.member().clone().user.name
            );
            match m.member().kick_with_reason(ctx, &*reason).await {
                Ok(()) => {
                    println!(
                        "membro {} excluido com sucesso",
                        m.member().clone().user.name
                    );
                    let mut redis = redis_con::get_connection().await;
                    let guild_string = m.member().guild_id.to_string();
                    let member_models_string: String =
                        redis.get(guild_string.clone()).await.unwrap();
                    let mut member_models: HashMap<UserId, MemberModel> =
                        serde_json::from_str(&*member_models_string).unwrap();
                    member_models.remove(&id);
                    let string_membro = serde_json::to_string(&member_models).unwrap();
                    let _: () = redis.set(guild_string, string_membro).await.unwrap();

                    let embed = CreateEmbed::new()
                        .description(reason)
                        .color(Colour::DARK_BLUE);
                    let message = CreateMessage::new().add_embed(embed);

                    m.member()
                        .user
                        .direct_message(ctx, message)
                        .await
                        .expect("TODO: panic message");
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn is_immune(member_model: MemberModel, roles_guild: Vec<Role>) -> bool {
    let id = roles_guild.iter().find(|r| r.name == "Imunidade");
    match id {
        Some(x) => member_model.member().roles.contains(&x.id),
        None => false,
    }
}

fn months_diff(
    atual: DateTime<Utc>,
    antigo: Timestamp,
    data_bot: DateTime<Utc>,
    days_redis: i32,
) -> bool {
    let data_comp = antigo.max(Timestamp::from(data_bot));

    let days = (((atual.year() * 12) + atual.month() as i32) * 30) + atual.day() as i32
        - ((((data_comp.year() * 12) + data_comp.month() as i32) * 30) + data_comp.day() as i32);

    days > days_redis
}

async fn get_days_redis(mut guild_id: String) -> i32 {
    let mut redis = redis_con::get_connection().await;
    guild_id.push_str("life_time");
    let days_redis: i32 = redis.get(guild_id).await.unwrap_or(30);
    days_redis
}

pub async fn death_handle_voice(
    ctx: &SerenityContext,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new: &VoiceState,
) -> Result<()> {
    death(ctx, new.guild_id.unwrap(), new.user_id).await?;

    Ok(())
}
