use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use log::error;
use maxminddb::MaxMindDBError;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum EchoIpError {
  #[display(fmt = "Could not compile Handlebars template")]
  HandlebarsFailed,

  #[display(fmt = "Could not get IP information from Max Mind DB")]
  MaxMindDbFailed { source: MaxMindDBError },
}

/// Actix web uses `ResponseError` for conversion of errors to a response
impl ResponseError for EchoIpError {
  fn error_response(&self) -> HttpResponse {
    match self {
      EchoIpError::HandlebarsFailed => {
        error!("Handlebars template compilation failed");
        HttpResponse::InternalServerError().finish()
      }

      EchoIpError::MaxMindDbFailed { source } => {
        error!("IP to location resolution failed: {}", source);
        HttpResponse::InternalServerError().finish()
      }
    }
  }
}
