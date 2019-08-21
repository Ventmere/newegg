use futures::compat::*;
use futures::FutureExt;

use crate::client::*;
use crate::result::{NeweggError, NeweggFuture};

mod types;

pub use self::types::*;

pub trait OrderApi {
  fn get_order_info(&self, request: &GetOrderInfoRequest) -> NeweggFuture<GetOrderInfoResponse>;
  fn cancel_order(
    &self,
    order_number: i64,
    reason: CancelOrderReasonCode,
  ) -> NeweggFuture<CancelOrderResponse>;
  fn ship_order(
    &self,
    order_number: i64,
    action: &ShipOrderAction,
  ) -> NeweggFuture<ShipOrderResponse>;
}

impl OrderApi for NeweggClient {
  /// Note for USA marketplace, the default Status is '0'(Unshipped)
  fn get_order_info(&self, request: &GetOrderInfoRequest) -> NeweggFuture<GetOrderInfoResponse> {
    let send = self
      .request(Method::PUT, "/ordermgmt/order/orderinfo")
      .json(&request)
      .send();
    async move { send.compat().await?.get_response().await }.boxed()
  }

  fn cancel_order(
    &self,
    order_number: i64,
    reason: CancelOrderReasonCode,
  ) -> NeweggFuture<CancelOrderResponse> {
    let send = self
      .request(
        Method::PUT,
        &format!("/ordermgmt/orderstatus/orders/{}", order_number),
      )
      .json(&CancelOrderAction::new(reason))
      .send();
    async move {
      let res: CancelOrderResponse = send.compat().await?.get_response().await?;
      if res.is_success() {
        Ok(res)
      } else {
        Err(NeweggError::CancelOrderNotSuccess(res).into())
      }
    }
      .boxed()
  }

  fn ship_order(
    &self,
    order_number: i64,
    action: &ShipOrderAction,
  ) -> NeweggFuture<ShipOrderResponse> {
    let send = self
      .request(
        Method::PUT,
        &format!("/ordermgmt/orderstatus/orders/{}", order_number),
      )
      .json(action)
      .send();

    async move {
      let res: ShipOrderResponse = send.compat().await?.get_response().await?;
      if res.is_success() {
        Ok(res)
      } else {
        Err(NeweggError::ShipOrderNotSuccess(res).into())
      }
    }
      .boxed()
  }
}
