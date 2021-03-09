use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use error::ResponseError;

#[derive(Debug, Display, Error)]
#[display(fmt = "Error: {}", message)]
pub struct EchoIpError<'a> {
  message: &'a str,
}

impl EchoIpError<'_> {
  pub fn new(msg: &str) -> EchoIpError {
    EchoIpError{message: msg}
  }
}

impl ResponseError for EchoIpError<'_> {
  fn status_code(&self) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponseBuilder::new(self.status_code())
      .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
      .body(self.to_string())
  }
}
