use crate::{Context, Error};


/// 🎶| Musica!
#[poise::command(slash_command, prefix_command)]
pub async fn play_song(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();


    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let client = ctx.data().http_client_voice.lock().unwrap().clone();
        let string = "https://www.youtube.com/watch?v=q392mSz4VeY".to_string();
        let source = songbird::input::YoutubeDl::new(client, string);

        let input1 = source.clone().into();
        let _ = handler.play_only_input(input1);
        ctx.say("play song!").await.unwrap();
    } else {
    }

    Ok(())
}

/// 🎫| Chame a Marie para o chat de voz!
#[poise::command(slash_command, prefix_command)]
pub async fn join_(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    // Armazene o valor do guild em uma variável para evitar o problema de vida temporária
    let guild = match ctx.guild() {
        Some(guild) => guild.clone(),
        None => {
            return Ok(());
        }
    };

    let voice_state = match guild.voice_states.get(&ctx.author().id) {
        Some(state) => state,
        None => {
            ctx.say("Você não está em um canal de voz.").await?;
            return Ok(());
        }
    };

    let guild_id = guild.id;
    let channel_id = match voice_state.channel_id {
        Some(channel_id) => channel_id,
        None => {
            ctx.say("Não foi possível encontrar o ID do canal de voz.").await?;
            return Ok(());
        }
    };

    ctx.say("OK").await?;

    let _handler = manager.join(guild_id, channel_id).await;

    Ok(())
}

/// 🎫| Tire a Marie do chat de voz!
#[poise::command(slash_command, prefix_command)]
pub async fn leave_(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    // Armazene o valor do guild em uma variável para evitar o problema de vida temporária
    let guild = match ctx.guild() {
        Some(guild) => guild.clone(),
        None => {
            return Ok(());
        }
    };
    let guild_id = guild.id;

    ctx.say("OK").await?;

    let _handler = manager.leave(guild_id).await;

    Ok(())
}
