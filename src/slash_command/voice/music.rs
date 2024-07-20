use crate::prelude::*;

/// 🎧| Musica!
#[poise::command(slash_command, prefix_command)]
pub async fn play_song(
    ctx: Context<'_>,
    #[description = "Digite a URL da musica OU seu NOME!"] url: String,
) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let mut title_music_f = "".to_string();

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
                title_music_f = title_music;
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
                title_music_f = get_title_music(&output);
            }

            songbird::input::YoutubeDl::new_search(client, url)
        };

        let input1 = input_audio.clone().into();
        let _ = handler.enqueue_input(input1).await;
    }

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
pub async fn join_(ctx: Context<'_>) -> Result<()> {
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
            ctx.say("Não foi possível encontrar o ID do canal de voz.")
                .await?;
            return Ok(());
        }
    };

    ctx.say("Ok, I'll go there").await?;

    let _handler = manager.join(guild_id, channel_id).await;

    Ok(())
}

/// 🎫| Tire a Marie do chat de voz!
#[poise::command(slash_command, prefix_command)]
pub async fn leave_(ctx: Context<'_>) -> Result<()> {
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
pub async fn skip_(ctx: Context<'_>) -> Result<()> {
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    ctx.say("Next Music!").await?;

    if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();
    }

    Ok(())
}

/// ⏹️| Pare a musica!
#[poise::command(slash_command, prefix_command)]
pub async fn stop_(ctx: Context<'_>) -> Result<()> {
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    ctx.say("Stop Music!").await?;

    if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();
    }

    Ok(())
}

/// 🎼| Crie sua playlist de musica!
#[poise::command(slash_command, prefix_command)]
pub async fn create_playlist(
    ctx: Context<'_>,
    #[description = "Digite o nome da playlist"] nome: String,
    #[description = "Digite sua lista de musicas"] musicas: String,
) -> Result<()> {
    let result = redis_con::get_playlist(ctx.author().id).await;

    match result {
        Err(_) => {
            let mut playlist: HashMap<String, String> = HashMap::new();
            playlist.insert(nome, musicas);
            redis_con::set_playlist(ctx.author().id, playlist)
                .await
                .unwrap();
        }
        Ok(mut list) => {
            list.insert(nome, musicas);
            redis_con::set_playlist(ctx.author().id, list)
                .await
                .unwrap();
        }
    }

    ctx.say("Lista salva com Sucesso!!!").await.unwrap();
    Ok(())
}

/// 🎼| Toque sua playlist de musica!
#[poise::command(slash_command, prefix_command)]
pub async fn play_playlist(
    ctx: Context<'_>,
    #[description = "Digite o nome da playlist"] nome: String,
) -> Result<()> {
    let result = redis_con::get_playlist(ctx.author().id).await;

    match result {
        Err(_) => {
            ctx.say("Nenhuma playlist com esse NOME encontrada.")
                .await
                .unwrap();
        }
        Ok(list) => {
            let option = list.get(&nome);
            match option {
                None => {
                    ctx.say("Nenhuma playlist com esse NOME encontrada.")
                        .await
                        .unwrap();
                }
                Some(lista) => {
                    ctx.say("Playing...").await.unwrap();

                    let guild_id = ctx.guild_id().unwrap();
                    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

                    if let Some(handler_lock) = manager.get(guild_id) {
                        let mut handler = handler_lock.lock().await;
                        let mut titulos_da_musicas = "".to_string();

                        let split = lista.split(";");
                        for m in split {
                            let string =
                                song(ctx, m.to_string(), &mut handler, titulos_da_musicas).await;
                            titulos_da_musicas = string;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

async fn song(
    ctx: Context<'_>,
    url: String,
    handler: &mut MutexGuard<'_, Call>,
    mut titulos_da_musicas: String,
) -> String {
    let client = ctx.data().http_client_voice.lock().unwrap().clone();

    let is_url = url.starts_with("http");

    let input_audio = if is_url {
        let output = Command::new("yt-dlp")
            .arg("--dump-json")
            .arg(url.clone())
            .output()
            .expect("Erro em buscar o JSON ;-;");

        if output.status.success() {
            let title_music = get_title_music(&output);
            titulos_da_musicas.push_str(&*title_music);
            titulos_da_musicas.push_str(";");
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
            titulos_da_musicas.push_str(&*title_music);
            titulos_da_musicas.push_str(";");
        }

        songbird::input::YoutubeDl::new_search(client, url)
    };

    let input1 = input_audio.clone().into();
    let _ = handler.enqueue_input(input1).await;

    let mut mutex_guard = ctx.data().music.lock().unwrap();

    *mutex_guard = titulos_da_musicas.clone();

    titulos_da_musicas
}
