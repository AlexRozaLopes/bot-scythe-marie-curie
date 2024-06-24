use poise::serenity_prelude as serenity;
use serenity::model::Permissions;

use crate::{Context, Error};

/// Defina palavras que sao proibidas para esse servidor!
#[poise::command(slash_command, prefix_command)]
pub async fn add_ban_word(
    ctx: Context<'_>,
    #[description = "Escreva uma palavra"] palavra: String,
) -> Result<(), Error> {
    let m = ctx.author_member().await.unwrap().clone();
    if m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        let guild_id = ctx.guild_id().unwrap();

        {
            let mut mutex_guard = ctx.data().ban_words.lock().unwrap();
            mutex_guard.entry(guild_id).or_insert(vec!["voldemort".to_string()]);
            mutex_guard.entry(guild_id).and_modify(|v| v.push(palavra));
        }

        ctx.say("palavra add com sucesso!").await?;
    } else {
        ctx.say("vc nao tem permissao para usar este comando").await?;
    }

    Ok(())
}
