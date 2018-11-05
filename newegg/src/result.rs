use order::{CancelOrderResponse, ShipOrderResponse};
use reqwest::StatusCode;

#[derive(Fail, Debug)]
pub enum NeweggError {
  #[fail(
    display = "request error: path = '{}', status = '{}', body = '{}'",
    path, status, body
  )]
  Request {
    path: String,
    status: StatusCode,
    body: String,
  },

  #[fail(display = "deserialize body error: msg = '{}', body = '{}'", msg, body)]
  Deserialize { msg: String, body: String },

  #[fail(display = "cancel order not success: {:?}", _0)]
  CancelOrderNotSuccess(CancelOrderResponse),

  #[fail(display = "ship order not success: {:?}", _0)]
  ShipOrderNotSuccess(ShipOrderResponse),

  #[fail(display = "http error: {}", _0)]
  Http(::reqwest::Error),

  #[fail(display = "json error: {}", _0)]
  Json(::serde_json::Error),
}

pub type NeweggResult<T> = ::std::result::Result<T, NeweggError>;

macro_rules! impl_from {
  ($v:ident($t:ty)) => {
    impl From<$t> for NeweggError {
      fn from(e: $t) -> Self {
        NeweggError::$v(e)
      }
    }
  };
}

impl_from!(Http(::reqwest::Error));
impl_from!(Json(::serde_json::Error));
