mod api;
mod error;
mod geoip_lookup;
mod guard;
mod model;

use actix_files as fs;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use handlebars::Handlebars;
use log::debug;

use crate::api::{html_response, json_response, plain_response, port_lookup};
use crate::guard::AcceptHeader;
use actix_web::middleware::ErrorHandlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "echoip=debug,actix_web=debug,info");
  std::env::set_var("RUST_BACKTRACE", "full");
  env_logger::init();

  debug!("Starting server.");

  let mut handlebars = Handlebars::new();
  handlebars
    .register_templates_directory(".hbs", "./templates")
    .unwrap();
  let handlebars_ref = web::Data::new(handlebars);

  debug!("Constructing the App");

  HttpServer::new(move || {
    App::new()
      .app_data(handlebars_ref.clone())
      .wrap(Logger::default())
      .wrap(ErrorHandlers::new().handler(
        http::StatusCode::INTERNAL_SERVER_ERROR,
        api::internal_server_error,
      ))
      .service(
        web::resource("/")
          .route(
            web::get()
              .guard(AcceptHeader {
                content_type: String::from("text/html"),
              })
              .to(html_response),
          )
          .route(web::get().to(plain_response)),
      )
      .service(web::resource("/json").to(json_response))
      .service(web::resource("/port/{port}").to(port_lookup))
      .service(fs::Files::new("/", "./static"))
  })
  .bind("0.0.0.0:8088")?
  .run()
  .await
}
