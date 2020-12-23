use std::cmp::Ordering;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

mod country;
#[allow(dead_code)]
mod v4_db;
#[allow(dead_code)]
mod v6_db;

pub use self::country::Country;


pub fn lookup(ip: &IpAddr) -> Option<(IpAddr, IpAddr, Country)> {
    match ip {
        &IpAddr::V4(v4_addr) => {
            let v4_number = u32::from(v4_addr);
            let ret = v4_db::IPV4_RECORDS.binary_search_by(|&(first, last, _cc)| {
                if v4_number >= last {
                    Ordering::Less
                } else if v4_number >= first && v4_number <= last {
                    Ordering::Equal
                } else if v4_number < first {
                    Ordering::Greater
                } else {
                    unreachable!()
                }
            });
            match ret {
                Ok(pos) => {
                    let (first, last, cc) = v4_db::IPV4_RECORDS[pos];
                    Some( (IpAddr::from(Ipv4Addr::from(first)),
                           IpAddr::from(Ipv4Addr::from(last)),
                           Country::from_index(cc)) )
                }
                Err(_) => None
            }
        }
        &IpAddr::V6(v6_addr) => {
            let v6_number = u128::from(v6_addr);
            let ret = v6_db::IPV6_RECORDS.binary_search_by(|&(first, last, _cc)| {
                if v6_number >= last {
                    Ordering::Less
                } else if v6_number >= first && v6_number <= last {
                    Ordering::Equal
                } else if v6_number < first {
                    Ordering::Greater
                } else {
                    unreachable!()
                }
            });

            match ret {
                Ok(pos) => {
                    let (first, last, cc) = v6_db::IPV6_RECORDS[pos];
                    Some( (IpAddr::from(Ipv6Addr::from(first)),
                           IpAddr::from(Ipv6Addr::from(last)),
                           Country::from_index(cc) ))
                }
                Err(_) => None
            }
        }
    }
}


#[test]
fn test_lookup_ipv4() {
    assert_eq!(lookup(&IpAddr::from(Ipv4Addr::new(8, 8, 8, 8))).is_some(), true);
}

#[test]
fn test_lookup_ipv6() {
    assert_eq!(lookup(&"2001:218::".parse().unwrap()).is_some(), true);
}