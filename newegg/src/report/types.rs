use serde::Serialize;
use serde_derive::Deserialize;

use crate::helpers::NeweggDateTime;
use crate::types::{NeweggApiResponse, NeweggApiResponseWrapped};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportRequest<B> {
  pub operation_type: String,
  pub request_body: B,
}

impl<B> ReportRequest<B>
where
  B: Serialize,
{
  pub fn new(operation_type: &str, body: B) -> Self {
    ReportRequest {
      operation_type: operation_type.to_string(),
      request_body: body,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetReportStatusRequest {
  get_request_status: GetRequestStatus,
}

impl GetReportStatusRequest {
  pub fn new(ids: &[&str], max_count: u64) -> Self {
    GetReportStatusRequest {
      get_request_status: GetRequestStatus {
        request_id_list: RequestIDList {
          request_id: ids.iter().map(|id| id.to_string()).collect(),
        },
        max_count: max_count.to_string(),
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GetRequestStatus {
  #[serde(rename = "RequestIDList")]
  request_id_list: RequestIDList,
  max_count: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestIDList {
  #[serde(rename = "RequestID")]
  request_id: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestStatus {
  Submitted,
  InProgress,
  Finished,
  Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetReportResultRequest {
  #[serde(rename = "RequestID")]
  pub request_id: String,
  pub page_info: PageInfo,
}

impl GetReportResultRequest {
  pub fn new(request_id: &str, page_index: u64, page_size: u64) -> Self {
    GetReportResultRequest {
      request_id: request_id.to_string(),
      page_info: PageInfo {
        page_size: page_size.to_string(),
        page_index: page_index.to_string(),
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PageInfo {
  pub page_size: String,
  pub page_index: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseInfo {
  #[serde(rename(deserialize = "RequestID", deserialize = "RequestId"))]
  pub request_id: String,
  pub request_type: String,
  pub request_date: NeweggDateTime,
  pub request_status: Option<RequestStatus>,
  pub total_count: Option<i64>,
  #[serde(rename = "ReportFileURL")]
  pub report_file_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseBody {
  pub response_list: Vec<ResponseInfo>,
}

pub type ReportResponse = NeweggApiResponse<ResponseBody>;
pub type ReportResultReponse = NeweggApiResponseWrapped<ResponseInfo>;
