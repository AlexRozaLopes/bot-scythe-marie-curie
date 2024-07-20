use crate::prelude::*;

pub async fn silence_handle_voice(
    ctx: &SerenityContext,
    _framework: FrameworkContext<'_, Data, Error>,
    new: &VoiceState,
) -> Result<()> {
    let member_models = redis_con::get_member_models(new.clone().guild_id.unwrap()).await;
    let silenced_member_models = filter_silenced_member_models(member_models);

    let user_id = new.member.clone().unwrap().user.id;
    let option_m = silenced_member_models.get(&user_id);

    match option_m {
        None => {}
        Some(_) => {
            new.member
                .clone()
                .unwrap()
                .disconnect_from_voice(ctx)
                .await
                .expect("TODO: panic message");
        }
    }

    Ok(())
}

pub async fn silence_handle(
    ctx: &SerenityContext,
    _framework: FrameworkContext<'_, Data, Error>,
    new_message: &Message,
) -> Result<()> {
    let guild_id = new_message.guild_id;
    match guild_id {
        None => {}
        Some(g) => {
            let member_models = redis_con::get_member_models(g).await;
            let silenced_member_models = filter_silenced_member_models(member_models);

            let user_id = new_message.author.id;
            let option_m = silenced_member_models.get(&user_id);

            match option_m {
                None => {}
                Some(_) => {
                    new_message.delete(ctx).await.unwrap();
                }
            }
        }
    }

    Ok(())
}

pub fn filter_silenced_member_models(
    member_models: HashMap<UserId, MemberModel>,
) -> HashMap<UserId, MemberModel> {
    member_models
        .iter()
        .filter(|(_, m)| m.is_silenced())
        .map(|(id, m)| (id.clone(), m.clone()))
        .collect()
}
