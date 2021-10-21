use serde::Serialize;
use url::Url;

use crate::client::*;
use crate::helpers::block;
use crate::result::{NeweggError, NeweggResult};
use async_trait::async_trait;

mod types;

pub use self::types::*;

#[async_trait]
pub trait ReportApi {
  async fn submit_report_request<R>(&self, request: ReportRequest<R>) -> NeweggResult<ReportResponse>
  where
    R: Serialize + Send + Sync;
  async fn get_report_status(
    &self,
    request_ids: &[&str],
    max_count: Option<u64>,
  ) -> NeweggResult<ReportResponse>;
  async fn get_report_result(
    &self,
    operation_type: &str,
    request_id: &str,
    page_index: u64,
    page_size: Option<u64>,
  ) -> NeweggResult<ReportResultReponse>;
  async fn get_report_file(&self, url: &str) -> NeweggResult<Vec<u8>>;
}

#[async_trait]
impl ReportApi for NeweggClient {
  async fn submit_report_request<R>(&self, request: ReportRequest<R>) -> NeweggResult<ReportResponse>
  where
    R: Serialize + Send + Sync,
  {
    self
      .request(Method::POST, "/reportmgmt/report/submitrequest")
      .json(&request)
      .send()
      .await?
      .get_response()
      .await
  }

  async fn get_report_status(
    &self,
    request_ids: &[&str],
    max_count: Option<u64>,
  ) -> NeweggResult<ReportResponse> {
    self
      .request(Method::PUT, "/reportmgmt/report/status")
      .json(&ReportRequest::new(
        "GetReportStatusRequest",
        GetReportStatusRequest::new(request_ids, max_count.unwrap_or(100)),
      ))
      .send()
      .await?
      .get_response()
      .await
  }

  async fn get_report_result(
    &self,
    operation_type: &str,
    request_id: &str,
    page_index: u64,
    page_size: Option<u64>,
  ) -> NeweggResult<ReportResultReponse> {
    self
      .request(Method::PUT, "/reportmgmt/report/result")
      .json(&ReportRequest::new(
        operation_type,
        GetReportResultRequest::new(request_id, page_index, page_size.unwrap_or(100)),
      ))
      .send()
      .await?
      .get_response()
      .await
  }

  async fn get_report_file(&self, url: &str) -> NeweggResult<Vec<u8>> {
    use ftp::FtpStream;
    use std::io::Read;
    let url = url.to_owned();
    block(move || {
      let url = Url::parse(&url)?;
      let host = url
        .host_str()
        .ok_or_else(|| NeweggError::FtpUrl(format!("no host")))?;
      let port = url
        .port_or_known_default()
        .ok_or_else(|| NeweggError::FtpUrl(format!("no port")))?;
      let mut stream = FtpStream::connect(&format!("{}:{}", host, port))?;
      stream.login(
        url.username(),
        url
          .password()
          .ok_or_else(|| NeweggError::FtpUrl(format!("no password")))?,
      )?;
      let path_segments: Vec<_> = url
        .path_segments()
        .ok_or_else(|| NeweggError::FtpUrl(format!("no path")))?
        .collect();
      let mut r = if path_segments.len() > 1 {
        let path: String = path_segments[..(path_segments.len() - 1)].join("");
        stream.cwd(&path)?;
        stream.get(path_segments[path_segments.len() - 1])?
      } else {
        stream.get(url.path())?
      };
      let mut data = vec![];
      r.read_to_end(&mut data)?;
      Ok(data)
    }).await
  }
}
