use std::process::{Command, Output};

use serde_json::Value;

use crate::{Context, Error};

/// 🎧| Musica!
#[poise::command(slash_command, prefix_command)]
pub async fn play_song(
    ctx: Context<'_>,
    #[description = "Digite a URL da musica OU seu NOME!"]  url: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let mut title_music_f= "".to_string();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;


        let client = ctx.data().http_client_voice.lock().unwrap().clone();

        let is_url = url.starts_with("http");

        ctx.say("playing...").await.unwrap();

        let input_audio = if is_url {
            let output = Command::new("yt-dlp")
                .arg("--dump-json")
                .arg(url.clone())
                .output()
                .expect("Erro em buscar o JSON ;-;");


            if output.status.success() {
                let title_music = get_title_music(&output);
                title_music_f = format!("play: {title_music}");
            }

            songbird::input::YoutubeDl::new(client, url)
        } else {
            let args_search = format!("ytsearch:{url}");

            let output = Command::new("yt-dlp")
                .arg("--dump-json")
                .arg(args_search)
                .output()
                .expect("Erro em buscar o JSON ;-;");


            if output.status.success() {
                let title_music = get_title_music(&output);
                title_music_f = format!("play: {title_music}");
            }

            songbird::input::YoutubeDl::new_search(client, url)
        };

        let input1 = input_audio.clone().into();
        let _ = handler.enqueue_input(input1).await;
    } else {}

    let mut mutex_guard = ctx.data().music.lock().unwrap();

    *mutex_guard = title_music_f;

    Ok(())
}

fn get_title_music(output: &Output) -> String {
    // Converte a saída do yt-dlp para string
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parseia a saída JSON
    let json: Value = serde_json::from_str(&stdout).expect("Failed to parse JSON");

    // Obtém o título do vídeo
    let title = json["title"].as_str().expect("Failed to get title");

    title.to_string()
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

    ctx.say("Ok, I'll go there").await?;

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

    ctx.say("Bye!").await?;

    let _handler = manager.leave(guild_id).await;

    Ok(())
}

/// ⏭️| Proxima musica!
#[poise::command(slash_command, prefix_command)]
pub async fn skip_(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    ctx.say("Next Music!").await?;

    if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();
    } else {}

    Ok(())
}


