use derive_more::Display;
use actix_web::{ResponseError, HttpResponse};

#[derive(Debug, Display)]
pub enum EchoIpError {
  /// Represents an empty source. For example, an empty text file being given
  /// as input to `count_words()`.
  #[display(fmt="IP address could not be resolved")]
  IpAddressResolutionFailed,

  #[display(fmt="Could not compile Handlebars template")]
  HandlebarsFailed,

  #[display(fmt="Could not get IP information from Max Mind DB")]
  MaxMindDbFailed,
}

/// Actix web uses `ResponseError` for conversion of errors to a response
impl ResponseError for EchoIpError {
  fn error_response(&self) -> HttpResponse {
    match self {
      EchoIpError::IpAddressResolutionFailed => {
        println!("IP Address resolution failed");
        HttpResponse::InternalServerError().finish()
      }

      EchoIpError::HandlebarsFailed => {
        println!("Handlebars template compilation failed");
        HttpResponse::InternalServerError().finish()
      }

      EchoIpError::MaxMindDbFailed => {
        println!("IP to location resolution failed");
        HttpResponse::InternalServerError().finish()
      }
    }
  }
}