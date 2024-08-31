use std::ops::Add;

use rand::Rng;

use crate::{
    model::game::{Player, TownNight, DAY, ROLE},
    prelude::*,
};

/// 🖼️| Faca sua acao!
#[poise::command(slash_command, prefix_command)]
pub async fn action_town_night(
    ctx: Context<'_>,
    player: Option<User>,
    skip: Option<String>,
) -> Result<()> {
    let channels_voice = ctx.guild().unwrap().voice_states.clone();
    let mut my_cv_id = channels_voice
        .get(&ctx.author().id)
        .unwrap()
        .channel_id
        .unwrap()
        .to_string();

    let mut redis = redis_con::get_connection().await;
    my_cv_id.push_str("town_night");
    let game_string: String = redis.get(&my_cv_id).await.unwrap();
    let game: TownNight = serde_json::from_str(&game_string).unwrap();
    if game.day.eq(&DAY::MORNING) {
        let mut game_update = vote_player(
            ctx.author().id.clone(),
            Some(player.clone().unwrap().id),
            game,
            skip,
        );
        let _ = ctx.say("Acao registrada!").await.unwrap();
        let count = game_update.players.iter().filter(|p|p.action).count();
        if count == game_update.players.len() {
            let mut max_vote = 0;
            let mut user_id = UserId::new(0);
            for (id, numero_votes) in game_update.votes {
                if numero_votes > max_vote {
                    max_vote = numero_votes;
                    user_id = id;
                }
            }
            let player = game_update
                .players
                .iter()
                .filter(|p| p.id.eq(&user_id))
                .next()
                .unwrap();
            let response = format!("votacao acabou o {} foi eliminado", player.name);
            ctx.say(response).await.unwrap();
            game_update.players.retain(|p| p.id.eq(&user_id));
            ctx.say("O dia acabou! Cuidado com os PERIGOS da noite!")
                .await
                .unwrap();
            game_update.day = DAY::NIGHT;
            game_update.votes = HashMap::new();
            game_update
                .players
                .iter_mut()
                .for_each(|p| p.action = false);
            let gs = serde_json::to_string(&game_update).unwrap();
            let _: () = redis.set_ex(my_cv_id.clone(), gs, 1800).await.unwrap();
        }
    }

    let game_string: String = redis.get(&my_cv_id).await.unwrap();
    let mut game: TownNight = serde_json::from_str(&game_string).unwrap();
    if game.day.eq(&DAY::NIGHT) {
        let _ = ctx
            .say("testando se foi para noite mesmo kkk")
            .await
            .unwrap();
        let _ = action_player(ctx.author().id, &ctx, player, &mut game).await;
        let actions: Vec<&Player> = game.players.iter().filter(|p|p.action).collect();
        let murders_live: Vec<&Player> = game.players.iter().filter(|p|!p.is_death && p.role.eq(&ROLE::MURDERER)).collect();
        let not_murders_live: Vec<&Player> = game.players.iter().filter(|p|!p.is_death && !p.role.eq(&ROLE::MURDERER)).collect();
        if game.players.len().eq(&actions.len()) {
            
            let death_pl = game.players.iter().filter(|p|p.is_death).next();
            if let Some(p) = death_pl {
                let msg = format!("O jogador {} foi morto",p.name);
                let _ = ctx.say(msg).await;
                let _ = ctx.say("Comeca mais um dia na vila!").await;
                game.day = DAY::MORNING;
                let gs = serde_json::to_string(&game).unwrap();
                let _: () = redis.set_ex(my_cv_id.clone(), gs, 1800).await.unwrap();
            }
        }
        if murders_live.len() >= not_murders_live.len() {
            let _ = ctx.say("Os Assasinos ganharam!").await;
        }
        if murders_live.len() == 0{
            let _ = ctx.say("A vila venceu!").await;
        }
    }
    Ok(())
}

async fn action_player(
    id: UserId,
    ctx: &Context<'_>,
    target: Option<User>,
    game: &mut TownNight,
) -> Result<()> {
    let player = game.players.iter().filter(|p| p.id.eq(&id)).next();
    if player.is_some() {
        let player = player.unwrap().clone();
        if !player.action && !player.is_death{
            if let Some(p_target) = target {
                if player.role.eq(&ROLE::CARTOMANTE) {
                    let info_target = game.players.iter().filter(|p| p.id.eq(&p_target.id)).next();
                    if let Some(info) = info_target {
                        let msg = format!("O {} e um {}", info.name, info.role);
                        let _ = ctx
                            .author()
                            .member
                            .clone()
                            .unwrap()
                            .user
                            .unwrap()
                            .direct_message(&ctx.http(), CreateMessage::new().content(msg))
                            .await;
                    }
                }
                if player.role.eq(&ROLE::MEDIC) {
                    let info_target = game.players.iter_mut().filter(|p| p.id.eq(&p_target.id)).next();
                    if let Some(info) = info_target {
                        info.is_death = false;
                    }
                }
                if player.role.eq(&ROLE::MURDERER) {
                    let info_target = game.players.iter_mut().filter(|p| p.id.eq(&p_target.id)).next();
                    if let Some(info) = info_target {
                        info.is_death = true;
                    }
                }
            }
        }
    }
    if let Some(p_a) = game.players.iter_mut().filter(|p| p.id.eq(&id)).next() {
        p_a.action = true;
    }
    let _ = ctx.say("Comando Registrado!").await;
    Ok(())
}

fn vote_player(
    owner: UserId,
    target: Option<UserId>,
    mut game: TownNight,
    skip: Option<String>,
) -> TownNight {
    let mut is_owner = false;
    let mut is_target = false;
    if target.is_some() {
        game.players.iter_mut().for_each(|ele: &mut Player| {
            if !ele.action && !ele.is_death {
                if ele.id.eq(&owner) {
                    is_owner = true;
                    ele.action = true;
                }
                if ele.id.eq(&target.unwrap()) {
                    is_target = true;
                }
            }
        });
    }
    if is_owner && is_target {
        let number_of_votes = game.votes.get(&target.unwrap()).unwrap_or(&0);
        let new_vote = number_of_votes.add(1);
        let _ = game.votes.insert(target.unwrap(), new_vote);
    }

    if skip.is_some() {
        let _ = game.skip.add(1);
    }
    game
}

/// 🌃| Jogue town_night!
#[poise::command(slash_command, prefix_command)]
pub async fn play_town_night(ctx: Context<'_>) -> Result<()> {
    let channels_voice = ctx.guild().unwrap().voice_states.clone();
    let my_cv_id = channels_voice
        .get(&ctx.author().id)
        .unwrap()
        .channel_id
        .unwrap();
    let mut users_in_call = vec![];
    for (user_id, channel) in channels_voice {
        if channel.channel_id.unwrap() == my_cv_id {
            users_in_call.push(user_id);
        }
    }
    let guild_id = ctx.guild_id().unwrap();
    let dic_members = redis_con::get_member_models(guild_id).await;
    let mut list_members = vec![];
    for id in users_in_call {
        let m = dic_members.get(&id).unwrap();
        list_members.push(m);
    }
    if list_members.len() < 4 {
        ctx.say("Para poder jogar e preciso de 4 jogadores no minimo!")
            .await
            .unwrap();
    } else {
        start_game(list_members, my_cv_id, ctx).await.unwrap();
        ctx.say("Que o jogo comece!!!").await.unwrap();
    }
    Ok(())
}

async fn start_game(
    list_members: Vec<&MemberModel>,
    channel_id: ChannelId,
    ctx: Context<'_>,
) -> Result<()> {
    let list_players: Vec<Player> = list_members
        .iter()
        .map(|m| Player::new(m.member().user.id, m.member().user.clone().name))
        .collect();
    let list_players = set_roles_to_players(list_players);
    let _ = dm_for_players(list_players.clone(), ctx).await;
    let game = TownNight::new(list_players);
    let mut redis = redis_con::get_connection().await;
    let value = serde_json::to_string(&game).unwrap();
    let mut channel_id = channel_id.to_string();
    channel_id.push_str("town_night");
    let _: () = redis.set_ex(channel_id, value, 1800).await.unwrap();
    Ok(())
}

async fn dm_for_players(list_players: Vec<Player>, ctx: Context<'_>) -> Result<()> {
    for ele in list_players {
        let members = redis_con::get_member_models(ctx.guild_id().unwrap()).await;
        let member = members.get(&ele.id).unwrap();
        let msg = format!("VC e o {}", ele.role.to_string());
        let _ = member
            .member()
            .user
            .direct_message(ctx.http(), CreateMessage::new().content(msg))
            .await;
    }
    Ok(())
}

fn set_roles_to_players(mut list_players: Vec<Player>) -> Vec<Player> {
    let number_of_players = list_players.len();
    if number_of_players == 4 {
        list_players = set_role(list_players, ROLE::MURDERER);
    } else if number_of_players > 4 && number_of_players < 6 {
        list_players = set_role(list_players, ROLE::MURDERER);
        list_players = set_role(list_players, ROLE::CARTOMANTE);
    } else if number_of_players >= 7 {
        list_players = set_role(list_players, ROLE::MURDERER);
        list_players = set_role(list_players, ROLE::MURDERER);
        list_players = set_role(list_players, ROLE::CARTOMANTE);
        list_players = set_role(list_players, ROLE::MEDIC);
    }
    list_players
}

fn set_role(mut list_players: Vec<Player>, role: ROLE) -> Vec<Player> {
    let mut rd = rand::thread_rng();
    let cp = rd.gen_range(0..=list_players.len());
    let p = list_players.get_mut(cp).unwrap();
    if ROLE::MURDERER != p.role {
        p.role = role;
    }
    list_players
}
