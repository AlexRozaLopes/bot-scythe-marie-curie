use crate::prelude::*;

pub mod handler {
    pub mod add;
    pub mod create_roles;
    pub mod death;
    pub mod fun;
    pub mod music;
    pub mod silence;
}
pub mod model {
    pub mod member;
}
pub mod prelude;
pub mod redis_con;

mod slash_command {
    pub mod general {
        pub mod age;
        pub mod ban_words;
        pub mod details;
        pub mod life;
        pub mod remove_ban_words;
        pub mod silence;
    }
    pub mod voice {
        pub mod music;
    }
}

pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
    http_client_voice: Mutex<HttpClientVoice>,
    music: Mutex<String>,
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = GatewayIntents::all();

    use slash_command::*;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                general::age::age(),
                general::details::getvotes(),
                general::details::help(),
                general::details::vote(),
                general::life::life(),
                general::details::info_about_me(),
                general::ban_words::add_ban_word(),
                general::remove_ban_words::remove_ban_word(),
                general::life::life_time(),
                general::life::get_life_time(),
                general::silence::remove_silence_someone(),
                general::silence::silence_someone(),
                general::silence::list_silence_people(),
                general::remove_ban_words::list_ban_word(),
                voice::music::play_song(),
                voice::music::join_(),
                voice::music::leave_(),
                voice::music::skip_(),
                voice::music::stop_(),
                voice::music::create_playlist(),
                voice::music::play_playlist(),
            ],
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler(ctx, event, framework))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                    http_client_voice: Mutex::new(HttpClientVoice::new()),
                    music: Mutex::new("".to_string()),
                })
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &SerenityContext,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
) -> Result<()> {
    use handler::*;
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            create_roles::create_role_imunidade(ctx, _framework, data_about_bot).await?;
        }
        FullEvent::Message { new_message } => {
            death::death_handler(ctx, _framework, new_message).await?;
            fun::dont_say_this_name(ctx, _framework, new_message).await?;
            silence::silence_handle(ctx, _framework, new_message).await?;
        }
        FullEvent::GuildMemberAddition { new_member, .. } => {
            println!("membro novo!");
            add::add_role_a_new_user(ctx, _framework, new_member).await?;
        }
        FullEvent::VoiceStateUpdate { new, .. } => {
            death::death_handle_voice(ctx, _framework, new).await?;
            silence::silence_handle_voice(ctx, _framework, new).await?;
        }
        FullEvent::GuildMemberUpdate { new, .. } => {
            slash_command::general::details::update_redis(new)
                .await
                .unwrap();
        }
        FullEvent::InteractionCreate { interaction } => {
            music::say_title_music(ctx, _framework, interaction)
                .await
                .unwrap();
        }
        _ => {}
    }
    Ok(())
}
