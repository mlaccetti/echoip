mod api;
mod model;
mod util;
mod error;

use actix_web::{web, App, HttpServer, middleware::Logger};
use handlebars::Handlebars;
use log::debug;

use crate::api::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "echoip=debug,actix_web=debug");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  debug!("Starting server.");

  let mut handlebars = Handlebars::new();
  handlebars
    .register_templates_directory(".html", "./templates")
    .unwrap();
  let handlebars_ref = web::Data::new(handlebars);

  debug!("Constructing the App");

  HttpServer::new(move || {
    App::new()
      .app_data(handlebars_ref.clone())
      .wrap(Logger::default())
      /*.wrap(ErrorHandlers::new()
        .handler(
          http::StatusCode::INTERNAL_SERVER_ERROR,
          api::internal_server_error,
        ))*/
      .service(web::resource("/").route(web::get().to(index)))
  })
    .bind("localhost:8088")?
    .run()
    .await
}
