use std::collections::HashMap;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use serenity::all::{GuildId, UserId};
use crate::model::membro::Membro;

pub async fn get_redis_connection() -> MultiplexedConnection {
    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let client = redis::Client::open("redis://redis-bot/").unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
}

pub async fn get_membros_redis(guild_id: GuildId) -> HashMap<UserId,Membro> {
    let mut redis = get_redis_connection().await;
    let membros_json: String = redis.get(guild_id.to_string()).await.unwrap();

    serde_json::from_str(&*membros_json).unwrap()
}

pub async fn set_membros_redis(guild_id: GuildId, membros: HashMap<UserId,Membro>) -> Result<(),()> {
    let mut redis = get_redis_connection().await;

    let string_membros = serde_json::to_string(&membros).unwrap();

    let _: () = redis.set(guild_id.to_string(), string_membros).await.unwrap();

    Ok(())

}

#[tokio::test]
async fn test_fetch_an_integer() {
    // Configurar um cliente Redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // Configurar o valor inicial para garantir um estado conhecido
    let _: () = con.set("my_key", 0).unwrap();

    // Chamar a função a ser testada
    let result: i32 = {
        // connect to redis
        let mut con = get_redis_connection().await;
        // throw away the result, just make sure it does not fail
        let _: () = con.set("my_key", 43).await.unwrap();
        // read back the key and return it.  Because the return value
        // from the function is a result for integer this will automatically
        // convert into one.
        let result = con.get("my_key").await.unwrap();

        Ok::<i32, i32>(result)
    }.unwrap();

    // Verificar se o valor retornado é o esperado
    assert_eq!(result, 43);
}
