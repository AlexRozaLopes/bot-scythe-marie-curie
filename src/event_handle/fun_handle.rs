use poise::serenity_prelude as serenity;
use serenity::all::{Colour, CreateEmbed, CreateMessage, GuildId, Message};

use crate::{Data, Error};

pub async fn dont_say_this_name(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
    data: &Data,
) -> Result<(), Error> {
    let ban_words = { data.ban_words.lock().unwrap().get(&new_message.guild_id.unwrap_or(GuildId::new(1))).unwrap_or(&vec!["voldemort".to_string()]).clone() };
    for bn in ban_words {
        if new_message.content.to_lowercase().contains(&bn)
            && new_message.author.id != ctx.cache.current_user().id
        {
            new_message
                .reply(
                    ctx,
                    "Nao falamos esse nome aqui!!!".to_string(),
                )
                .await?;
            new_message.delete(ctx).await?;

            let intro_description = "Recentemente, recebemos relatos e observamos comportamentos que não estão de acordo com os padrões de conduta esperados neste servidor.".to_string();
            let motivo_description = format!("Esse incidente ocorreu devido esta mensagem em particular: ====> {}", new_message.content);
            let intro = CreateEmbed::new().title("Conduta Inapropriada").description(intro_description).color(Colour::RED);
            let motivo = CreateEmbed::new().title("Principal Motivo").description(motivo_description).color(Colour::RED);
            let message_into = CreateMessage::new().add_embed(intro);
            let message_motivo = CreateMessage::new().add_embed(motivo);

            new_message.author.direct_message(ctx, message_into).await?;
            new_message.author.direct_message(ctx, message_motivo).await?;
        }
    }

    Ok(())
}
