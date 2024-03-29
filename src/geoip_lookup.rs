use maxminddb::{geoip2, geoip2::city::Subdivision, Reader};
use std::net::IpAddr;

use crate::error::EchoIpError;
use crate::model::GeoInfo;

pub struct GeoipLookup {
  city_reader: Reader<Vec<u8>>,
  asn_reader: Reader<Vec<u8>>,
}

impl GeoipLookup {
  pub fn new() -> Self {
    Self {
      // TODO these throw errors if the files are missing
      city_reader: Reader::open_readfile("./geoip/GeoLite2-City.mmdb").unwrap(),
      asn_reader: Reader::open_readfile("./geoip/GeoLite2-ASN.mmdb").unwrap(),
    }
  }

  pub fn lookup_geo_for_ip(&self, _ip: IpAddr) -> Result<GeoInfo, EchoIpError> {
    let geoip_city: geoip2::City = self
      .city_reader
      .lookup::<geoip2::City>(_ip)
      .map_err(|err| EchoIpError::MaxMindDbFailed { source: err })?;
    let geoip_asn: geoip2::Asn = self
      .asn_reader
      .lookup::<geoip2::Asn>(_ip)
      .map_err(|err| EchoIpError::MaxMindDbFailed { source: err })?;

    // TODO if the IP cannot be found, insert some dummy data, or redirect to a different page?

    let _country = geoip_city.country.unwrap();
    let _region: Subdivision = geoip_city.subdivisions.unwrap().get(0).unwrap().clone();
    let _location = geoip_city.location.unwrap();

    let country_name = _country.names.unwrap().get("en").unwrap().to_string();
    let country_iso = _country.iso_code.unwrap().to_string();
    let city = geoip_city
      .city
      .unwrap()
      .names
      .unwrap()
      .get("en")
      .unwrap()
      .to_string();

    let region = _region.names.unwrap().get("en").unwrap().to_string();
    let region_code = _region.iso_code.unwrap().to_string();

    let metro_code = _location.metro_code.unwrap_or(0).to_owned();

    let postal_code = String::from(
      geoip_city
        .postal
        .unwrap()
        .code
        .iter()
        .next()
        .unwrap()
        .to_owned(),
    );

    let latitude = _location.latitude.unwrap();
    let longitude = _location.longitude.unwrap();
    let timezone = _location.time_zone.unwrap().to_string();

    let mut asn = geoip_asn.autonomous_system_number.unwrap().to_string();
    let asn_org = geoip_asn
      .autonomous_system_organization
      .unwrap()
      .to_string();

    let parsed = asn.parse::<i32>();
    if parsed.is_ok() {
      asn = format!("AS{}", parsed.unwrap());
    }

    Ok(GeoInfo {
      country_name,
      country_iso,
      country_in_eu: _country.is_in_european_union.unwrap_or(false),
      region,
      region_code,
      city,
      metro_code,
      postal_code,
      latitude,
      longitude,
      timezone,
      asn,
      asn_org,
      box_lat_top: latitude + 0.05,
      box_lat_bottom: latitude - 0.05,
      box_lon_left: longitude - 0.05,
      box_lon_right: longitude + 0.05,
    })
  }
}
