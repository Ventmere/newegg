use futures::compat::*;
use futures::FutureExt;
use serde::Serialize;
use url::Url;

use crate::client::*;
use crate::helpers::block;
use crate::result::{NeweggError, NeweggFuture};

mod types;

pub use self::types::*;

pub trait ReportApi {
  fn submit_report_request<R>(&self, request: &ReportRequest<R>) -> NeweggFuture<ReportResponse>
  where
    R: Serialize;
  fn get_report_status(
    &self,
    request_ids: &[&str],
    max_count: Option<u64>,
  ) -> NeweggFuture<ReportResponse>;
  fn get_report_result(
    &self,
    operation_type: &str,
    request_id: &str,
    page_index: u64,
    page_size: Option<u64>,
  ) -> NeweggFuture<ReportResultReponse>;
  fn get_report_file(&self, url: &str) -> NeweggFuture<Vec<u8>>;
}

impl ReportApi for NeweggClient {
  fn submit_report_request<R>(&self, request: &ReportRequest<R>) -> NeweggFuture<ReportResponse>
  where
    R: Serialize,
  {
    let send = self
      .request(Method::POST, "/reportmgmt/report/submitrequest")
      .json(request)
      .send();
    async move { send.compat().await?.get_response().await }.boxed()
  }

  fn get_report_status(
    &self,
    request_ids: &[&str],
    max_count: Option<u64>,
  ) -> NeweggFuture<ReportResponse> {
    let send = self
      .request(Method::PUT, "/reportmgmt/report/status")
      .json(&ReportRequest::new(
        "GetReportStatusRequest",
        GetReportStatusRequest::new(request_ids, max_count.unwrap_or(100)),
      ))
      .send();
    async move { send.compat().await?.get_response().await }.boxed()
  }

  fn get_report_result(
    &self,
    operation_type: &str,
    request_id: &str,
    page_index: u64,
    page_size: Option<u64>,
  ) -> NeweggFuture<ReportResultReponse> {
    let send = self
      .request(Method::PUT, "/reportmgmt/report/result")
      .json(&ReportRequest::new(
        operation_type,
        GetReportResultRequest::new(request_id, page_index, page_size.unwrap_or(100)),
      ))
      .send();
    async move { send.compat().await?.get_response().await }.boxed()
  }

  fn get_report_file(&self, url: &str) -> NeweggFuture<Vec<u8>> {
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
    })
    .boxed()
  }
}
