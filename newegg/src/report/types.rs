use serde::Serialize;
use serde_derive::Deserialize;

use crate::helpers::NeweggDateTime;
use crate::types::NeweggApiResponse;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportRequest<B> {
  pub operation_type: &'static str,
  pub request_body: B,
}

impl<B> ReportRequest<B>
where
  B: Serialize,
{
  pub fn new(operation_type: &'static str, body: B) -> Self {
    ReportRequest {
      operation_type,
      request_body: body,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestStatus {
  Submitted,
  InProgress,
  Finished,
  Cancelled,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseInfo {
  pub request_id: bool,
  pub request_type: String,
  pub request_date: NeweggDateTime,
  pub request_status: RequestStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseBody {
  pub response_info: Vec<ResponseInfo>,
}

pub type ReportResponse = NeweggApiResponse<ResponseBody>;
