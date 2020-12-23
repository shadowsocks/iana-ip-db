extern crate iana_ip_db;

use iana_ip_db::{lookup, Country};


use std::net::IpAddr;
use std::collections::HashMap;
use std::collections::HashSet;


pub struct Whitelist {
    cache: HashMap<IpAddr, bool>,
    loc_set: HashSet<Country>,
}

impl Whitelist {
    pub fn with_loc_set(loc_set: HashSet<Country>) -> Self {
        let cache = HashMap::new();

        Self { cache, loc_set }
    }

    pub fn contains(&mut self, addr: &IpAddr) -> bool {
        if let Some(v) = self.cache.get(addr) {
            return *v;
        }

        match lookup(addr) {
            Some((_start, _end, cc)) => {
                let v = self.loc_set.contains(&cc);
                self.cache.insert(addr.clone(), v);
                v
            },
            None => false,
        }
    }
}

impl std::fmt::Debug for Whitelist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Whitelist")
            .field("loc_set", &self.loc_set)
            .finish()
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 沙特阿拉伯 Saudi Arabia
    let sa = "SA".parse::<Country>()?;

    let mut set = HashSet::new();
    set.insert(Country::CN);
    set.insert(sa);

    let mut acl = Whitelist::with_loc_set(set);
    println!("ACL: {:?}", acl);


    let ip  = "1.1.1.1".parse::<IpAddr>()?;
    let val = acl.contains(&ip);
    println!("[{}] {}", if val { "ACCEPT" } else { "REJECT" }, &ip);
    

    let ip  = "8.8.8.8".parse::<IpAddr>()?;
    let val = acl.contains(&ip);
    println!("[{}] {}", if val { "ACCEPT" } else { "REJECT" }, &ip);


    let ip  = "220.181.38.148".parse::<IpAddr>()?; // baidu.com
    let val = acl.contains(&ip);
    println!("[{}] {}", if val { "ACCEPT" } else { "REJECT" }, &ip);

    let ip  = "5.42.250.33".parse::<IpAddr>()?; // www.moh.gov.sa
    let val = acl.contains(&ip);
    println!("[{}] {}", if val { "ACCEPT" } else { "REJECT" }, &ip);

    Ok(())
}