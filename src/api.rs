use actix_web::{HttpResponse, HttpRequest, web, dev, Result};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse};
use handlebars::Handlebars;
use log::debug;
use serde_json::json;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use crate::model::Index;
use crate::util;
use actix_files::NamedFile;
use crate::error::EchoIpError;

fn ip_to_decimal(ip: IpAddr) -> String {
  match ip {
    IpAddr::V4(ip4) => u32::from(ip4).to_string(),
    IpAddr::V6(ip6) => u128::from(ip6).to_string(),
  }
}

pub(crate) async fn index(req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, EchoIpError<'_>> {
  let _conn_info = req.connection_info();
  let _realip = _conn_info.realip_remote_addr();
  let _realip = match _realip {
    Some(ip) => ip,
    None => return Err(EchoIpError::new("No remote IP found for connection."))
  };

  let _ipaddr = SocketAddr::from_str(_realip)?.ip();
  debug!("IP from client: {:#?}", _ipaddr);

  let lookup: util::GeoipLookup = util::GeoipLookup::new();
  let geo_info = lookup.lookup_geo_for_ip(_ipaddr)?;

  debug!("{:#?}", geo_info);
  let data = Index {
    host: String::from(req.connection_info().host()),
    ip: _ipaddr.to_string(),
    decimal_ip: ip_to_decimal(_ipaddr),
    geo_info,
    json: Default::default(),
  };

  debug!("Converting response to JSON.");
  let response = json!({
    "data": data,
    "json": serde_json::to_string(&data).unwrap()
  });

  debug!("Rendering Handlebars template.");
  let body = hb.render("index", &response).map_err(EchoIpError::new("Could not render template."))?;

  debug!("Returning response to browser.");
  Ok(HttpResponse::Ok().body(body))
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
