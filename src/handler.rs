use crate::{data::* redislib::*, WebResult, Result};
use warp::{http::StatusCode, Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};

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
