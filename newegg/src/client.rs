use crate::result::{NeweggError, NeweggFuture, NeweggResult};
use futures::compat::*;
use futures::FutureExt;
use reqwest::header::HeaderValue;
pub use reqwest::r#async::RequestBuilder;
use reqwest::r#async::{Client, Response};
pub use reqwest::Method;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Clone, Copy)]
pub enum NeweggPlatform {
  Newegg,
  NeweggBusiness,
  NeweggCanada,
}

impl NeweggPlatform {
  pub fn base_url(&self) -> &str {
    match *self {
      NeweggPlatform::Newegg => "https://api.newegg.com/marketplace",
      NeweggPlatform::NeweggBusiness => "https://api.newegg.com/marketplace/b2b",
      NeweggPlatform::NeweggCanada => "https://api.newegg.com/marketplace/can",
    }
  }
}

pub struct NeweggClient {
  http: Client,
  seller_id_: String,
  token: HeaderValue,
  secret_key: HeaderValue,
  platform: NeweggPlatform,
}

impl NeweggClient {
  pub fn new(
    platform: NeweggPlatform,
    seller_id: &str,
    token: &str,
    secret_key: &str,
  ) -> NeweggResult<Self> {
    let client = Client::builder()
      .timeout(::std::time::Duration::from_secs(300))
      .build()?;
    Self::with_http_client(platform, seller_id, token, secret_key, client)
  }

  /// Please make sure your HTTP client has very long timeout
  /// because some APIs are very slow
  pub fn with_http_client(
    platform: NeweggPlatform,
    seller_id: &str,
    token: &str,
    secret_key: &str,
    http: Client,
  ) -> NeweggResult<Self> {
    Ok(Self {
      seller_id_: seller_id.to_owned(),
      token: HeaderValue::from_str(token)
        .map_err(|_| NeweggError::InvalidHeader("Authorization"))?,
      secret_key: HeaderValue::from_str(secret_key)
        .map_err(|_| NeweggError::InvalidHeader("SecretKey"))?,
      platform,
      http,
    })
  }

  pub fn seller_id(&self) -> &str {
    self.seller_id_.as_ref()
  }

  pub fn get_platform(&self) -> NeweggPlatform {
    self.platform
  }

  pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
    use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
    let mut b = self
      .http
      .request(method, &format!("{}{}", self.platform.base_url(), path));

    b = b.query(&[("sellerid", &self.seller_id_ as &str)]);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, self.token.clone());
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("SecretKey", self.secret_key.clone());
    b.headers(headers)
  }
}

pub trait NeweggResponse {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> NeweggFuture<T>;
}

const BOM: char = '\u{feff}';

impl NeweggResponse for Response {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> NeweggFuture<T> {
    let status = self.status().clone();
    let url = self.url().to_string();
    let text = self.text();
    async move {
      let body = text.compat().await?;
      let body_str: &str = if let Some(c) = body.chars().next() {
        // strip BOM
        if c == BOM {
          &body[BOM.len_utf8()..]
        } else {
          body.as_ref()
        }
      } else {
        ""
      };

      if status != StatusCode::OK {
        Err(NeweggError::Request {
          path: url,
          status,
          body: body_str.to_string(),
        })
      } else {
        match serde_json::from_str(body_str) {
          Ok(v) => Ok(v),
          Err(err) => Err(NeweggError::Deserialize {
            msg: err.to_string(),
            body: body_str.to_string(),
          }),
        }
      }
    }
      .boxed()
  }
}
