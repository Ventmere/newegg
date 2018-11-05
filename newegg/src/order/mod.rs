use client::*;
use result::{NeweggError, NeweggResult};

mod types;

pub use self::types::*;

pub trait OrderApi {
  fn get_order_info(&self, request: &GetOrderInfoRequest) -> NeweggResult<GetOrderInfoResponse>;
  fn cancel_order(
    &self,
    order_number: i64,
    reason: CancelOrderReasonCode,
  ) -> NeweggResult<CancelOrderResponse>;
  fn ship_order(
    &self,
    order_number: i64,
    action: &ShipOrderAction,
  ) -> NeweggResult<ShipOrderResponse>;
}

impl OrderApi for NeweggClient {
  /// Note for USA marketplace, the default Status is '0'(Unshipped)
  fn get_order_info(&self, request: &GetOrderInfoRequest) -> NeweggResult<GetOrderInfoResponse> {
    self
      .request(Method::Put, "/ordermgmt/order/orderinfo")
      .json(request)
      .send()?
      .get_response()
  }

  fn cancel_order(
    &self,
    order_number: i64,
    reason: CancelOrderReasonCode,
  ) -> NeweggResult<CancelOrderResponse> {
    let res: CancelOrderResponse = self
      .request(
        Method::Put,
        &format!("/ordermgmt/orderstatus/orders/{}", order_number),
      )
      .json(&CancelOrderAction::new(reason))
      .send()?
      .get_response()?;
    if res.is_success() {
      Ok(res)
    } else {
      Err(NeweggError::CancelOrderNotSuccess(res).into())
    }
  }

  fn ship_order(
    &self,
    order_number: i64,
    action: &ShipOrderAction,
  ) -> NeweggResult<ShipOrderResponse> {
    let res: ShipOrderResponse = self
      .request(
        Method::Put,
        &format!("/ordermgmt/orderstatus/orders/{}", order_number),
      )
      .json(action)
      .send()?
      .get_response()?;
    if res.is_success() {
      Ok(res)
    } else {
      Err(NeweggError::ShipOrderNotSuccess(res).into())
    }
  }
}
