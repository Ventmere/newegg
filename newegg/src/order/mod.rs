use crate::client::*;
use crate::result::{NeweggError, NeweggResult};
use async_trait::async_trait;

mod types;

pub use self::types::*;

#[async_trait]
pub trait OrderApi {
  async fn get_order_info(&self, request: &GetOrderInfoRequest) -> NeweggResult<GetOrderInfoResponse>;
  async fn cancel_order(
    &self,
    order_number: i64,
    reason: CancelOrderReasonCode,
  ) -> NeweggResult<CancelOrderResponse>;
  async fn ship_order(
    &self,
    order_number: i64,
    action: &ShipOrderAction,
  ) -> NeweggResult<ShipOrderResponse>;
}

#[async_trait]
impl OrderApi for NeweggClient {
  /// Note for USA marketplace, the default Status is '0'(Unshipped)
  async fn get_order_info(&self, request: &GetOrderInfoRequest) -> NeweggResult<GetOrderInfoResponse> {
    self
      .request(Method::PUT, "/ordermgmt/order/orderinfo")
      .json(&request)
      .send()
      .await?
      .get_response()
      .await
  }

  async fn cancel_order(
    &self,
    order_number: i64,
    reason: CancelOrderReasonCode,
  ) -> NeweggResult<CancelOrderResponse> {
    let res: CancelOrderResponse = self
      .request(
        Method::PUT,
        &format!("/ordermgmt/orderstatus/orders/{}", order_number),
      )
      .json(&CancelOrderAction::new(reason))
      .send()
      .await?
      .get_response()
      .await?;
    if res.is_success() {
      Ok(res)
    } else {
      Err(NeweggError::CancelOrderNotSuccess(res).into())
    }
  }

  async fn ship_order(
    &self,
    order_number: i64,
    action: &ShipOrderAction,
  ) -> NeweggResult<ShipOrderResponse> {
    let res: ShipOrderResponse = self
      .request(
        Method::PUT,
        &format!("/ordermgmt/orderstatus/orders/{}", order_number),
      )
      .json(action)
      .send()
      .await?
      .get_response()
      .await?;
    if res.is_success() {
      Ok(res)
    } else {
      Err(NeweggError::ShipOrderNotSuccess(res).into())
    }
  }
}
