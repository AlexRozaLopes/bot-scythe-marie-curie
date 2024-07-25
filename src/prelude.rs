pub use crate::{handler, model::member::MemberModel, redis::redis_con, Data};
pub use chrono::{DateTime, Datelike, Utc};
pub use chrono_tz::Tz;
pub use poise::{
    serenity_prelude::all::{Context as SerenityContext, *},
    FrameworkContext,
};
pub use redis::{aio::MultiplexedConnection, AsyncCommands, Client as RedisClient, RedisResult};
pub use reqwest::Client as HttpClientVoice;
pub use serde_json::Value;
pub use songbird::{Call, SerenityInit};
pub use std::{
    collections::HashMap,
    process::{Command, Output},
    sync::Mutex,
};
pub use tokio::sync::MutexGuard;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

// User data, which is stored and accessible in all command invocations
pub type Context<'a> = poise::Context<'a, Data, Error>;
