use poise::serenity_prelude as serenity;
use serenity::all::{CreateEmbed, Member};
use serenity::builder::CreateMessage;

use crate::{Data, Error};

pub async fn welcome(ctx: &serenity::Context,
                     _framework: poise::FrameworkContext<'_, Data, Error>,
                     new_member: &Member ) -> Result<(), Error> {
    println!("membro novo!");
    let guild = ctx.http.get_guild(new_member.guild_id).await.unwrap();
    let titulo = format!("Seja Bem vindo ao Servidor: {}", guild.name);
    let embed = CreateEmbed::new().title(titulo).description("Aproveite as pequenas coisas, pois um dia você pode olhar para trás e perceber que eram as grandes coisas.");
    let info = CreateEmbed::new().title("Aviso").description("Lembra-se que sou a **ceifadora** desse **SERVIDOR** caso vc nao siga as **REGRAS** sera **PENALIZADO**!🪦");
    new_member.user.direct_message(ctx, CreateMessage::new().embeds(vec![embed, info])).await.unwrap();
    Ok(())
}