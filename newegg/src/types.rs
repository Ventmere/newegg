use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NeweggApiResponse<B> {
  is_success: String,
  pub operation_type: String,
  #[serde(rename = "sellerID")]
  pub seller_id: Option<String>,
  pub response_body: B,
  pub memo: Option<String>,
}

impl<B> NeweggApiResponse<B> {
  pub fn get_is_success(&self) -> bool {
    self.is_success == "true"
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NeweggApiResponseWrapped<B> {
  #[serde(rename = "NeweggAPIResponse")]
  pub newegg_api_response: NeweggApiResponse<B>,
}

impl<B> std::ops::Deref for NeweggApiResponseWrapped<B> {
  type Target = NeweggApiResponse<B>;

  fn deref(&self) -> &Self::Target {
    &self.newegg_api_response
  }
}
