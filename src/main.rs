use std::collections::HashMap;
use std::sync::Mutex;
use poise::serenity_prelude as serenity;

pub mod slash_command;

pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
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
            commands: vec![slash_command::age_command::age(),slash_command::details_command::getvotes(),slash_command::details_command::help(),
                           slash_command::details_command::vote(), slash_command::life_command::life()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {votes: Mutex::new(HashMap::new())})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}