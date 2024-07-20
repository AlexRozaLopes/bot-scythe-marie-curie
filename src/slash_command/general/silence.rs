use crate::prelude::*;

/// 🔇| Silencie alguem.
#[poise::command(slash_command, prefix_command)]
pub async fn silence_someone(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: User,
) -> Result<()> {
    let m = ctx.author_member().await.unwrap().clone();

    if !m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        ctx.say("vc nao tem permissao para usar este comando!")
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    let guild_id = ctx.author_member().await.unwrap().guild_id;
    let mut member_models: HashMap<UserId, MemberModel> =
        redis_con::get_member_models(guild_id).await;

    let member_model = member_models.get_mut(&user.id).unwrap();

    member_model.set_silenced(true);

    let msg = format!("**{}** silenciado com sucesso!", user.name);
    let info = format!(
        "Procure o **{}** para mais informacoes. (GUILD: **{}**)",
        ctx.author(),
        ctx.guild().unwrap().name
    );
    member_model
        .member()
        .user
        .direct_message(
            ctx,
            CreateMessage::new().embed(
                CreateEmbed::new()
                    .title("VC FOI SILENCIADO")
                    .description(info),
            ),
        )
        .await
        .unwrap();
    ctx.say(msg).await.unwrap();

    redis_con::set_member_models(guild_id, member_models)
        .await
        .unwrap();

    Ok(())
}

/// 📢| Remova o silence de alguem.
#[poise::command(slash_command, prefix_command)]
pub async fn remove_silence_someone(
    ctx: Context<'_>,
    #[description = "Selecione um Usuario"] user: User,
) -> Result<()> {
    let m = ctx.author_member().await.unwrap().clone();
    if !m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        ctx.say("vc nao tem permissao para usar este comando!")
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    let guild_id = ctx.author_member().await.unwrap().guild_id;
    let mut member_models: HashMap<UserId, MemberModel> =
        redis_con::get_member_models(guild_id).await;

    let member_model = member_models.get_mut(&user.id).unwrap();

    member_model.set_silenced(false);

    redis_con::set_member_models(guild_id, member_models)
        .await
        .unwrap();

    let msg = format!("**{}** foi dessilenciado!", user.name);
    ctx.say(msg).await.unwrap();

    Ok(())
}

/// 📜| Lista todos os membros silenciados atualmente!
#[poise::command(slash_command, prefix_command)]
pub async fn list_silence_people(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.author_member().await.unwrap().guild_id;
    let member_models: HashMap<UserId, MemberModel> = redis_con::get_member_models(guild_id).await;

    let silenced_member_models = handler::silence::filter_silenced_member_models(member_models);

    let mut lista = "### LISTA DE MEMBROS SILENCIADOS".to_string();

    silenced_member_models.iter().for_each(|(_, m)| {
        lista.push_str("\n");
        let msg = format!("- {}", m.member().user.name);
        lista.push_str(&*msg)
    });

    if silenced_member_models.is_empty() {
        ctx.say("Nenhum membro está silenciado!").await.unwrap();
    } else {
        ctx.say(lista).await.unwrap();
    }

    Ok(())
}
