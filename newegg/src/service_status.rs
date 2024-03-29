use serde_derive::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::client::*;
use crate::result::NeweggResult;

pub enum ServiceStatusDomain {
  Content,
  Order,
  DateFeed,
  Service,
  Report,
  Seller,
  Sbn,
  ShippingLabel,
}

impl ServiceStatusDomain {
  pub fn as_str(&self) -> &'static str {
    match *self {
      ServiceStatusDomain::Content => "contentmgmt",
      ServiceStatusDomain::Order => "ordermgmt",
      ServiceStatusDomain::DateFeed => "datafeedmgmt",
      ServiceStatusDomain::Service => "servicemgmt",
      ServiceStatusDomain::Report => "reportmgmt",
      ServiceStatusDomain::Seller => "sellermgmt",
      ServiceStatusDomain::Sbn => "sbnmgmt",
      ServiceStatusDomain::ShippingLabel => "shippinglabelmgmt",
    }
  }

  pub fn from_str(v: &str) -> Option<Self> {
    match v {
      "contentmgmt" => Some(ServiceStatusDomain::Content),
      "ordermgmt" => Some(ServiceStatusDomain::Order),
      "datafeedmgmt" => Some(ServiceStatusDomain::DateFeed),
      "servicemgmt" => Some(ServiceStatusDomain::Service),
      "reportmgmt" => Some(ServiceStatusDomain::Report),
      "sellermgmt" => Some(ServiceStatusDomain::Seller),
      "sbnmgmt" => Some(ServiceStatusDomain::Sbn),
      "shippinglabelmgmt" => Some(ServiceStatusDomain::ShippingLabel),
      _ => None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetServiceStatusResponse {
  #[serde(rename = "NeweggAPIResponse")]
  newegg_api_response: NeweggApiResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeweggApiResponse {
  #[serde(rename = "IsSuccess")]
  is_success: String,
  #[serde(rename = "OperationType")]
  operation_type: String,
  #[serde(rename = "SellerID")]
  seller_id: String,
  #[serde(rename = "ResponseBody")]
  response_body: ResponseBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
  #[serde(rename = "Status")]
  status: String,
  #[serde(rename = "Timestamp")]
  timestamp: String,
}

#[async_trait]
pub trait ServiceStatusApi {
  async fn get_service_status(
    &self,
    domain: ServiceStatusDomain,
  ) -> NeweggResult<GetServiceStatusResponse>;
}

#[async_trait]
impl ServiceStatusApi for NeweggClient {
  async fn get_service_status(
    &self,
    domain: ServiceStatusDomain,
  ) -> NeweggResult<GetServiceStatusResponse> {
    self
      .request(Method::GET, &format!("/{}/servicestatus", domain.as_str()))
      .send()
      .await?
      .get_response()
      .await
  }
}
