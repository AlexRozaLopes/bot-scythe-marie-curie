use poise::serenity_prelude as serenity;
use redis::{AsyncCommands, RedisResult};
use serenity::all::{Colour, CreateEmbed, CreateMessage, Message};

use crate::{Data, Error};
use crate::redis_connection::redis_con::get_redis_connection;

pub async fn dont_say_this_name(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<(), Error> {
    let ban_words: Vec<String> = {
        let mut redis = get_redis_connection().await;
        let guild_id = new_message.guild_id;

        match guild_id {
            None => { return Ok(()) }
            Some(_) => {}
        }

        let mut guild_id_string = guild_id.unwrap().to_string();
        guild_id_string.push_str(&*"ban_word".to_string());
        let ban_w: RedisResult<String> = redis.get(guild_id_string.clone()).await;
        match ban_w {
            Ok(x) => serde_json::from_str(&*x).unwrap(),
            _ => vec!["voldemort".to_string()]
        }
    };
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
            let motivo_description = format!("Esse incidente ocorreu devido esta mensagem em particular: || **{}** ||", new_message.content);
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
