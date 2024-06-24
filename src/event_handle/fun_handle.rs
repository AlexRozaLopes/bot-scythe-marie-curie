use poise::serenity_prelude as serenity;
use serenity::all::Message;

use crate::{Data, Error};

pub async fn dont_say_this_name(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
    data: &Data,
) -> Result<(), Error> {
    let ban_words = { data.ban_words.lock().unwrap().get(&new_message.guild_id.unwrap()).unwrap_or(&vec!["voldemort".to_string()]).clone() };
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
        }
    }

    Ok(())
}
