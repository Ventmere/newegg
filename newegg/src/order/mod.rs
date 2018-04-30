use result::NeweggResult;

mod types;

pub use self::types::*;

pub trait OrderApi {
  fn get_order_info(&self, request: &GetOrderInfoRequest) -> NeweggResult<GetOrderInfoResponse>;
}
