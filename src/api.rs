use actix_files::NamedFile;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, web, HttpRequest, HttpResponse, Result};
use dns_lookup::lookup_addr;
use handlebars::Handlebars;
use log::{debug, warn};
use serde_json::json;
use std::net::IpAddr;
use std::str::FromStr;

use crate::error::EchoIpError;
use crate::model::{GeoInfo, Index, UserInfo};
use crate::geoip_lookup;

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

fn generate_response(http_request: HttpRequest) -> Index {
  let _conn_info = http_request.connection_info();
  let _realip = _conn_info.realip_remote_addr();
  let _realip = clean_ip(_realip.unwrap().to_string());

  let mut _ipaddr: IpAddr = IpAddr::from_str("127.0.0.1").unwrap();

  let mut geo_info: Option<GeoInfo> = None;
  let mut user_info: Option<UserInfo> = None;

  debug!("Converting IP {} to IpAddr.", _realip);
  let _parsed_ip = _realip.parse::<IpAddr>();
  let _has_valid_ip = _parsed_ip.is_ok();
  if _has_valid_ip {
    debug!("Converted IP {} properly, getting GeoIP info.", _realip);
    _ipaddr = _parsed_ip.unwrap();
    let lookup: geoip_lookup::GeoipLookup = geoip_lookup::GeoipLookup::new();

    let _geo_info = lookup.lookup_geo_for_ip(_ipaddr);
    if _geo_info.is_ok() {
      debug!("Collected GeoIP info for {}.", _realip);
      geo_info = Some(_geo_info.unwrap());
    } else {
      warn!("Could not retrieve GeoIP info for {}.", _realip);
    }

    debug!("Getting user data for {}.", _realip);
    user_info = Some(get_user_info(&http_request, &_ipaddr));
  }

  Index {
    host: String::from(http_request.connection_info().host()),
    ip: _ipaddr.to_string(),
    decimal_ip: ip_to_decimal(_ipaddr),
    has_geo_info: geo_info.is_some(),
    geo_info,
    user_info
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
    .map_err(|_| EchoIpError::HandlebarsFailed)?;

  debug!("Returning response to browser.");
  Ok(HttpResponse::Ok().body(body))
}

pub(crate) fn plain_response(http_request: HttpRequest) -> HttpResponse {
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

pub(crate) async fn json_response(http_request: HttpRequest) -> Result<HttpResponse> {
  let data = generate_response(http_request);

  debug!("Sending JSON response.");
  Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&data).unwrap()))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
  // debug!("Error! {:#?}", res.);
  let new_resp = NamedFile::open("static/errors/500.html")?
    .set_status_code(res.status())
    .into_response(res.request())?;
  Ok(ErrorHandlerResponse::Response(
    res.into_response(new_resp.into_body()),
  ))
}
