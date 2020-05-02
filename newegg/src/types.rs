use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NeweggApiResponse<B> {
  is_success: IsSuccess,
  pub operation_type: String,
  #[serde(rename = "sellerID")]
  pub seller_id: Option<String>,
  pub response_body: B,
  pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum IsSuccess {
  StringValue(String),
  BoolValue(bool),
}

impl<B> NeweggApiResponse<B> {
  pub fn get_is_success(&self) -> bool {
    match &self.is_success {
      &IsSuccess::StringValue(ref v) => v == "true",
      &IsSuccess::BoolValue(v) => v,
    }
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeList<T> {
  Single(T),
  List(Vec<T>),
}
