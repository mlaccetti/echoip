mod geoip_lookup;
mod model;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, middleware::Logger, Error};
use handlebars::Handlebars;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use std::net::{IpAddr};
use std::str::FromStr;
use crate::model::GeoInfo;

#[derive(Serialize, Deserialize, Debug)]
struct Index {
  host: String,
  ip: String,
  decimal_ip: String,
  geo_info: GeoInfo,
  json: Value,
}

fn ip_to_decimal(ip: IpAddr) -> String {
  match ip {
    IpAddr::V4(ip4) => u32::from(ip4).to_string(),
    IpAddr::V6(ip6) => u128::from(ip6).to_string(),
  }
}

async fn index(req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {
  let _conn_info = req.connection_info();
  let _ip = _conn_info.realip_remote_addr().unwrap().split(":").next().unwrap();
  let _ipaddr = IpAddr::from_str(_ip).unwrap();

  let lookup: geoip_lookup::GeoipLookup = geoip_lookup::GeoipLookup::new();
  let geo_info = lookup.lookup_geo_for_ip(_ipaddr);

  debug!("{:#?}", geo_info);
  let data = Index {
    host: String::from(req.connection_info().host()),
    ip: String::from(_ip),
    decimal_ip: ip_to_decimal(_ipaddr),
    geo_info,
    json: Default::default(),
  };

  let response = json!({
    "data": data,
    "json": serde_json::to_string(&data).unwrap()
  });

  let body = hb.render("index", &response).unwrap();
  Ok(HttpResponse::Ok().body(body))
}

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

  HttpServer::new(move || {
    App::new()
      .app_data(handlebars_ref.clone())
      .wrap(Logger::default())
      .service(web::resource("/").route(web::get().to(index)))
  })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
