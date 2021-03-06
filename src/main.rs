#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result, middleware::Logger};
use askama::Template;
use log::debug;
use maxminddb::{geoip2, Reader};
use std::net::{IpAddr};
use std::str::FromStr;

struct Readers {
  city_reader: Reader<Vec<u8>>
}

impl Readers {
  fn load_readers() -> Self {
    let _city_reader = Reader::open_readfile("geoip/GeoLite2-City.mmdb").unwrap();
    Self { city_reader: _city_reader }
  }
}

lazy_static! {
  static ref READERS: Readers = Readers::load_readers();
}

#[derive(Template)]
#[template(path = "template.html")]
struct Index<'a> {
  host: &'a str,
  ip: &'a str,
  decimal_ip: u128,
  // city: geoip2::City<'a>,
}

fn get_geoip_for_ip<'a>(_ip: IpAddr) -> geoip2::City<'a> {
  // READERS.city_reader.lookup(ip).unwrap()
  READERS.city_reader.lookup(IpAddr::from_str("174.88.222.81").unwrap()).unwrap()
}

fn ip_to_decimal(ip: IpAddr) -> u128 {
  match ip {
    IpAddr::V4(ip4) => u32::from(ip4) as u128,
    IpAddr::V6(ip6) => u128::from(ip6),
  }
}

async fn index(req: HttpRequest) -> Result<HttpResponse> {
  let _conn_info = req.connection_info();
  let _ip = _conn_info.realip_remote_addr().unwrap().split(":").next().unwrap();
  let _ipaddr = IpAddr::from_str(_ip).unwrap();

  debug!("{:#?}", get_geoip_for_ip(_ipaddr));

  let s = Index {
    host: req.connection_info().host(),
    ip: _ip,
    decimal_ip: ip_to_decimal(_ipaddr),
    // city: get_geoip_for_ip(_ipaddr),
  }.render().unwrap();
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "echoip=debug,actix_web=info");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  debug!("Starting server.");

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .service(web::resource("/").route(web::get().to(index)))
  })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
