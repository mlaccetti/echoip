use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GeoInfo {
  pub(crate) country_name: String,
  pub(crate) country_iso: String,
  pub(crate) country_in_eu: bool,
  pub(crate) region: String,
  pub(crate) region_code: String,
  pub(crate) city: String,
  pub(crate) metro_code: u16,
  pub(crate) postal_code: String,
  pub(crate) latitude: f64,
  pub(crate) longitude: f64,
  pub(crate) timezone: String,
  pub(crate) asn: String,
  pub(crate) asn_org: String,
  pub(crate) box_lat_top: f64,
  pub(crate) box_lat_bottom: f64,
  pub(crate) box_lon_left: f64,
  pub(crate) box_lon_right: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserInfo {
  pub(crate) hostname: String,
  pub(crate) user_agent: String,
  pub(crate) user_agent_comment: String,
  pub(crate) user_agent_raw: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
  pub(crate) host: String,
  pub(crate) ip: String,
  pub(crate) decimal_ip: String,
  pub(crate) has_geo_info: bool,
  pub(crate) geo_info: Option<GeoInfo>,
  pub(crate) user_info: Option<UserInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortLookup {
  pub(crate) ip: String,
  pub(crate) port: u16,
  pub(crate) reachable: bool,
}

pub struct IpResult {
  pub(crate) ip: IpAddr,
  pub(crate) real_ip: String,
}
