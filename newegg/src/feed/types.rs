use crate::types::MaybeList;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestEnvelope<T> {
  #[serde(rename = "-xmlns:xsi")]
  pub xmlns_xsi: &'static str,
  #[serde(rename = "-xsi:noNamespaceSchemaLocation")]
  pub xsd_name: &'static str,
  #[serde(rename = "Header")]
  pub header: HashMap<String, String>,
  #[serde(rename = "MessageType")]
  pub message_type: String,
  #[serde(rename = "Message")]
  pub message: T,
  /// https://developer.newegg.com/newegg_marketplace_api/datafeed_management/submit_feed/inventory_and_price_feed/
  #[serde(rename = "Overwrite")]
  pub inventory_and_price_data_overwrite: Option<String>,
}

impl<T> RequestEnvelope<T> {
  pub fn new(
    xsd_name: &'static str,
    headers: &[(&str, &str)],
    message_type: &str,
    message: T,
  ) -> Self {
    RequestEnvelope {
      xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance",
      xsd_name,
      header: headers
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect(),
      message_type: message_type.to_string(),
      message,
      inventory_and_price_data_overwrite: None,
    }
  }

  pub fn inventory_and_price_data_overwrite(self, v: bool) -> Self {
    Self {
      inventory_and_price_data_overwrite: Some(if v {
        "True".to_string()
      } else {
        "False".to_string()
      }),
      ..self
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEnvelope<T> {
  #[serde(rename = "NeweggEnvelope")]
  pub inner: ResponseEnvelopeInner<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEnvelopeInner<T> {
  #[serde(rename = "Header")]
  pub header: HashMap<String, String>,
  #[serde(rename = "MessageType")]
  pub message_type: String,
  #[serde(rename = "Message")]
  pub message: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedResponse {
  #[serde(rename = "IsSuccess")]
  pub is_success: bool,
  #[serde(rename = "OperationType")]
  pub operation_type: String,
  #[serde(rename = "ResponseBody")]
  pub response_body: FeedResponseBody,
  #[serde(rename = "SellerID")]
  pub seller_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedResponseBody {
  #[serde(rename = "ResponseList")]
  pub response_list: Vec<FeedResponseList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedResponseList {
  #[serde(rename = "RequestDate")]
  pub request_date: String,
  #[serde(rename = "RequestId")]
  pub request_id: String,
  #[serde(rename = "RequestStatus")]
  pub request_status: String,
  #[serde(rename = "RequestType")]
  pub request_type: String,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestStatus {
  Submitted,
  InProgress,
  Finished,
  Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetRequestStatus {
  #[serde(rename = "RequestIDList")]
  pub request_id_list: RequestIdList,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "MaxCount")]
  pub max_count: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "RequestStatus")]
  pub request_status: Option<RequestStatus>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestIdList {
  #[serde(rename = "RequestID")]
  pub request_id: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingReportMessage {
  #[serde(rename = "ProcessingReport")]
  pub processing_report: ProcessingReport,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingReport {
  #[serde(rename = "OriginalMessageName")]
  pub original_message_name: String,
  #[serde(rename = "StatusCode")]
  pub status_code: String,
  #[serde(rename = "ProcessingSummary")]
  pub processing_summary: ProcessingSummary,
  #[serde(rename = "Result")]
  pub result: MaybeList<ProcessingResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingSummary {
  #[serde(rename = "ProcessedCount")]
  pub processed_count: String,
  #[serde(rename = "SuccessCount")]
  pub success_count: String,
  #[serde(rename = "WithErrorCount")]
  pub with_error_count: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingResult {
  #[serde(rename = "AdditionalInfo")]
  pub additional_info: AdditionalInfo,
  #[serde(rename = "ErrorList")]
  pub error_list: ErrorList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfo {
  #[serde(rename = "SubCategoryID")]
  pub sub_category_id: Option<String>,
  #[serde(rename = "SellerPartNumber")]
  pub seller_part_number: String,
  #[serde(rename = "ManufacturerPartNumberOrISBN")]
  pub manufacturer_part_number_or_isbn: Option<String>,
  #[serde(rename = "UPC")]
  pub upc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorList {
  #[serde(rename = "ErrorDescription")]
  pub error_description: MaybeList<String>,
}

#[test]
fn test_deserialize_processing_report_message() {
  let json = r#"{
    "ProcessingReport": {
      "OriginalMessageName": "Ventmere_APIAutoFile.xml",
      "OriginalMessageType": "Inventory",
      "ProcessedStamp": "2020/05/02 01:20:00",
      "ProcessingSummary": {
        "ProcessedCount": "2",
        "SuccessCount": "1",
        "WithErrorCount": "1"
      },
      "Result": {
        "AdditionalInfo": {
          "CountryCode": "CAN",
          "SellerPartNumber": "edifier-r1280t-fba"
        },
        "ErrorList": {
          "ErrorDescription": "CEI0001:Can't find this item in the system, please create it first"
        }
      },
      "StatusCode": "ProcessReport"
    }
  }"#;
  serde_json::from_str::<ProcessingReportMessage>(json).unwrap();
}
