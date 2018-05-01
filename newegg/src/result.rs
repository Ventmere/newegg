use order::{CancelOrderResponse, ShipOrderResponse};
use reqwest::StatusCode;

error_chain! {
  errors {
    Request(path: String, status: StatusCode, body: String) {
      description("request error")
      display("request error: path = '{}', status = '{}', body = '{}'", path, status, body)
    }
    Deserialize(msg: String, body: String) {
      description("deserialize body error")
      display("deserialize body error: {}, body = '{}'", msg, body)
    }
    CancelOrderNotSuccess(res: CancelOrderResponse) {
      description("cancel order not success")
      display("cancel order not success: {:?}", res)
    }
    ShipOrderNotSuccess(res: ShipOrderResponse) {
      description("ship order not success")
      display("ship order not success: {:?}", res)
    }
  }

  foreign_links {
    Http(::reqwest::Error);
    Json(::serde_json::Error);
  }
}

pub type NeweggResult<T> = ::std::result::Result<T, Error>;
