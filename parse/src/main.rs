extern crate getopts;
extern crate smoltcp;


use smoltcp::wire::{
    IpAddress,
    Ipv4Address, Ipv4Cidr,
    Ipv6Address, Ipv6Cidr,
};


use std::fmt;
use std::cmp;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::io::{Write, Read};
use std::fs::{self, File, OpenOptions};
use std::collections::HashSet;
use std::net::{Ipv4Addr, Ipv6Addr};


mod status;
mod country;
mod registry;

use self::status::{Status, InvalidStatus};
use self::country::{Country, InvalidCountryCode};
use self::registry::{Registry, InvalidRegistry};


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ParseError {
    InvalidStatus,
    InvalidCountryCode,
    InvalidRegistry,
    Unrecognized,
    Dropped,
    Truncated,
}

impl std::error::Error for ParseError { }

impl From<InvalidStatus> for ParseError {
    fn from(_src: InvalidStatus) -> ParseError {
        ParseError::InvalidStatus
    }
}
impl From<InvalidCountryCode> for ParseError {
    fn from(_src: InvalidCountryCode) -> ParseError {
        ParseError::InvalidCountryCode
    }
}
impl From<InvalidRegistry> for ParseError {
    fn from(_src: InvalidRegistry) -> ParseError {
        ParseError::InvalidRegistry
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


// Files not exists
// ("delegated-arin-latest",             "https://ftp.arin.net/pub/stats/arin/delegated-arin-latest"),
// ("delegated-iana-extended-latest",    "ftp://ftp.apnic.net/public/stats/iana/delegated-iana-extended-latest"),
pub const IANA_RIR_FILES: [(&str, &str); 10] = [
    ("delegated-arin-extended-latest",    "https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest"),
    ("delegated-ripencc-latest",          "https://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-latest"),
    ("delegated-ripencc-extended-latest", "https://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest"),
    ("delegated-apnic-latest",            "https://ftp.apnic.net/stats/apnic/delegated-apnic-latest"),
    ("delegated-apnic-extended-latest",   "https://ftp.apnic.net/stats/apnic/delegated-apnic-extended-latest"),
    ("delegated-lacnic-latest",           "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-latest"),
    ("delegated-lacnic-extended-latest",  "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest"),
    ("delegated-afrinic-latest",          "https://ftp.afrinic.net/pub/stats/afrinic/delegated-afrinic-latest"),
    ("delegated-afrinic-extended-latest", "https://ftp.afrinic.net/pub/stats/afrinic/delegated-afrinic-extended-latest"),
    ("delegated-iana-latest",             "https://ftp.apnic.net/stats/iana/delegated-iana-latest"),
];


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct Ipv4Range {
    pub start_ip: Ipv4Address,
    pub end_ip  : Ipv4Address,
}

impl Ord for Ipv4Range {
    fn cmp(&self, other: &Ipv4Range) -> cmp::Ordering {
        self.start_ip.cmp(&other.start_ip)
    }
}

impl fmt::Display for Ipv4Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.start_ip, self.end_ip)
    }
}

impl Ipv4Range {
    pub fn new(start_ip: Ipv4Address, end_ip: Ipv4Address) -> Self {
        Ipv4Range { start_ip, end_ip }
    }
    
    pub fn with_nums(start_ip: Ipv4Address, nums: u32) -> Self {
        let end_ip_number = u32::from(Ipv4Addr::from(start_ip.0)) + (nums - 1);
        let end_ip = Ipv4Address( Ipv4Addr::from(end_ip_number).octets() );

        Ipv4Range { start_ip, end_ip }
    }

    pub fn first(&self) -> Ipv4Address {
        self.start_ip
    }

    pub fn last(&self) -> Ipv4Address {
        self.end_ip
    }

    pub fn total(&self) -> u32 {
        u32::from(Ipv4Addr::from(self.end_ip.0)) - u32::from(Ipv4Addr::from(self.start_ip.0)) + 1
    }

    pub fn addrs(&self) -> Ipv4AddrsIter {
        Ipv4AddrsIter {
            offset: u32::from(Ipv4Addr::from(self.start_ip.0)) as u64,
            end   : u32::from(Ipv4Addr::from(self.end_ip.0)) as u64,
        }
    }

    pub fn cidrs(&self) -> Ipv4CidrIter {
        Ipv4CidrIter {
            start: u32::from(Ipv4Addr::from(self.start_ip.0)) as u64,
            end  : u32::from(Ipv4Addr::from(self.end_ip.0)) as u64,
        }
    }
}

pub struct Ipv4AddrsIter {
    offset: u64,
    end: u64,
}

impl Iterator for Ipv4AddrsIter {
    type Item = Ipv4Address;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end >= self.offset {
            let ip = Ipv4Addr::from(self.offset as u32);
            self.offset += 1;
            Some(Ipv4Address(ip.octets()))
        } else {
            None
        }
    }
}

pub struct Ipv4CidrIter {
    start: u64,
    end  : u64,
}

impl Iterator for Ipv4CidrIter {
    type Item = Ipv4Cidr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end > self.start {
            let mut shift = cmp::min(32, self.start.trailing_zeros());
            let num: u64;

            loop {
                let n = 2u64.pow(shift);
                if self.start + n > self.end + 1 {
                    if shift == 0 {
                        panic!("oops ...")
                    }
                    shift -= 1;
                } else {
                    num = n;
                    break;
                }
            }
            let prefix_len = 32 - shift;
            let ip = Ipv4Addr::from(self.start as u32);
            let cidr = Ipv4Cidr::new(Ipv4Address(ip.octets()), prefix_len as u8);
            self.start += num;
            Some(cidr)
        } else if self.end == self.start {
            let ip = Ipv4Addr::from(self.end as u32);
            let cidr = Ipv4Cidr::new(Ipv4Address(ip.octets()), 32);
            self.start += 1;
            Some(cidr)
        } else {
            None
        }
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IpBlock {
    Ipv4Range(Ipv4Range),
    Ipv4Cidr(Ipv4Cidr),
    Ipv6Cidr(Ipv6Cidr),
}

impl IpBlock {
    pub fn first(&self) -> IpAddress {
        match *self {
            IpBlock::Ipv4Range(v4_range) => IpAddress::Ipv4(v4_range.first()),
            IpBlock::Ipv4Cidr(v4_cidr) => {
                IpAddress::from(v4_cidr.network().address())
            },
            IpBlock::Ipv6Cidr(v6_cidr) => {
                IpAddress::from(v6_cidr.address())
            },
        }
    }

    pub fn last(&self) -> IpAddress {
        match *self {
            IpBlock::Ipv4Range(v4_range) => IpAddress::Ipv4(v4_range.last()),
            IpBlock::Ipv4Cidr(v4_cidr) => {
                
                let first_number = match self.first() {
                    IpAddress::Ipv4(v4_addr) => u32::from(Ipv4Addr::from(v4_addr)),
                    IpAddress::Ipv6(_) => unreachable!(),
                    _ => unreachable!()
                };

                let max = first_number + 2u32.pow(v4_cidr.prefix_len() as u32);
                IpAddress::from(Ipv4Address::from(Ipv4Addr::from(max)))
            },
            IpBlock::Ipv6Cidr(v6_cidr) => {
                let first_number = match self.first() {
                    IpAddress::Ipv4(_) => unreachable!(),
                    IpAddress::Ipv6(v6_addr) => u128::from(Ipv6Addr::from(v6_addr)),
                    _ => unreachable!()
                };

                let max = first_number + 2u128.pow(v6_cidr.prefix_len() as u32);
                IpAddress::from(Ipv6Address::from(Ipv6Addr::from(max)))
            },
        }
    }

    pub fn is_ipv4(&self) -> bool {
        match *self {
            IpBlock::Ipv4Range(_) | IpBlock::Ipv4Cidr(_) => true,
            IpBlock::Ipv6Cidr(_) => false,
        }
    }

    pub fn is_ipv6(&self) -> bool {
        match *self {
            IpBlock::Ipv4Range(_) | IpBlock::Ipv4Cidr(_) => false,
            IpBlock::Ipv6Cidr(_) => true,
        }
    }
}


impl fmt::Display for IpBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpBlock::Ipv4Range(v4_range) => fmt::Display::fmt(&v4_range, f),
            IpBlock::Ipv4Cidr(v4_cidr) => fmt::Display::fmt(&v4_cidr, f),
            IpBlock::Ipv6Cidr(v6_cidr) => fmt::Display::fmt(&v6_cidr, f),
        }
    }
}


#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct Record {
    pub src_registry: Registry,
    pub country: Country,
    pub ip_block: IpBlock,
    pub status: Status,
    pub dst_registry: Option<Registry>,
}

impl Record {
    pub fn src_registry(&self) -> Registry {
        self.src_registry
    }

    pub fn country(&self) -> Country {
        self.country
    }

    pub fn type_(&self) -> String {
        if self.is_ipv4() {
            "ipv4".to_string()
        } else if self.is_ipv6() {
            "ipv6".to_string()
        } else {
            unreachable!()
        }
    }

    pub fn ip_version(&self) -> String {
        self.type_()
    }

    pub fn ip_block(&self) -> IpBlock {
        self.ip_block
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn dst_registry(&self) -> Option<Registry> {
        self.dst_registry
    }

    pub fn is_ipv4(&self) -> bool {
        self.ip_block.is_ipv4()
    }

    pub fn is_ipv6(&self) -> bool {
        self.ip_block.is_ipv6()
    }

    pub fn codegen(&self) -> String {
        let first_ip = self.ip_block.first();
        let last_ip = self.ip_block.last();
        
        let ip_to_number_string = |ipaddr| -> String {
            match ipaddr {
                IpAddress::Ipv4(v4_addr) => format!("{}", u32::from(Ipv4Addr::from(v4_addr))),
                IpAddress::Ipv6(v6_addr) => format!("{}", u128::from(Ipv6Addr::from(v6_addr.0))),
                _ => unreachable!()
            }
        };
        
        format!("({}, {}, {})",
                ip_to_number_string(first_ip),
                ip_to_number_string(last_ip),
                self.country.index())
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> cmp::Ordering {
        self.ip_block.first().cmp(&other.ip_block.first())
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.ip_block.first() == other.ip_block.first()
    }
}


impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {}",
            self.src_registry,
            self.country,
            self.type_(),
            match self.ip_block {
                IpBlock::Ipv4Range(v4_range) => format!("{} {}", v4_range.first(), v4_range.total()),
                IpBlock::Ipv4Cidr(v4_cidr) => format!("{} {}", v4_cidr.address(), v4_cidr.prefix_len()),
                IpBlock::Ipv6Cidr(v6_cidr) => format!("{} {}", v6_cidr.address(), v6_cidr.prefix_len()),
            },
            self.status,
            match self.dst_registry {
                Some(reg) => format!("{}", reg),
                None => "none".to_string()
            })
    }
}

impl FromStr for Record {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 4.3 Record format
        // https://www.apnic.net/about-apnic/corporate-documents/documents/resource-guidelines/rir-statistics-exchange-format/#RecordFormat
        // 
        // Format:
        // 
        //      registry | cc | type | start | value | date | status [ | extensions ... ]
        let fields: Vec<&str> = s.split("|").collect();
        
        if fields.len() < 7 {
            // Less than 7 fields length of this RIR Record
            return Err(ParseError::Truncated);
        }
        
        let src_registry = Registry::from_str(fields[0])?;
        let cc = if fields[1].trim() == "" { "ZZ" } else { fields[1] };
        let country_code = Country::from_str(cc)?;
        let type_  = fields[2];

        match type_ {
            "ipv4" => {
                let start: Ipv4Addr = fields[3].parse().map_err(|_| ParseError::Unrecognized)?;
                let start_ip = Ipv4Address(start.octets());
                let nums: u32 = fields[4].parse().map_err(|_| ParseError::Unrecognized)?;

                assert!(nums > 0);

                let status_ = fields[6];
                let (status, dst_registry) = if src_registry == Registry::Iana {
                    (Status::Assigned, Some(Registry::from_str(fields[7])?))
                } else {
                    (Status::from_str(status_)?, None)
                };

                let ip_block = IpBlock::Ipv4Range(Ipv4Range::with_nums(start_ip, nums));

                let record = Record {
                    src_registry: src_registry,
                    country: country_code,
                    ip_block: ip_block,
                    status: status,
                    dst_registry: dst_registry
                };

                Ok(record)
            }
            "ipv6" => {
                let start: Ipv6Addr = fields[3].parse().map_err(|_| ParseError::Unrecognized)?;
                let start_ip = Ipv6Address(start.octets());
                let prefix_len: u8 = fields[4].parse().map_err(|_| ParseError::Unrecognized)?;

                assert!(prefix_len <= 128);

                let status_ = fields[6];
                let (status, dst_registry) = if src_registry == Registry::Iana {
                    (Status::Assigned, Some(Registry::from_str(fields[7])?))
                } else {
                    (Status::from_str(status_)?, None)
                };

                let ip_block = IpBlock::Ipv6Cidr(Ipv6Cidr::new(start_ip, prefix_len));

                let record = Record {
                    src_registry: src_registry,
                    country: country_code,
                    ip_block: ip_block,
                    status: status,
                    dst_registry: dst_registry
                };

                Ok(record)
            }
            _ => {
                // Not an IPv4 or IPv6 Record Line.
                Err(ParseError::Dropped)
            }
        }
    }
}



fn parse(data_path: &PathBuf) -> Result<HashSet<Record>, Box<dyn std::error::Error>> {
    let mut records: HashSet<Record> = HashSet::new();

    let filepaths: Vec<PathBuf> = IANA_RIR_FILES.iter()
                                                .map(|&(filename, _)| data_path.join(filename) )
                                                .collect();

    for filepath in filepaths {
        println!("parse file {:?}", filepath);

        if !filepath.exists() || !filepath.is_file() {
            continue;
        }
        
        let file_content = {
            let mut file = File::open(&filepath)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            content
        };
        
        let mut line_idx = 0usize;

        for line in file_content.lines() {
            if line.starts_with("#") {
                // Ignore comment line
                continue;
            }

            if line_idx == 0 || line.ends_with("summary") {
                // Ignore summary
                line_idx += 1;
                continue;
            }
            
            match Record::from_str(line) {
                Ok(record) => {
                    records.insert(record);
                },
                Err(e) => {
                    match e {
                        ParseError::Dropped => {

                        },
                        _ => {
                            let fields: Vec<&str> = line.split("|").collect();
                            println!("Line: {:?}", fields);
                            println!("Error: {:?}", e);
                            return Err(Box::new(e));
                        }
                    }
                }
            }

            line_idx += 1;
        }
    }

    Ok(records)
}


fn boot() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("o", "data-path", "Specify the default data path", "");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", program);
        print!("{}", opts.usage(&brief));

        std::process::exit(1);
    }

    let value = matches.opt_str("o").unwrap_or("data".to_string());

    Path::new(value.to_lowercase().as_str()).to_path_buf()
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let data_path = boot();
    
    if !data_path.exists() {
        println!("[ERROR] data path not exists.");
        std::process::exit(1);
    }

    let record_sets = parse(&data_path)?;

    let mut v4_records: Vec<&Record> = record_sets.iter().filter(|record| {
        record.is_ipv4() && record.src_registry() != Registry::Iana
    } ).collect();
    let mut v6_records: Vec<&Record> = record_sets.iter().filter(|record| {
        record.is_ipv6() && record.src_registry() != Registry::Iana
    } ).collect();

    let mut iana_v4_records: Vec<&Record> = record_sets.iter().filter(|record| {
        record.is_ipv4() && record.dst_registry().is_some() && record.src_registry() == Registry::Iana
    } ).collect();
    let mut iana_v6_records: Vec<&Record> = record_sets.iter().filter(|record| {
        record.is_ipv6() && record.dst_registry().is_some() && record.src_registry() == Registry::Iana
    } ).collect();

    v4_records.sort_unstable();
    v6_records.sort_unstable();
    iana_v4_records.sort_unstable();
    iana_v6_records.sort_unstable();

    let v4_output_filepath = data_path.join("v4_records");
    let v6_output_filepath = data_path.join("v6_records");
    let iana_v4_output_filepath = data_path.join("iana_v4_records");
    let iana_v6_output_filepath = data_path.join("iana_v6_records");

    let v4_db_filepath = "src/v4_db.rs";
    let v6_db_filepath = "src/v6_db.rs";

    let _ = fs::remove_file(&v4_output_filepath);
    let _ = fs::remove_file(&v6_output_filepath);
    let _ = fs::remove_file(&iana_v4_output_filepath);
    let _ = fs::remove_file(&iana_v6_output_filepath);

    let _ = fs::remove_file(&v4_db_filepath);
    let _ = fs::remove_file(&v6_db_filepath);

    let mut v4_output_file = OpenOptions::new().create(true).write(true).append(true)
                        .open(&v4_output_filepath)?;
    let mut v6_output_file = OpenOptions::new().create(true).write(true).append(true)
                        .open(&v6_output_filepath)?;
    let mut iana_v4_output_file = OpenOptions::new().create(true).write(true).append(true)
                        .open(&iana_v4_output_filepath)?;
    let mut iana_v6_output_file = OpenOptions::new().create(true).write(true).append(true)
                        .open(&iana_v6_output_filepath)?;
    
    let mut v4_db_file = OpenOptions::new().create(true).write(true).append(true)
                        .open(&v4_db_filepath)?;
    let mut v6_db_file = OpenOptions::new().create(true).write(true).append(true)
                        .open(&v6_db_filepath)?;

    for record in v4_records.iter() {
        v4_output_file.write(format!("{}\n", record).as_bytes())?;
    }
    for record in v6_records.iter() {
        v6_output_file.write(format!("{}\n", record).as_bytes())?;
    }

    for record in iana_v4_records.iter() {
        iana_v4_output_file.write(format!("{}\n", record).as_bytes())?;
    }
    for record in iana_v6_records.iter() {
        iana_v6_output_file.write(format!("{}\n", record).as_bytes())?;
    }

    let v4_db = v4_records.iter().map(|record| format!("    {}", record.codegen()) ).collect::<Vec<String>>();
    let v6_db = v6_records.iter().map(|record| format!("    {}", record.codegen()) ).collect::<Vec<String>>();

    v4_db_file.write(b"// Format: (first_ip, last_ip, country_index)\n")?;
    v4_db_file.write(b"#[doc(hidden)]\n")?;
    v4_db_file.write(format!("pub static IPV4_RECORDS: [(u32, u32, u8); {}] = [\n{}\n];",
                                v4_db.len(),
                                v4_db.join(",\n"))
                                    .as_bytes())?;

    v6_db_file.write(b"// Format: (first_ip, last_ip, country_index)\n")?;
    v6_db_file.write(b"#[doc(hidden)]\n")?;
    v6_db_file.write(format!("pub static IPV6_RECORDS: [(u128, u128, u8); {}] = [\n{}\n];",
                                v6_db.len(),
                                v6_db.join(",\n"))
                                    .as_bytes())?;

    Ok(())
}
