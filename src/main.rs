use std::collections::HashMap;
use std::sync::Mutex;

use poise::serenity_prelude as serenity;
use reqwest::Client as HttpClientVoice;
use serenity::all::GatewayIntents;
use songbird::SerenityInit;

use crate::event_handle::add_handle::add_role_a_new_user;
use crate::event_handle::create_roles::create_role_imunidade;
use crate::event_handle::death_handle::{death_handle_voice, death_handler};
use crate::event_handle::fun_handle::dont_say_this_name;
use crate::event_handle::silence_handle::{silence_handle, silence_handle_voice};
use crate::slash_command::details_command::update_redis;

pub mod slash_command;
pub mod model;
pub mod event_handle;
mod redis_connection;
pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
    http_client_voice: Mutex<HttpClientVoice>,
}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();
    let x = intents.contains(GatewayIntents::GUILD_VOICE_STATES);
    println!("{x}");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![slash_command::age_command::age(), slash_command::details_command::getvotes(), slash_command::details_command::help(),
                           slash_command::details_command::vote(), slash_command::life_command::life(), slash_command::details_command::info_about_me(),
                           slash_command::ban_words_command::add_ban_word(), slash_command::remove_ban_words_command::remove_ban_word(), slash_command::life_command::life_time(),
                           slash_command::life_command::get_life_time(), slash_command::silence_command::remove_silence_someone(), slash_command::silence_command::silence_someone(),
                           slash_command::silence_command::list_silence_people(), slash_command::remove_ban_words_command::list_ban_word(),
                           slash_command::voice_command::play_song(), slash_command::voice_command::join_(),
                           slash_command::voice_command::leave_(),slash_command::voice_command::queue_song()
            ],
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler(ctx, event, framework))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { votes: Mutex::new(HashMap::new()), http_client_voice: Mutex::new(HttpClientVoice::new()) })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            create_role_imunidade(ctx, _framework, data_about_bot).await?;
        }
        serenity::FullEvent::Message { new_message } => {
            death_handler(ctx, _framework, new_message).await?;
            dont_say_this_name(ctx, _framework, new_message).await?;
            silence_handle(ctx, _framework, new_message).await?;
        }
        serenity::FullEvent::GuildMemberAddition { new_member, .. } => {
            println!("membro novo!");
            add_role_a_new_user(ctx, _framework, new_member).await?;
        }
        serenity::FullEvent::VoiceStateUpdate { new, .. } => {
            death_handle_voice(ctx, _framework, new).await?;
            silence_handle_voice(ctx, _framework, new).await?;
        }
        serenity::FullEvent::GuildMemberUpdate { new, .. } => {
            update_redis(new).await.unwrap();
        }
        _ => {}
    }
    Ok(())
}