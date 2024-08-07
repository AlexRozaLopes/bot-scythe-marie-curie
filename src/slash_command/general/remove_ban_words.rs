use crate::prelude::*;

/// ✏️| Tire o banimento de uma palavra!
#[poise::command(slash_command, prefix_command)]
pub async fn remove_ban_word(
    ctx: Context<'_>,
    #[description = "Escreva uma palavra"] palavra: String,
) -> Result<()> {
    let m = ctx.author_member().await.unwrap().clone();
    if m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        let guild_id = ctx.guild_id().unwrap();
        let mut guild_id_string = guild_id.clone().to_string();

        let mut redis = redis_con::get_connection().await;
        guild_id_string.push_str("ban_word");
        let json_list: RedisResult<String> = redis.get(guild_id_string.clone()).await;
        match json_list {
            Ok(x) => {
                let mut list_ban: Vec<String> = serde_json::from_str(&x)?;
                if list_ban.contains(&palavra) {
                    list_ban.retain(|p| p.clone() != palavra.clone());
                };
                let _: () = redis
                    .set(guild_id_string, serde_json::to_string(&list_ban)?)
                    .await
                    .unwrap();
            }
            Err(_) => {
                let vec_v = serde_json::to_string(&vec!["voldemort".to_string(), palavra])?;
                let _: () = redis.set(guild_id_string, vec_v).await.unwrap();
            }
        }

        ctx.say("banimento tirado com sucesso!").await?;
    } else {
        ctx.say("vc nao tem permissao para usar este comando!")
            .await?;
    }

    Ok(())
}

/// 📜| Liste as palavras banidas!
#[poise::command(slash_command, prefix_command)]
pub async fn list_ban_word(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let ban_word_redis = redis_con::get_ban_word(guild_id).await;

    let mut lista = "### LISTA DE PALAVRAS SILENCIADAS".to_string();

    ban_word_redis.iter().for_each(|m| {
        lista.push_str("\n");
        let msg = format!("- {}", m);
        lista.push_str(&*msg)
    });

    if ban_word_redis.is_empty() {
        ctx.say("Nenhuma palavra foi silenciado!").await.unwrap();
    } else {
        ctx.author()
            .direct_message(ctx, CreateMessage::new().content(lista))
            .await
            .unwrap();
        ctx.say("Mensagem enviada para sua DM.").await.unwrap();
    }

    Ok(())
}
