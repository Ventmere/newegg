use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NeweggApiResponse<B> {
  pub is_success: bool,
  pub operation_type: String,
  pub seller_id: String,
  pub response_body: B,
}
