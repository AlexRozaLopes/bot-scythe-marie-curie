use crate::prelude::*;

pub async fn add_role_a_new_user(
    ctx: &SerenityContext,
    _framework: FrameworkContext<'_, Data, Error>,
    new_member: &Member,
) -> Result<()> {
    let role_inicial = {
        let roles = &ctx.cache.guild(new_member.guild_id).unwrap().roles.clone();
        roles
            .iter()
            .find(|(_, m)| m.name.contains("Membro"))
            .map(|(id, _)| *id)
    };

    let mut role = Role::default();
    role.name = "Membro".to_string();
    role.colour = Colour::from_rgb(87, 242, 135);
    if !role_inicial.is_some() {
        role = ctx
            .http
            .create_role(new_member.guild_id, &role, None)
            .await?;
    }

    let _ = save_member(new_member).await;

    ctx.http
        .add_member_role(
            new_member.guild_id,
            new_member.user.id,
            role_inicial.unwrap_or_else(|| role.id),
            None,
        )
        .await?;
    println!("role add com sucesso!!!");
    Ok(())
}

async fn save_member(member: &Member) -> Result<()> {
    let mut redis = redis_con::get_connection().await;
    let m = MemberModel::new(member.clone());
    let member_models_guild_json: String = redis.get(member.guild_id.to_string()).await?;
    let mut member_models_guild: HashMap<UserId, MemberModel> =
        serde_json::from_str(&*member_models_guild_json)?;
    member_models_guild.insert(member.user.id, m);
    let smg = serde_json::to_string(&member_models_guild)?;
    let _: () = redis.set(member.guild_id.to_string(), smg).await.unwrap();
    println!("membro add a lista do redis com sucesso!");
    Ok(())
}
