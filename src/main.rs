use axum::routing::get;
use axum::Router;
use chrono::Utc;
use redis::{Commands, RedisResult};
use std::env;
use tokio::net::TcpListener;

const KEY: &str = "last_saved";
const REDIS: &str = "rediss://default:{}leapcell-wsjm-jigi-424512.leapcell.cloud:6379";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/save", get(save))
        .route("/read", get(read));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    println!("Root called");
    "Hello World"
}

async fn save() -> Result<&'static str, String> {
    save_redis().map_err(|err| err.category().to_string())
}

fn save_redis() -> RedisResult<&'static str> {
    let password = env::var("REDIS_PASSWORD").unwrap();
    let endpoint = format!("redis://default:{}@leapcell-wsjm-jigi-424512.leapcell.cloud:6379", password);
    let client = redis::Client::open(endpoint)?;

    let mut connection = client.get_connection()?;

    connection.set(KEY, Utc::now().to_rfc3339())
        .map(|ok: ()| "OK")
}

async fn read() -> Result<String, String> {
    read_redis().map_err(|err| err.category().to_string())
}

fn read_redis() -> RedisResult<String> {
    let password = env::var("REDIS_PASSWORD").unwrap();
    let endpoint = format!("redis://default:{}@leapcell-wsjm-jigi-424512.leapcell.cloud:6379", password);
    let client = redis::Client::open(endpoint)?;

    let mut connection = client.get_connection()?;

    connection.get(KEY)
}