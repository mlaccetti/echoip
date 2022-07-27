use actix_web::dev::RequestHead;
use actix_web::guard::Guard;
use actix_web::http;

pub(crate) struct AcceptHeader {
  pub(crate) content_type: String,
}

impl Guard for AcceptHeader {
  fn check(&self, req: &RequestHead) -> bool {
    req.headers().contains_key(http::header::ACCEPT)
      && String::from(
        req
          .headers
          .get(http::header::ACCEPT)
          .unwrap()
          .to_str()
          .unwrap(),
      )
      .contains(&self.content_type)
  }
}
