use crate::prelude::*;

/// 🌃| Jogue town_night!
#[poise::command(slash_command, prefix_command)]
pub async fn play_town_night(ctx: Context<'_>) -> Result<()> {
    let channels_voice = ctx.guild().unwrap().voice_states.clone();
    let my_cv_id = channels_voice.get(&ctx.author().id).unwrap().channel_id.unwrap();
    for (user_id, channel) in channels_voice {
       if channel.channel_id.unwrap() == my_cv_id {
           dbg!(user_id);
       } 
    }
    ctx.say("Que o jogo comece!!!").await.unwrap();
    Ok(())
}
