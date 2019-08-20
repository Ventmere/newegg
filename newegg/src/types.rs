use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NeweggApiResponse<B> {
  pub is_success: bool,
  pub operation_type: String,
  pub seller_id: Option<String>,
  pub response_body: B,
  pub memo: Option<String>,
}
