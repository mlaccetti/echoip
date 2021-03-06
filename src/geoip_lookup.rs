use maxminddb::{geoip2, Reader};
use std::net::IpAddr;

use crate::model::GeoInfo;
use maxminddb::geoip2::model::Subdivision;

pub struct GeoipLookup {
  city_reader: Reader<Vec<u8>>
}

impl GeoipLookup {
  pub fn new() -> GeoipLookup {
    GeoipLookup {
      city_reader: Reader::open_readfile("geoip/GeoLite2-City.mmdb").unwrap(),
    }
  }

  pub(crate) fn lookup_geo_for_ip(&self, _ip: IpAddr) -> GeoInfo {
    let geoip_city: geoip2::City = self.city_reader.lookup(_ip).unwrap();

    let _country = geoip_city.country.unwrap();
    let _region: Subdivision = geoip_city.subdivisions.unwrap().iter().next().unwrap().clone();
    let _location = geoip_city.location.unwrap();

    let country_name = String::from(_country.names.unwrap().get("en").unwrap().to_owned());
    let country_iso = String::from(_country.iso_code.unwrap().to_owned());
    let city = String::from(geoip_city.city.unwrap().names.unwrap().get("en").unwrap().to_owned());

    let region = String::from(_region.names.unwrap().get("en").unwrap().to_owned());
    let region_code = String::from(_region.iso_code.unwrap().to_owned());

    let metro_code = _location.metro_code.unwrap_or(0).to_owned();

    let postal_code = String::from(geoip_city.postal.unwrap().code.iter().next().unwrap().to_owned());

    let latitude = _location.latitude.unwrap();
    let longitude = _location.longitude.unwrap();
    let timezone = String::from(_location.time_zone.unwrap().to_owned());

    GeoInfo {
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
    }
  }
}
