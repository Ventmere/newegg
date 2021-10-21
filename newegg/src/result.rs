use thiserror::Error;
use reqwest::StatusCode;

use crate::order::{CancelOrderResponse, ShipOrderResponse};

#[derive(Error, Debug)]
pub enum NeweggError {
  #[error(
    "request error: path = '{path}', status = '{status}', body = '{body}'"
  )]
  Request {
    path: String,
    status: StatusCode,
    body: String,
  },

  #[error("deserialize body error: msg = '{msg}', body = '{body}'")]
  Deserialize { msg: String, body: String },

  #[error("cancel order not success: {0:?}")]
  CancelOrderNotSuccess(CancelOrderResponse),

  #[error("ship order not success: {0:?}")]
  ShipOrderNotSuccess(ShipOrderResponse),

  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),

  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("invalid header: {0}")]
  InvalidHeader(&'static str),

  #[error("parse url error: {0}")]
  Url(#[from] url::ParseError),

  #[error("ftp error: {0}")]
  Ftp(#[from] ftp::types::FtpError),

  #[error("ftp url error: {0}")]
  FtpUrl(String),

  #[error("id error: {0}")]
  Io(#[from] std::io::Error),

  #[error("runtime: {0}")]
  Runtime(#[from] tokio::task::JoinError),
}

impl NeweggError {
  pub fn should_try_again(&self) -> bool {
    match *self {
      NeweggError::Request { status, .. } => {
        let code = status.as_u16();
        // 429 Too Many Requests
        code == 429 || code == 500 || code == 503
      }
      _ => false,
    }
  }
}

pub type NeweggResult<T> = ::std::result::Result<T, NeweggError>;