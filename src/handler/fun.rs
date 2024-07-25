use crate::prelude::*;

pub async fn dont_say_this_name(
    ctx: &SerenityContext,
    _framework: FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<()> {
    let ban_words: Vec<String> = {
        let mut redis = redis_con::get_connection().await;
        let guild_id = new_message.guild_id;

        match guild_id {
            None => return Ok(()),
            Some(_) => {}
        }

        let mut guild_id_string = guild_id.unwrap().to_string();
        guild_id_string.push_str(&*"ban_word".to_string());
        let ban_w: RedisResult<String> = redis.get(guild_id_string.clone()).await;
        match ban_w {
            Ok(x) => serde_json::from_str(&*x).unwrap(),
            _ => vec!["voldemort".to_string()],
        }
    };
    for bn in ban_words {
        let new_msg = new_message.content.clone();
        if (is_ban_word(bn.clone(), new_msg.clone()) || new_msg.to_lowercase().contains(&bn))
            && new_message.author.id != ctx.cache.current_user().id
        {
            new_message
                .reply(ctx, "Nao falamos esse nome aqui!!!".to_string())
                .await?;
            new_message.delete(ctx).await?;

            let intro_description = "Recentemente, recebemos relatos e observamos comportamentos que não estão de acordo com os padrões de conduta esperados neste servidor.".to_string();
            let motivo_description = format!(
                "Esse incidente ocorreu devido esta mensagem em particular: || **{}** ||",
                new_message.content
            );
            let intro = CreateEmbed::new()
                .title("Conduta Inapropriada")
                .description(intro_description)
                .color(Colour::RED);
            let motivo = CreateEmbed::new()
                .title("Principal Motivo")
                .description(motivo_description)
                .color(Colour::RED);
            let message_into = CreateMessage::new().add_embed(intro);
            let message_motivo = CreateMessage::new().add_embed(motivo);

            new_message.author.direct_message(ctx, message_into).await?;
            new_message
                .author
                .direct_message(ctx, message_motivo)
                .await?;
        }
    }

    Ok(())
}

fn is_ban_word(ban_word: String, msg: String) -> bool {
    let msg_lc = msg.to_lowercase();
    let binding = replace_caracters(msg_lc);
    let msg_replace = binding.0.chars();

    let mut count = 0;
    let mut count_n = 0;
    let mut is_b = false;
    let mut is_v = true;
    let mut is_validado_pelo_algoritmo = false;
    let mut distancia_do_algoritmo = 0;

    if ban_word.len() > msg.len() {
        msg_replace.for_each(|c| {
            if ban_word.contains(c) {
                count += 1
            };
            count_n += 1;
            is_v = (count + binding.1.len()).eq(&ban_word.len());
        });
    } else {
        if binding.0.len() < ban_word.len() {
            binding.0.chars().for_each(|c| {
                if ban_word.contains(c) {
                    count += 1
                };
                count_n += 1;
            })
        } else {
            ban_word.chars().for_each(|c| {
                if binding.0.contains(c) {
                    if !is_validado_pelo_algoritmo {
                        distancia_do_algoritmo = validacao_usando_levenshtein(
                            binding.0.clone(),
                            c,
                            ban_word.len(),
                            ban_word.clone(),
                        );
                        is_validado_pelo_algoritmo = true;
                    }
                    count += 1
                };
                count_n += 1;
            })
        }
    }

    if !binding.0.is_empty() && !binding.1.is_empty() && is_v {
        is_b = count.eq(&count_n);
    }

    let mut distancia_aceitavel = ban_word.len() / 2;
    if distancia_aceitavel.eq(&0) {
        distancia_aceitavel = 1
    }
    if is_validado_pelo_algoritmo && distancia_do_algoritmo < distancia_aceitavel {
        is_b = true
    }
    if is_validado_pelo_algoritmo && distancia_do_algoritmo > distancia_aceitavel {
        is_b = false
    }

    is_b
}

fn validacao_usando_levenshtein(p0: String, p1: char, p2: usize, ban_word: String) -> usize {
    return if let Some(inicio_da_palavra) = p0.find(p1) {
        let final_da_palavra = inicio_da_palavra + p2;
        if final_da_palavra <= p0.len() {
            let slice = &p0[inicio_da_palavra..final_da_palavra];
            let formated = slice.trim();
            let no_whitespace: String = formated.chars().filter(|c| !c.is_whitespace()).collect();
            strsim::levenshtein(&no_whitespace, &ban_word)
        } else {
            999
        }
    } else {
        999
    }
}


fn replace_caracters(msg: String) -> (String, String) {
    let letras = msg.chars();
    let mut msg_valida = "".to_string();
    let mut msg_invalida = "".to_string();
    for letra in letras {
        if letra.is_digit(10) || (!letra.is_alphanumeric() && !letra.is_whitespace()) {
            msg_invalida.push(letra);
        } else {
            msg_valida.push(letra);
        }
    }

    (msg_valida, msg_invalida)
}
