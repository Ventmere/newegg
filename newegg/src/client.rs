use reqwest::{Client, Response, StatusCode};
pub use reqwest::{Method, RequestBuilder};
use result::{NeweggError, NeweggResult};
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

  /// The country for your orders. Only the ISO standard 3-digit codes are accepted.
  /// To review the complete list of available values, please download the following:
  ///
  /// https://promotions.newegg.com/Marketplace/Sellers/resourceLibrary/International%20Country%20Guide.pdf
  pub fn country_code(&self) -> &str {
    match *self {
      NeweggMarketplace::Usa => "USA",
      NeweggMarketplace::Canada => "CAN",
    }
  }
}

pub struct NeweggClient {
  http: Client,
  seller_id_: String,
  token: String,
  secret_key: String,
  marketplace_: NeweggMarketplace,
}

impl NeweggClient {
  pub fn new(
    marketplace: NeweggMarketplace,
    seller_id: &str,
    token: &str,
    secret_key: &str,
  ) -> NeweggResult<Self> {
    let client = Client::builder()
      .timeout(::std::time::Duration::from_secs(300))
      .build()?;
    Ok(Self::with_http_client(
      marketplace,
      seller_id,
      token,
      secret_key,
      client,
    ))
  }

  /// Please make sure your HTTP client has very long timeout
  /// because some APIs are very slow
  pub fn with_http_client(
    marketplace: NeweggMarketplace,
    seller_id: &str,
    token: &str,
    secret_key: &str,
    http: Client,
  ) -> Self {
    Self {
      seller_id_: seller_id.to_owned(),
      token: token.to_owned(),
      secret_key: secret_key.to_owned(),
      marketplace_: marketplace,
      http,
    }
  }

  pub fn seller_id(&self) -> &str {
    self.seller_id_.as_ref()
  }

  pub fn marketplace(&self) -> NeweggMarketplace {
    self.marketplace_
  }

  pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
    use reqwest::{
      header::{qitem, Accept, Authorization, Headers},
      mime,
    };
    let mut b = self
      .http
      .request(method, &format!("{}{}", self.marketplace_.base_url(), path));

    b.query(&[("sellerid", &self.seller_id_ as &str)]);

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

const BOM: char = '\u{feff}';

impl NeweggResponse for Response {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> NeweggResult<T> {
    let body = self.text()?;

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

    if self.status() != StatusCode::Ok {
      return Err(NeweggError::Request {
        path: self.url().to_string(),
        status: self.status(),
        body: body_str.to_string(),
      });
    }

    match serde_json::from_str(body_str) {
      Ok(v) => Ok(v),
      Err(err) => {
        return Err(NeweggError::Deserialize {
          msg: err.to_string(),
          body: body_str.to_string(),
        })
      }
    }
  }
}
