extern crate iana_ip_db;

use iana_ip_db::{lookup, Country};


use std::net::IpAddr;
use std::collections::HashMap;
use std::collections::HashSet;


#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Action {
    Accept,
    Reject,
    Unknow,
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Action::Accept => write!(f, "ACCEPT"),
            Action::Reject => write!(f, "REJECT"),
            Action::Unknow => write!(f, "UNKNOW"),
        }
    }
}


pub struct Whitelist {
    cache: HashMap<IpAddr, Action>,
    loc_set: HashSet<Country>,
}

impl Whitelist {
    pub fn with_loc_set(loc_set: HashSet<Country>) -> Self {
        let cache = HashMap::new();

        Self { cache, loc_set }
    }

    pub fn ask(&mut self, addr: &IpAddr) -> Action {
        if let Some(v) = self.cache.get(addr) {
            return *v;
        }

        match lookup(addr) {
            Some((_start, _end, cc)) => {
                let v = self.loc_set.contains(&cc);
                let act = if v { Action::Reject } else { Action::Accept };
                self.cache.insert(addr.clone(), act);
                act
            },
            None => Action::Unknow,
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
    println!("[{:?}] {}", acl.ask(&ip), &ip);
    
    let ip  = "8.8.8.8".parse::<IpAddr>()?;
    println!("[{:?}] {}", acl.ask(&ip), &ip);

    let ip  = "220.181.38.148".parse::<IpAddr>()?; // baidu.com
    println!("[{:?}] {}", acl.ask(&ip), &ip);

    let ip  = "5.42.250.33".parse::<IpAddr>()?;    // www.moh.gov.sa
    println!("[{:?}] {}", acl.ask(&ip), &ip);


    Ok(())
}