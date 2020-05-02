use futures::compat::*;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::*;
use crate::result::NeweggFuture;

mod types;
pub use self::types::*;

pub mod message;

pub trait FeedApi {
  fn submit_feed<T>(
    &self,
    request_type: &str,
    request: &RequestEnvelope<T>,
  ) -> NeweggFuture<FeedResponse>
  where
    T: Serialize;
  fn get_feed_status(&self, request: &GetRequestStatus) -> NeweggFuture<FeedResponse>;
  fn get_feed_result<T>(&self, request_id: &str) -> NeweggFuture<ResponseEnvelope<T>>
  where
    T: for<'de> Deserialize<'de>;
}

impl FeedApi for NeweggClient {
  fn submit_feed<T>(
    &self,
    request_type: &str,
    request: &RequestEnvelope<T>,
  ) -> NeweggFuture<FeedResponse>
  where
    T: Serialize,
  {
    let send = self
      .request(Method::POST, "/datafeedmgmt/feeds/submitfeed")
      .query(&[("requesttype", request_type)])
      .json(&json!({ "NeweggEnvelope": request }))
      .send();
    async move { send.compat().await?.get_response().await }.boxed()
  }

  fn get_feed_status(&self, request: &GetRequestStatus) -> NeweggFuture<FeedResponse> {
    let send = self
      .request(Method::PUT, "/datafeedmgmt/feeds/status")
      .json(&json!({
        "OperationType": "GetFeedStatusRequest",
        "RequestBody": {
          "GetRequestStatus": request
        }
      }))
      .send();
    async move { send.compat().await?.get_response().await }.boxed()
  }
  fn get_feed_result<T>(&self, request_id: &str) -> NeweggFuture<ResponseEnvelope<T>>
  where
    T: for<'de> Deserialize<'de>,
  {
    let send = self
      .request(
        Method::GET,
        &format!("/datafeedmgmt/feeds/result/{}", request_id),
      )
      .send();
    async move { send.compat().await?.get_response().await }.boxed()
  }
}
