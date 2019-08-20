use futures::compat::*;
use futures::FutureExt;
use serde::Serialize;

use crate::client::*;
use crate::result::{NeweggFuture};

mod types;

pub use self::types::*;

pub trait ReportApi {
  fn submit_report_request<R>(&self, request: &ReportRequest<R>) -> NeweggFuture<ReportResponse> where R: Serialize;
  fn get_report_status(&self, request_ids: &[&str], max_count: Option<u64>) -> NeweggFuture<ReportResponse>;
  fn get_report_result(
    &self,
    request_id: &str,
    page_index: u64,
    page_size: Option<u64>,
  ) -> NeweggFuture<ReportResultReponse>;
}

impl ReportApi for NeweggClient {
  fn submit_report_request<R>(&self, request: &ReportRequest<R>) -> NeweggFuture<ReportResponse> 
  where R: Serialize
  {
    let send = self
        .request(Method::POST, "/reportmgmt/report/submitrequest")
        .json(request)
        .send();
    async move {
      send.compat().await?
        .get_response().await
    }.boxed()
  }

  fn get_report_status(&self, request_ids: &[&str], max_count: Option<u64>) -> NeweggFuture<ReportResponse> {
    let send = self
        .request(Method::PUT, "/reportmgmt/report/status")
        .json(&
          GetReportStatusRequest::new(request_ids, max_count.unwrap_or(100))
        )
        .send();
    async move {
      send.compat().await?
        .get_response().await
    }.boxed()
  }

  fn get_report_result(
    &self,
    request_id: &str,
    page_index: u64,
    page_size: Option<u64>,
  ) -> NeweggFuture<ReportResultReponse> {
    let send = self
      .request(Method::PUT, "/reportmgmt/report/result")
      .json(&
        GetReportResultRequest::new(request_id, page_index, page_size.unwrap_or(100))
      )
      .send();
    async move {
      send.compat().await?
        .get_response().await
    }.boxed()
  }
}
