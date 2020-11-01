use serde_derive::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum RedisError {
  #[error("error parsing string from redis result: {0}")]
  RedisTypeError(redis::RedisError),
  #[error("error executing redis command: {0}")]
  RedisCMDError(redis::RedisError),
  #[error("error creating Redis client: {0}")]
  RedisClientError(redis::RedisError),
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("direct redis error: {0}")]
  RedisError(#[from] RedisError),
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
  message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
  let code;
  let message;

  if err.is_not_found() {
    code = StatusCode::NOT_FOUND;
    message = "Not Found";
  } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
    code = StatusCode::BAD_REQUEST;
    message = "Invalid Body";
  } else if let Some(e) = err.find::<Error>() {
    match e {
      Error::RedisError::RedisCMDError(_) => {
          code = StatusCode::BAD_REQUEST;
          message = "Could not Execute request";
      }
      _ => {
          eprintln!("Unhandled application error: {:?}", err);
          code = StatusCode::INTERNAL_SERVER_ERROR;
          message = "Internal Server Error";
      }
    }
  } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
    code = StatusCode::METHOD_NOT_ALLOWED;
    message = "Method Not Allowed";
  } else {
    eprintln!("unhandled error: {:?}", err);
    code = StatusCode::INTERNAL_SERVER_ERROR;
    message = "Internal Server Error";
  }

  let json = warp::reply::json(&ErrorResponse {
    message: message.into(),
  });

  Ok(warp::reply::with_status(json, code))
}
