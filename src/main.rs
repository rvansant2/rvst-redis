extern crate log;
extern crate pretty_env_logger;

use std::env;
use std::convert::Infallible;
use thiserror::Error;

use warp::{http::StatusCode, Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};

mod redislib;

#[derive(Debug, Serialize, Deserialize)]
struct HttpResponse {
  message: String,
}

type WebResult<T> = std::result::Result<T, Rejection>;
type Result<T> = std::result::Result<T, Error>;

const REDIS_CON_STRING: &str = "redis://127.0.0.1:6379/";

#[tokio::main]
async fn main() {

  env::set_var("RUST_LOG", "info,routing=debug");
  pretty_env_logger::init();

  let redis_client = redis::Client::open(REDIS_CON_STRING).expect("Redis client connected");
  let home_route = warp::any().and_then(default_handler);
  let redis_route = warp::path!("redis")
    .and(with_redis_client(redis_client.clone()))
    .and_then(redis_handler);
  
  let routes = redis_route
    .or(home_route)
    .with(warp::cors().allow_any_origin());

  println!("Started server at localhost:8000");
  warp::serve(routes.with(warp::log("warp-server")))
    .run(([0, 0, 0, 0], 8000)).await;
}

async fn default_handler() -> WebResult<impl Reply> {
  let message;
  let code;
  code = StatusCode::OK;
  message = "Welcome to RVST!";
  
  let json = warp::reply::json(&HttpResponse {
    message: message.into(),
  });

  Ok(warp::reply::with_status(json, code))
}

async fn redis_handler(client: redis::Client) -> WebResult<impl Reply> {
  let message;
  let code;

  let mut con = redislib::get_con(client)
      .await
      .map_err(|e| warp::reject::custom(e))?;
      redislib::set_str(&mut con, "hello", "direct_world", 60)
      .await
      .map_err(|e| warp::reject::custom(e))?;
  let value = redislib::get_str(&mut con, "hello")
      .await
      .map_err(|e| warp::reject::custom(e))?;

  code = StatusCode::OK;
  message = value;
  let json = warp::reply::json(&HttpResponse {
    message: message.into(),
  });
  Ok(warp::reply::with_status(json, code))
}

fn with_redis_client(
  client: redis::Client,
) -> impl Filter<Extract = (redis::Client,), Error = Infallible> + Clone {
  warp::any().map(move || client.clone())
}

#[derive(Error, Debug)]
pub enum RedisError {
  #[error("error parsing string from redis result: {0}")]
  RedisTypeError(redis::RedisError),
  #[error("error executing redis command: {0}")]
  RedisCMDError(redis::RedisError),
  #[error("error creating redis client: {0}")]
  RedisClientError(redis::RedisError),
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("redislib error: {0}")]
  RedisError(#[from] RedisError),
}

impl warp::reject::Reject for Error {}
