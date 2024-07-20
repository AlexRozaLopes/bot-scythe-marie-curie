use poise::{FrameworkContext, serenity_prelude as serenity};
use serenity::all::{EditInteractionResponse, Interaction};
use serenity::builder::CreateEmbed;

use crate::{Data, Error};

pub async fn say_title_music(ctx: &serenity::Context, _framework: FrameworkContext<'_, Data, Error>, interaction: &Interaction) -> Result<(), Error> {
    let data = interaction.clone().command().unwrap().data;
    if data.name.eq("play_song") {
        let mutex = _framework.user_data.music.lock().unwrap().to_string();
        let string = format!("**{mutex}**");
        let embed = CreateEmbed::new().description(string);
        let response = EditInteractionResponse::new().embed(embed);
        let _ = interaction.clone().command().unwrap().edit_response(ctx, response).await;
    }

    if data.name.eq("play_playlist") {
        let mutex = _framework.user_data.music.lock().unwrap().to_string();
        let mut vec: Vec<CreateEmbed> = Vec::new();
        let split = mutex.split(";");
        split.for_each(|t| {
            if !t.is_empty() {
                let string = format!("**{t}**");
                let embed = CreateEmbed::new().description(string);
                vec.push(embed);
            }
        });

        let response = EditInteractionResponse::new().add_embeds(vec);
        let _ = interaction.clone().command().unwrap().edit_response(ctx, response).await;
    }

    Ok(())
}
