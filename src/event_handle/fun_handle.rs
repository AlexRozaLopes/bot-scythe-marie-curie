use poise::serenity_prelude as serenity;
use serenity::all::Message;

use crate::{Data, Error};

pub async fn dont_say_this_name(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<(), Error> {
    if new_message.content.to_lowercase().contains("voldemort")
        && new_message.author.id != ctx.cache.current_user().id
    {
        new_message
            .reply(
                ctx,
                format!("Nao falamos esse nome aqui!!!"),
            )
            .await?;
        new_message.delete(ctx).await?;
    }
    Ok(())
}
