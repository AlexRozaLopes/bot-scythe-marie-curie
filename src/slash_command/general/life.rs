use crate::prelude::*;

/// 🕰️| Descubra quando sua conta foi criada!
#[poise::command(slash_command, prefix_command)]
pub async fn life(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: Option<User>,
) -> Result<()> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let member = ctx.http().get_member(ctx.guild_id().unwrap(), u.id).await?;

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

/// 🪦| Defina o tempo de vida dos membros.
#[poise::command(slash_command, prefix_command)]
pub async fn life_time(
    ctx: Context<'_>,
    #[description = "digite o numero de dias para a coleta! 🗡️"] days: i32,
) -> Result<()> {
    let m = ctx.author_member().await.unwrap().clone();
    if m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        let mut guild_id = ctx.guild_id().unwrap().to_string();
        guild_id.push_str("life_time");
        let mut redis = redis_con::get_connection().await;

        let _: () = redis.set(guild_id, days).await.unwrap();
        ctx.say("tempo definido com sucesso!")
            .await
            .expect("TODO: panic message");
    } else {
        ctx.say("vc nao tem permissao para usar este comando!")
            .await
            .expect("TODO: panic message");
    }

    Ok(())
}

/// ⚔️| Descubra quanto dias foram definidos para a coleita!
#[poise::command(slash_command, prefix_command)]
pub async fn get_life_time(ctx: Context<'_>) -> Result<()> {
    let mut redis = redis_con::get_connection().await;
    let mut guild_id = ctx.guild_id().unwrap().to_string();
    let s = "life_time";
    guild_id.push_str(s);
    let days: i32 = redis.get(guild_id).await.unwrap_or(30);

    let msg = format!("O **life_time** esta definida para: **{days}** dias!");
    ctx.say(msg).await.expect("TODO: panic message");
    Ok(())
}
