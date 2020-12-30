extern crate getopts;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;

use hyper::Uri;
use hyper::Client;
use hyper::body::Bytes;
use hyper::body::to_bytes as body_to_bytes;
use hyper_tls::HttpsConnector;

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};


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

async fn fetch(uri: Uri) -> Result<Bytes, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(uri).await?;
    if res.status() != 200 {
        let e = io::Error::new(io::ErrorKind::Other, format!("Http Status Code: {:?}", res.status()));
        return Err(Box::new(e));
    }

    let body = body_to_bytes(res).await?;

    Ok(body)
}

async fn sync(data_path: PathBuf, filename: &str, fileurl: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fileuri = fileurl.parse::<Uri>()?;
    assert_eq!(fileuri.scheme_str(), Some("https"), "URL Scheme Not Supported.");

    let filepath = data_path.join(filename);
    let md5_filepath = data_path.join(format!("{}.md5", filename));
    let md5_fileuri  = (format!("{}.md5", fileuri)).parse::<Uri>()?;

    if filename != "delegated-iana-latest" {
        let mut old_md5_file_content = Vec::new();

        if !md5_filepath.exists() {
            File::create(&md5_filepath)?;
        } else {
            old_md5_file_content = std::fs::read(&md5_filepath)?;
        }

        let md5_file_content: Bytes = fetch(md5_fileuri).await?;

        if md5_file_content.is_empty() || md5_file_content != old_md5_file_content {
            // NOTE: 数据需要更新
            let content: Bytes = fetch(fileuri).await?;
            let mut file = OpenOptions::new().create(true).write(true).append(false).open(&filepath)?;
            file.write_all(&content)?;

            // 更新 MD5 校验文件
            let mut file = OpenOptions::new().create(false).write(true).append(false).open(&md5_filepath)?;
            file.write_all(&md5_file_content)?;
        } else {
            // NOTE: 数据不需要更新
            return Ok(());
        }
    } else {
        // NOTE: delegated-iana-latest 文件没有 MD5 校验码。
        let content: Bytes = fetch(fileuri).await?;
        let mut file = OpenOptions::new().create(true).write(true).append(false).open(&filepath)?;
        file.write_all(&content)?;
    }

    Ok(())
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

async fn run(data_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !data_path.exists() {
        fs::create_dir(&data_path)?;
    }
    
    println!("Data Path: {:?}", &data_path);
    println!();
    
    for rir_file in IANA_RIR_FILES.iter() {
        let filename  = rir_file.0;
        let fileurl   = rir_file.1;
        let data_path = data_path.clone();

        print!("sync {:34} ...", filename);

        let ret = sync(data_path, filename, fileurl).await;
        match ret {
            Ok(_) => {
                println!("    [\x1b[32mOK\x1b[0m]");
            },
            Err(e) => {
                println!("    [\x1b[31mFAILED\x1b[0m]  {:?}", e);
            }
        }
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_path = boot();
    
    let rt  = tokio::runtime::Runtime::new()?;
    rt.block_on(run(data_path))?;

    Ok(())
}
