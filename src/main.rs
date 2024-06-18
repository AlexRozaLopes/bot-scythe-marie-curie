use std::collections::HashMap;
use std::sync::Mutex;

use poise::serenity_prelude as serenity;
use serenity::all::UserId;

use crate::event_handle::death_handle::death_handler;
use crate::event_handle::fun_handle::dont_say_this_name;
use crate::model::membro::Membro;

pub mod slash_command;
pub mod model;
pub mod event_handle;

pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
    membros: Mutex<HashMap<UserId, Membro>>,
}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![slash_command::age_command::age(), slash_command::details_command::getvotes(), slash_command::details_command::help(),
                           slash_command::details_command::vote(), slash_command::life_command::life()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { votes: Mutex::new(HashMap::new()), membros: Mutex::new(HashMap::new()) })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            death_handler(ctx,_framework, data, new_message).await?;
            dont_say_this_name(ctx, _framework, new_message).await?;
        }
        _ => {}
    }
    Ok(())
}