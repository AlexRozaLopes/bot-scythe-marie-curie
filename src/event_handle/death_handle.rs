use poise::serenity_prelude as serenity;
use serenity::all::Message;

use crate::{Data, Error};
use crate::model::membro::Membro;

pub async fn death_handler(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    new_message: &Message,
) -> Result<(), Error> {
    let mut membros = data.membros.lock().unwrap();
    if membros.is_empty() {
        let hash_map = &ctx.cache.guild(&new_message.guild_id.unwrap()).unwrap().members;
        hash_map.iter().for_each(|(id, m)| {
            membros.insert(*id, Membro::new(m.clone()));
        })
    }
    membros.entry(new_message.clone().author.id).and_modify(|m| m.set_ativo(true));
    drop(dbg!(membros));
    Ok(())
}