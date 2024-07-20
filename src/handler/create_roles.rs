use poise::{serenity_prelude as serenity, FrameworkContext};
use serenity::all::{Colour, Context, Ready, Role};

use crate::{Data, Error};

pub async fn create_role_imunidade(
    ctx: &Context,
    _framework: FrameworkContext<'_, Data, Error>,
    data_about_bot: &Ready,
) -> Result<(), Error> {
    let mut role = Role::default();
    role.name = "Imunidade".to_string();
    role.colour = Colour::RED;

    let guilds = &data_about_bot.guilds;
    for g in guilds {
        let roles = ctx.http.get_guild_roles(g.id).await?;
        let role_imu: Vec<Role> = roles
            .iter()
            .filter(|r| r.name == role.name)
            .map(|i| i.clone())
            .collect();
        if role_imu.is_empty() {
            ctx.http.create_role(g.id, &role, None).await?;
        }
    }

    Ok(())
}
