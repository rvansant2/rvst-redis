extern crate log;
extern crate pretty_env_logger;

use std::env;
use std::convert::Infallible;
use thiserror::Error;

use warp::{http::StatusCode, Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};

mod redislib;

//TODO: Move to data.rs and debug
#[derive(Debug, Serialize, Deserialize)]
pub struct JSONResponse {
  pub message: String,
}

#[derive(Deserialize)]
pub struct PostRedisRequest {
  pub key: String,
  pub value: String,
  pub ttl_seconds: Option<usize>
}

#[derive(Deserialize)]
pub struct RedisQuery {
  pub key: String,
}

type WebResult<T> = std::result::Result<T, Rejection>;
type Result<T> = std::result::Result<T, Error>;

const REDIS_CONNECTION_STRING: &str = "redis://redis:6379/";

#[tokio::main]
async fn main() {

  env::set_var("RUST_LOG", "info,routing=debug");
  pretty_env_logger::init();

  let redis_client = redis::Client::open(REDIS_CONNECTION_STRING).expect("Redis client connected");
  let home_route = warp::any().and_then(default_handler);
  let redis_base_route = warp::path!("redis");
  let redis_route = redis_base_route
    .and(warp::post())
    .and(warp::body::json())
    .and(with_redis_client(redis_client.clone()))
    .and_then(redis_set_handler)
    .or(redis_base_route
      .and(warp::get())
      .and(warp::query())
      .and(with_redis_client(redis_client.clone()))
      .and_then(redis_get_handler))
    .or(redis_base_route
      .and(warp::delete())
      .and(warp::body::json())
      .and(with_redis_client(redis_client.clone()))
      .and_then(redis_del_handler));
  
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
  message = "Welcome to RVST Redis!";
  
  let json = warp::reply::json(&JSONResponse {
    message: message.into(),
  });

  Ok(warp::reply::with_status(json, code))
}

async fn redis_set_handler(body: PostRedisRequest, client: redis::Client) -> WebResult<impl Reply> {
  let message;
  let code;

  let mut con = redislib::get_con(client)
      .await
      .map_err(|e| warp::reject::custom(e))?;
      redislib::set_str(&mut con, &body.key, &body.value, None)
      .await
      .map_err(|e| warp::reject::custom(e))?;
  let value = redislib::get_str(&mut con, &body.key)
      .await
      .map_err(|e| warp::reject::custom(e))?;

  code = StatusCode::OK;
  message = value;
  let json = warp::reply::json(&JSONResponse {
    message: message.into(),
  });
  Ok(warp::reply::with_status(json, code))
}

async fn redis_get_handler(body: RedisQuery, client: redis::Client) -> WebResult<impl Reply> {
  let message;
  let code;

  let mut con = redislib::get_con(client)
      .await
      .map_err(|e| warp::reject::custom(e))?;

  let value = redislib::get_str(&mut con, &body.key)
      .await
      .map_err(|e| warp::reject::custom(e))?;

  code = StatusCode::OK;
  message = value;
  let json = warp::reply::json(&JSONResponse {
    message: message.into(),
  });
  Ok(warp::reply::with_status(json, code))
}

async fn redis_del_handler(body: RedisQuery, client: redis::Client) -> WebResult<impl Reply> {
  let message;
  let code;

  let mut con = redislib::get_con(client)
      .await
      .map_err(|e| warp::reject::custom(e))?;

  let value = redislib::remove_str(&mut con, &body.key)
      .await
      .map_err(|e| warp::reject::custom(e))?;

  code = StatusCode::OK;
  message = value;
  let json = warp::reply::json(&JSONResponse {
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

#[cfg(test)]
mod tests {
  use super::*;
  #[tokio::test]
  async fn test_redis() {
    let res = warp::test::request()
      .method("GET")
      .path("/redis")
      .reply(&std::result::Result::Ok)
      .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");

    println!("{:#?}", res.body());
  }
}
