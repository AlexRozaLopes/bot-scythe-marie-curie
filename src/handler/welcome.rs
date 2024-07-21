use crate::prelude::*;

pub async fn welcome_user(ctx: &SerenityContext,
                          _framework: FrameworkContext<'_, Data, Error>, new_member: &Member) -> Result<()>{
    println!("membro novo!");
    let guild = ctx.http.get_guild(new_member.guild_id).await.unwrap();
    let titulo = format!("Seja Bem vindo ao Servidor: {}", guild.name);
    let embed = CreateEmbed::new().title(titulo).description("Aproveite as pequenas coisas, pois um dia você pode olhar para trás e perceber que eram as grandes coisas.");
    let info = CreateEmbed::new().title("Aviso").description("Lembra-se que sou a **ceifadora** desse **SERVIDOR** caso vc nao siga as **REGRAS** sera **PENALIZADO**!🪦");
    new_member.user.direct_message(ctx, CreateMessage::new().embeds(vec![embed, info])).await.unwrap();
    Ok(())
}