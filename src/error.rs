use derive_more::Display;
use actix_web::{ResponseError, HttpResponse};
use maxminddb::MaxMindDBError;
use thiserror::Error;
use std::net::AddrParseError;

#[derive(Debug, Display, Error)]
pub enum EchoIpError {
  #[display(fmt = "IP address could not be resolved")]
  RemoteIpNotAvailable,

  #[display(fmt = "IP address could not be resolved")]
  IpAddressResolutionFailed { source: AddrParseError },

  #[display(fmt = "Could not compile Handlebars template")]
  HandlebarsFailed,

  #[display(fmt = "Could not get IP information from Max Mind DB")]
  MaxMindDbFailed { source: MaxMindDBError },
}

/// Actix web uses `ResponseError` for conversion of errors to a response
impl ResponseError for EchoIpError {
  fn error_response(&self) -> HttpResponse {
    match self {
      EchoIpError::RemoteIpNotAvailable => {
        eprintln!("Remote IP address not available");
        HttpResponse::InternalServerError().finish()
      }

      EchoIpError::IpAddressResolutionFailed{source} => {
        eprintln!("IP Address resolution failed: {}", source.to_string());
        HttpResponse::InternalServerError().finish()
      }

      EchoIpError::HandlebarsFailed => {
        eprintln!("Handlebars template compilation failed");
        HttpResponse::InternalServerError().finish()
      }

      EchoIpError::MaxMindDbFailed{source} => {
        eprintln!("IP to location resolution failed: {}", source.to_string());
        HttpResponse::InternalServerError().finish()
      }
    }
  }
}