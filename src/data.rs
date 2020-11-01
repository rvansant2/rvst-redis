use serde::{Serialize, Deserialize};

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
