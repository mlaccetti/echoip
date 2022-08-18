use actix_files::NamedFile;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, web, HttpRequest, HttpResponse, Responder, Result};
use dns_lookup::lookup_addr;
use handlebars::Handlebars;
use log::{debug, warn};
use serde_json::json;
use std::net::{AddrParseError, IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

use crate::error::EchoIpError;
use crate::geoip_lookup;
use crate::model::{GeoInfo, Index, IpResult, PortLookup, UserInfo};

fn extract_ip(req: &HttpRequest) -> std::result::Result<IpResult, AddrParseError> {
  let _conn_info = req.connection_info();
  let _realip = _conn_info.realip_remote_addr();
  let _realip = clean_ip(_realip.unwrap().to_string());

  let mut _ipaddr: IpAddr = IpAddr::from_str("127.0.0.1").unwrap();
  debug!("Converting IP {} to IpAddr.", _realip);

  let _parsed_ip = _realip.parse::<IpAddr>();
  if _parsed_ip.is_ok() {
    return Ok(IpResult {
      ip: _parsed_ip.unwrap(),
      real_ip: _realip,
    });
  }

  Err(_parsed_ip.err().unwrap())
}

fn ip_to_decimal(ip: IpAddr) -> String {
  match ip {
    IpAddr::V4(ip4) => u32::from(ip4).to_string(),
    IpAddr::V6(ip6) => u128::from(ip6).to_string(),
  }
}

/***
* Possible IPs that we've seen so far:
* [::1]:51224
* 127.0.0.1:47324
* IPv4/v6 without trailing ports
 */
fn clean_ip(mut ip: String) -> String {
  let pos = ip.rfind(':');
  if let Some(value) = pos {
    debug!("Removing trailing colon from IP.");
    ip = ip.split_at(value).0.to_string();
  }

  debug!("Removing square braces from IP.");
  ip = ip.replace(&['[', ']'][..], "");

  debug!("Cleaned IP: {}", ip);

  ip
}

fn get_user_info(req: &HttpRequest, ip: &IpAddr) -> UserInfo {
  let user_agent_raw: String = req
    .headers()
    .get("User-Agent")
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();
  debug!(
    "Raw user agent [ {} ] for {}",
    user_agent_raw,
    ip.to_string()
  );

  let mut user_agent = user_agent_raw.clone();
  let mut user_agent_comment = String::new();
  if user_agent_raw.contains(' ') {
    let ua_split: Vec<&str> = user_agent_raw.splitn(2, ' ').collect();
    user_agent = ua_split[0].to_string();
    user_agent_comment = ua_split[1].to_string();
  }

  UserInfo {
    hostname: lookup_addr(ip).unwrap(),
    user_agent,
    user_agent_comment,
    user_agent_raw,
  }
}

fn generate_response(req: HttpRequest) -> Index {
  let mut geo_info: Option<GeoInfo> = None;
  let mut user_info: Option<UserInfo> = None;

  let mut _ipaddr: IpAddr = IpAddr::from_str("127.0.0.1").unwrap();
  let mut _realip;

  let _parsed_ip = extract_ip(&req);
  let _has_valid_ip = _parsed_ip.is_ok();
  if _has_valid_ip {
    let _unwrapped_ip = _parsed_ip.unwrap();
    _ipaddr = _unwrapped_ip.ip;
    _realip = _unwrapped_ip.real_ip;

    debug!("Converted IP {} properly, getting GeoIP info.", _realip);

    let lookup: geoip_lookup::GeoipLookup = geoip_lookup::GeoipLookup::new();

    let _geo_info = lookup.lookup_geo_for_ip(_ipaddr);
    if _geo_info.is_ok() {
      debug!("Collected GeoIP info for {}.", _realip);
      geo_info = Some(_geo_info.unwrap());
    } else {
      warn!("Could not retrieve GeoIP info for {}.", _realip);
    }

    debug!("Getting user data for {}.", _realip);
    user_info = Some(get_user_info(&req, &_ipaddr));
  }

  Index {
    host: String::from(req.connection_info().host()),
    ip: _ipaddr.to_string(),
    decimal_ip: ip_to_decimal(_ipaddr),
    has_geo_info: geo_info.is_some(),
    geo_info,
    user_info,
  }
}

pub(crate) async fn html_response(
  http_request: HttpRequest,
  handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, EchoIpError> {
  let data = generate_response(http_request);

  debug!("Converting response to JSON.");
  let response = json!({
    "data": data,
    "json": serde_json::to_string(&data).unwrap()
  });

  debug!("Rendering Handlebars template.");
  let body = handlebars
    .render("index", &response)
    .map_err(|err| EchoIpError::HandlebarsFailed { source: err })?;

  debug!("Returning response to browser.");
  Ok(HttpResponse::Ok().body(body))
}

pub(crate) async fn plain_response(http_request: HttpRequest) -> HttpResponse {
  let _realip = http_request
    .connection_info()
    .realip_remote_addr()
    .unwrap()
    .to_string();
  debug!("Extracting IP from plain response: {}", _realip);

  let _realip = clean_ip(_realip);
  debug!("IP from the client: {}", _realip);
  HttpResponse::Ok().content_type("text/plain").body(_realip)
}

pub(crate) async fn json_response(http_request: HttpRequest) -> HttpResponse {
  let data = generate_response(http_request);

  debug!("Sending JSON response.");
  HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string(&data).unwrap())
}

pub async fn port_lookup(req: HttpRequest, path: web::Path<u16>) -> HttpResponse {
  let _port = path.into_inner();
  let _ip = extract_ip(&req).unwrap();
  let _real_ip = _ip.real_ip;
  let _sock_addr = SocketAddr::new(_ip.ip, _port);

  let stream = TcpStream::connect_timeout(&_sock_addr, Duration::new(5, 0));
  let reachable = stream.is_ok();

  let result = PortLookup {
    ip: _real_ip,
    port: _port,
    reachable,
  };

  debug!("Sending port lookup response.");
  HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string(&result).unwrap())
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
  let new_resp = NamedFile::open("static/errors/500.html")?
    .customize()
    .with_status(res.status())
    .respond_to(res.request())
    .map_into_boxed_body()
    .map_into_right_body();

  Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}
