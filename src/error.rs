use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};
use handlebars::RenderError;
use log::error;
use maxminddb::MaxMindDBError;

#[derive(Debug, Display, Error)]
pub enum EchoIpError {
  #[display(fmt = "Could not compile Handlebars template")]
  HandlebarsFailed { source: RenderError },

  #[display(fmt = "Could not get IP information from Max Mind DB")]
  MaxMindDbFailed { source: MaxMindDBError },
}

/// Actix web uses `ResponseError` for conversion of errors to a response
impl ResponseError for EchoIpError {
  fn error_response(&self) -> HttpResponse {
    match self {
      EchoIpError::HandlebarsFailed { source } => {
        error!("Handlebars template compilation failed: {}", source);
        HttpResponse::InternalServerError().finish()
      }

      EchoIpError::MaxMindDbFailed { source } => {
        error!("IP to location resolution failed: {}", source);
        HttpResponse::InternalServerError().finish()
      }
    }
  }
}
