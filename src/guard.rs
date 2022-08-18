use actix_web::guard::{Guard, GuardContext};
use actix_web::http;

pub(crate) struct AcceptHeader {
  pub(crate) content_type: String,
}

impl Guard for AcceptHeader {
  fn check(&self, ctx: &GuardContext<'_>) -> bool {
    ctx.head().headers.contains_key(http::header::ACCEPT)
      && String::from(
        ctx
          .head()
          .headers
          .get(http::header::ACCEPT)
          .unwrap()
          .to_str()
          .unwrap(),
      )
      .contains(&self.content_type)
  }
}
