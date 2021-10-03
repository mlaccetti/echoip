use serde::{Deserialize, Serialize};
use serde_json::Value;

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
}

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
    pub(crate) geo_info: GeoInfo,
    pub(crate) json: Value,
}
