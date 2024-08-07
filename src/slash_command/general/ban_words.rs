use crate::prelude::*;

/// 🚫| Defina palavras que sao proibidas para esse servidor!
#[poise::command(slash_command, prefix_command)]
pub async fn add_ban_word(
    ctx: Context<'_>,
    #[description = "Escreva uma palavra"] palavra: String,
) -> Result<()> {
    let m = ctx.author_member().await.unwrap().clone();
    if m.permissions.unwrap().contains(Permissions::ADMINISTRATOR) {
        let guild_id = ctx.guild_id().unwrap();
        let mut guild_id_string = guild_id.clone().to_string();

        let mut redis = redis_con::get_connection().await;
        guild_id_string.push_str(&*"ban_word".to_string());
        let json_list: RedisResult<String> = redis.get(guild_id_string.clone()).await;
        match json_list {
            Ok(x) => {
                let mut list_ban: Vec<String> = serde_json::from_str(&*x)?;
                list_ban.push(palavra);
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

        ctx.say("palavra add com sucesso!").await?;
    } else {
        ctx.say("vc nao tem permissao para usar este comando")
            .await?;
    }

    Ok(())
}
