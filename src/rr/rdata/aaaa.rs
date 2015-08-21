use std::net::Ipv6Addr;

use ::serialize::binary::*;
use ::error::*;
use ::rr::record_data::RData;

//-- RFC 1886 -- IPv6 DNS Extensions              December 1995

// 2.2 AAAA data format
//
//    A 128 bit IPv6 address is encoded in the data portion of an AAAA
//    resource record in network byte order (high-order byte first).
//
// AAAA { address: Ipv6Addr }
pub fn read(decoder: &mut BinDecoder) -> DecodeResult<RData> {
  let a: u16 = try!(decoder.read_u16());
  let b: u16 = try!(decoder.read_u16());
  let c: u16 = try!(decoder.read_u16());
  let d: u16 = try!(decoder.read_u16());
  let e: u16 = try!(decoder.read_u16());
  let f: u16 = try!(decoder.read_u16());
  let g: u16 = try!(decoder.read_u16());
  let h: u16 = try!(decoder.read_u16());

  Ok(RData::AAAA{ address: Ipv6Addr::new(a,b,c,d,e,f,g,h)})
}

pub fn emit(encoder: &mut BinEncoder, aaaa: &RData) -> EncodeResult {
  if let RData::AAAA { address } = *aaaa {
    let segments = address.segments();

    try!(encoder.emit_u16(segments[0]));
    try!(encoder.emit_u16(segments[1]));
    try!(encoder.emit_u16(segments[2]));
    try!(encoder.emit_u16(segments[3]));
    try!(encoder.emit_u16(segments[4]));
    try!(encoder.emit_u16(segments[5]));
    try!(encoder.emit_u16(segments[6]));
    try!(encoder.emit_u16(segments[7]));
    Ok(())
  } else {
    panic!("wrong type here {:?}", aaaa)
  }
}


#[cfg(test)]
mod tests {
  use std::net::Ipv6Addr;
  use std::str::FromStr;

  use super::*;
  use ::rr::record_data::RData;
  use ::serialize::binary::bin_tests::{test_read_data_set, test_emit_data_set};

  fn get_data() -> Vec<(RData, Vec<u8>)> {
    vec![
    (RData::AAAA{ address: Ipv6Addr::from_str("::").unwrap()}, vec![0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0]), // base case
    (RData::AAAA{ address: Ipv6Addr::from_str("1::").unwrap()}, vec![0,1,0,0,0,0,0,0, 0,0,0,0,0,0,0,0]),
    (RData::AAAA{ address: Ipv6Addr::from_str("0:1::").unwrap()}, vec![0,0,0,1,0,0,0,0, 0,0,0,0,0,0,0,0]),
    (RData::AAAA{ address: Ipv6Addr::from_str("0:0:1::").unwrap()}, vec![0,0,0,0,0,1,0,0, 0,0,0,0,0,0,0,0]),
    (RData::AAAA{ address: Ipv6Addr::from_str("0:0:0:1::").unwrap()}, vec![0,0,0,0,0,0,0,1, 0,0,0,0,0,0,0,0]),
    (RData::AAAA{ address: Ipv6Addr::from_str("::1:0:0:0").unwrap()}, vec![0,0,0,0,0,0,0,0, 0,1,0,0,0,0,0,0]),
    (RData::AAAA{ address: Ipv6Addr::from_str("::1:0:0").unwrap()}, vec![0,0,0,0,0,0,0,0, 0,0,0,1,0,0,0,0]),
    (RData::AAAA{ address: Ipv6Addr::from_str("::1:0").unwrap()}, vec![0,0,0,0,0,0,0,0, 0,0,0,0,0,1,0,0]),
    (RData::AAAA{ address: Ipv6Addr::from_str("::1").unwrap()}, vec![0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1]),
    (RData::AAAA{ address: Ipv6Addr::from_str("::127.0.0.1").unwrap()}, vec![0,0,0,0,0,0,0,0, 0,0,0,0,127,0,0,1]),
    (RData::AAAA{ address: Ipv6Addr::from_str("FF00::192.168.64.32").unwrap()}, vec![255,0,0,0,0,0,0,0, 0,0,0,0,192,168,64,32]),
    ]
  }

  #[test]
  fn test_read() {
    test_read_data_set(get_data(), |ref mut d| read(d));
  }

  #[test]
  fn test_emit() {
    test_emit_data_set(get_data(), |e,d| emit(e,&d));
  }
}
