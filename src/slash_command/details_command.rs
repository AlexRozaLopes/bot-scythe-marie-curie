use std::collections::HashMap;

use poise::serenity_prelude as serenity;
use redis::AsyncCommands;
use serenity::all::{Member, UserId};
use serenity::builder::{CreateEmbedAuthor, CreateEmbedFooter};

use crate::{Context, Error};
use crate::model::membro::Membro;
use crate::redis_connection::redis_con::get_redis_connection;

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}

/// vote em alguma coisa
///
/// Enter `~vote pumpkin` to vote for pumpkins
#[poise::command(prefix_command, slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let num_votes = {
        let mut hash_map = ctx.data().votes.lock().unwrap();
        let num_votes = hash_map.entry(choice.clone()).or_default();
        *num_votes += 1;
        *num_votes
    };

    let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
    ctx.say(response).await?;
    Ok(())
}

/// Recupere todos os votos!
///
/// Retrieve the number of votes either in general, or for a specific choice:
/// ```
/// ~getvotes
/// ~getvotes pumpkin
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
pub async fn getvotes(
    ctx: Context<'_>,
    #[description = "Choice to retrieve votes for"] choice: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = choice {
        let num_votes = *ctx.data().votes.lock().unwrap().get(&choice).unwrap_or(&0);
        let response = match num_votes {
            0 => format!("Nobody has voted for {} yet", choice),
            _ => format!("{} people have voted for {}", num_votes, choice),
        };
        ctx.say(response).await?;
    } else {
        let mut response = String::new();
        for (choice, num_votes) in ctx.data().votes.lock().unwrap().iter() {
            response += &format!("{}: {} votes", choice, num_votes);
        }

        if response.is_empty() {
            response += "Nobody has voted for anything yet :(";
        }

        ctx.say(response).await?;
    };

    Ok(())
}

/// info sobre o bot!
#[poise::command(slash_command, prefix_command)]
pub async fn info_about_me(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let img_url = "https://avatars.githubusercontent.com/u/69591013?s=400&u=716d3458707ff7035b6d303db868118effed0495&v=4";
        let embed = serenity::CreateEmbed::default()
            .description("Bot para gerenciamento de servidores!!!")
            .footer(CreateEmbedFooter::new("\u{200B}").icon_url("https://img.icons8.com/?size=160&id=AeV543ttZrcT&format=png"))
            .author(CreateEmbedAuthor::new("AlexRoza").icon_url(img_url))
            .field("contato: ", "alex.roza.dev@gmail.com", false)
            .field("------------------------------------------", "", true);

        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new_link("https://github.com/AlexRozaLopes/bot-scythe-marie-curie")
                .label("oficial repo!")
                .style(serenity::ButtonStyle::Success),
        ])];

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    };

    ctx.send(reply).await?;

    Ok(())
}

pub async fn update_redis(new: &Option<Member>) -> Result<(), Error> {
    match new {
        None => {}
        Some(m) => {
            let mut redis = get_redis_connection().await;
            let membros_string: String = redis.get(m.guild_id.to_string()).await.unwrap();
            let mut membros: HashMap<UserId, Membro> = serde_json::from_str(&*membros_string).unwrap();

            let membro_old = membros.get(&m.user.id);
            match membro_old {
                None => {
                    let membro_new = Membro::new(m.clone());

                    membros.insert(m.user.id, membro_new);

                    let sm = serde_json::to_string(&membros)?;

                    let _: () = redis.set(m.guild_id.to_string(), sm).await.unwrap();
                }
                Some(mo) => {
                    let mut membro_new = Membro::new(m.clone());

                    membro_new.set_ativo_em(mo.ativo_em());

                    membros.insert(m.user.id, membro_new);

                    let sm = serde_json::to_string(&membros)?;

                    let _: () = redis.set(m.guild_id.to_string(), sm).await.unwrap();
                }
            }
        }
    }
    Ok(())
}