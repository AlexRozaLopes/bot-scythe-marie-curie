use crate::prelude::*;

pub async fn get_connection() -> MultiplexedConnection {
    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let client = redis::Client::open("redis://redis-bot/").unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
}

pub async fn get_member_models(guild_id: GuildId) -> HashMap<UserId, MemberModel> {
    let mut redis = get_connection().await;
    let member_models_json: String = redis.get(guild_id.to_string()).await.unwrap();

    serde_json::from_str(&member_models_json).unwrap()
}

pub async fn get_ban_word(guild_id: GuildId) -> Vec<String> {
    let mut redis = get_connection().await;
    let mut string_guild = guild_id.to_string();
    string_guild.push_str("ban_word");
    let json_string: String = redis.get(string_guild).await.unwrap();

    serde_json::from_str(&json_string).unwrap()
}

pub async fn get_playlist(user_id: UserId) -> serde_json::Result<HashMap<String, String>> {
    let mut redis = get_connection().await;
    let mut string_guild = user_id.to_string();
    string_guild.push_str("playlist");
    let json_string: String = redis.get(string_guild).await.unwrap_or("".to_string());

    serde_json::from_str(&json_string)
}

pub async fn set_playlist(user_id: UserId, playlist: HashMap<String, String>) -> Result<()> {
    let mut redis = get_connection().await;

    let string_playlist = serde_json::to_string(&playlist).unwrap();

    let mut string = user_id.to_string();
    string.push_str("playlist");

    let _: () = redis.set(string, string_playlist).await.unwrap();

    Ok(())
}

pub async fn set_member_models(
    guild_id: GuildId,
    membros: HashMap<UserId, MemberModel>,
) -> Result<()> {
    let mut redis = get_connection().await;

    let string_member_models = serde_json::to_string(&membros).unwrap();

    let _: () = redis
        .set(guild_id.to_string(), string_member_models)
        .await
        .unwrap();

    Ok(())
}

