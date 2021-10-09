use actix_files::NamedFile;
use actix_web::{HttpResponse, HttpRequest, web, dev, Result};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse};
use dns_lookup::lookup_addr;
use handlebars::Handlebars;
use log::{debug, warn};
use serde_json::json;
use std::net::IpAddr;

use crate::model::{Index, UserInfo, GeoInfo};
use crate::util;
use crate::error::EchoIpError;
use std::str::FromStr;

fn ip_to_decimal(ip: IpAddr) -> String {
  match ip {
    IpAddr::V4(ip4) => u32::from(ip4).to_string(),
    IpAddr::V6(ip6) => u128::from(ip6).to_string(),
  }
}

fn get_user_info(req: &HttpRequest, ip: &IpAddr) -> UserInfo {
  let user_agent_raw: String = req.headers().get("User-Agent").unwrap().to_str().unwrap().to_string();
  debug!("Raw user agent [ {} ] for {}", user_agent_raw, ip.to_string());

  let mut user_agent = user_agent_raw.clone();
  let mut user_agent_comment = String::new();
  if user_agent_raw.contains(" ") {
    let ua_split: Vec<&str> = user_agent_raw.splitn(2, " ").collect();
    user_agent = ua_split[0].to_string();
    user_agent_comment = ua_split[1].to_string();
  }

  return UserInfo {
    hostname: lookup_addr(ip).unwrap(),
    user_agent,
    user_agent_comment,
    user_agent_raw,
  };
}

pub(crate) async fn index(_req: HttpRequest, _hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, EchoIpError> {
  let _accept = _req.headers().get("Accept");
  if _accept.is_some() {
    let _accept = _accept.unwrap();
    if !_accept.is_empty() && _accept.to_str().unwrap().contains("text/plain") {
      let res: HttpResponse = plain_response(_req);
      return Ok(res);
    }
  }

  let _conn_info = _req.connection_info();
  let _realip = _conn_info.realip_remote_addr();
  let _realip = match _realip {
    Some(ip) => ip.splitn(2, ":").nth(0).unwrap(),
    None => ""
  };
  debug!("IP from the client: {}", _realip);

  let _has_ip_info: bool = _realip != "";
  debug!("We got an IP from the client: {}", _has_ip_info);

  let mut _ipaddr: IpAddr = IpAddr::from_str("127.0.0.1").unwrap();

  let mut geo_info: Option<GeoInfo> = None;
  let mut user_info: Option<UserInfo> = None;

  if _has_ip_info {
    debug!("Converting IP {} to IpAddr.", _realip);
    let _parsed_ip = _realip.parse::<IpAddr>();
    if _parsed_ip.is_ok() {
      debug!("Converted IP {} properly, getting GeoIP info.", _realip);
      _ipaddr = _parsed_ip.unwrap();
      let lookup: util::GeoipLookup = util::GeoipLookup::new();

      let _geo_info = lookup.lookup_geo_for_ip(_ipaddr);
      if _geo_info.is_ok() {
        debug!("Collected GeoIP info for {}.", _realip);
        geo_info = Some(_geo_info.unwrap());
      } else {
        warn!("Could not retrieve GeoIP info for {}.", _realip);
      }

      debug!("Getting user data for {}.", _realip);
      user_info = Some(get_user_info(&_req, &_ipaddr));
    } else {
      warn!("Could not convert {} to IpAddr, skipped looking up GeoIP/ user data.", _realip);
    }
  }

  let data = Index {
    host: String::from(_req.connection_info().host()),
    ip: _ipaddr.to_string(),
    decimal_ip: ip_to_decimal(_ipaddr),
    has_geo_info: _has_ip_info,
    geo_info,
    user_info,
    json: Default::default(),
  };

  debug!("Converting response to JSON.");
  let response = json!({
    "data": data,
    "json": serde_json::to_string(&data).unwrap()
  });

  debug!("Rendering Handlebars template.");
  let body = _hb.render("index", &response).map_err(|_| EchoIpError::HandlebarsFailed)?;

  debug!("Returning response to browser.");
  Ok(HttpResponse::Ok().body(body))
}

pub(crate) fn plain_response(_req: HttpRequest) -> HttpResponse {
  let _realip = _req.connection_info().realip_remote_addr().unwrap().to_string();
  // let _realip = _conninfo.realip_remote_addr();
  let _splitip = _realip.splitn(2, ":").nth(0).unwrap();
  debug!("IP from the client: {}", _splitip);
  HttpResponse::Ok().content_type("text/plain").body(_splitip)
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
