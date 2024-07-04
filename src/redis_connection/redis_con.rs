use redis::aio::MultiplexedConnection;

pub async fn get_redis_connection() -> MultiplexedConnection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let client = redis::Client::open("redis://redis-bot/").unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
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
