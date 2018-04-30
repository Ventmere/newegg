use reqwest::{Client, Response, StatusCode};
pub use reqwest::{Method, RequestBuilder};
use result::{ErrorKind, NeweggResult};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Clone, Copy)]
pub enum NeweggMarketplace {
  Usa,
  Canada,
}

impl NeweggMarketplace {
  pub fn base_url(&self) -> &str {
    match *self {
      NeweggMarketplace::Usa => "https://api.newegg.com/marketplace",
      NeweggMarketplace::Canada => "https://api.newegg.com/marketplace/can",
    }
  }
}

pub struct NeweggClient {
  http: Client,
  pub seller_id: String,
  token: String,
  secret_key: String,
  marketplace: NeweggMarketplace,
}

impl NeweggClient {
  pub fn new(
    marketplace: NeweggMarketplace,
    seller_id: &str,
    token: &str,
    secret_key: &str,
  ) -> Self {
    Self::with_http_client(marketplace, seller_id, token, secret_key, Client::new())
  }

  pub fn with_http_client(
    marketplace: NeweggMarketplace,
    seller_id: &str,
    token: &str,
    secret_key: &str,
    http: Client,
  ) -> Self {
    Self {
      seller_id: seller_id.to_owned(),
      token: token.to_owned(),
      secret_key: secret_key.to_owned(),
      marketplace,
      http,
    }
  }

  pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
    use reqwest::{header::{qitem, Accept, Authorization, Headers},
                  mime};
    let mut b = self
      .http
      .request(method, &format!("{}{}", self.marketplace.base_url(), path));
    let mut headers = Headers::new();
    headers.set(Authorization(self.token.clone()));
    headers.set(Accept(vec![qitem(mime::APPLICATION_JSON)]));
    headers.set_raw("SecretKey", &self.secret_key as &str);
    b.headers(headers);
    b
  }
}

pub trait NeweggResponse {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> NeweggResult<T>;
}

impl NeweggResponse for Response {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> NeweggResult<T> {
    let body = self.text()?;

    if self.status() != StatusCode::Ok {
      return Err(ErrorKind::Request(self.url().to_string(), self.status(), body).into());
    }

    match serde_json::from_str(&body) {
      Ok(v) => Ok(v),
      Err(err) => return Err(ErrorKind::Deserialize(err.to_string(), body).into()),
    }
  }
}
