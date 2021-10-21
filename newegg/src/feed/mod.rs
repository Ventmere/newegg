use serde::{Deserialize, Serialize};
use serde_json::json;
use async_trait::async_trait;
use crate::result::NeweggResult;

use crate::client::*;

mod types;
pub use self::types::*;

pub mod message;

#[async_trait]
pub trait FeedApi {
  async fn submit_feed<T>(
    &self,
    request_type: &str,
    request: RequestEnvelope<T>,
  ) -> NeweggResult<FeedResponse>
  where
    T: Serialize + Send;
  async fn get_feed_status(&self, request: &GetRequestStatus) -> NeweggResult<FeedResponse>;
  async fn get_feed_result<T>(&self, request_id: &str) -> NeweggResult<ResponseEnvelope<T>>
  where
    T: for<'de> Deserialize<'de> + Send;
}

#[async_trait]
impl FeedApi for NeweggClient {
  async fn submit_feed<T>(
    &self,
    request_type: &str,
    request: RequestEnvelope<T>,
  ) -> NeweggResult<FeedResponse>
  where
    T: Serialize + Send,
  {
    self
      .request(Method::POST, "/datafeedmgmt/feeds/submitfeed")
      .query(&[("requesttype", request_type.to_owned())])
      .json(&json!({ "NeweggEnvelope": request }))
      .send()
      .await?
      .get_response()
      .await
  }

  async fn get_feed_status(&self, request: &GetRequestStatus) -> NeweggResult<FeedResponse> {
    self
      .request(Method::PUT, "/datafeedmgmt/feeds/status")
      .json(&json!({
        "OperationType": "GetFeedStatusRequest",
        "RequestBody": {
          "GetRequestStatus": request
        }
      }))
      .send()
      .await?
      .get_response()
      .await
  }

  async fn get_feed_result<T>(&self, request_id: &str) -> NeweggResult<ResponseEnvelope<T>>
  where
    T: for<'de> Deserialize<'de> + Send,
  {
    self
      .request(
        Method::GET,
        &format!("/datafeedmgmt/feeds/result/{}", request_id),
      )
      .send()
      .await?
      .get_response()
      .await
  }
}
