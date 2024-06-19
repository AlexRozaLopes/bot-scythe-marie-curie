use poise::serenity_prelude as serenity;
use serenity::all::{Colour, Member, Role};

use crate::{Data, Error};

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

    ctx.http.add_member_role(new_member.guild_id, new_member.user.id, role_inicial.unwrap_or_else(||role.id), None).await?;
    println!("role add com sucesso!!!");
    Ok(())
}