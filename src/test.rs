extern crate git2;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

use git2::{BlameOptions, Repository};
use hyper::Client;
use hyper::header::{Authorization, Bearer, Headers};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;
use serde_json::value::Map;
use std::io::Read;
use std::path::Path;


fn blame() -> Result<(), git2::Error> {
    let repo = match Repository::open("/home/randy/chewse") {
        Ok(value) => {value},
        Err(_) => panic!("Couldn't open repo.")
    };
    let path = Path::new("chewse/media/js/dispatcher/components/order.js");

    let commit_range = "ce3e3044f76aa5c369cafb8d4aedc45a8d7b0102";
    let revspec = repo.revparse(&commit_range)?;
    println!("{:?}", revspec.mode());

    let mut options = BlameOptions::new();
    if let (Some(oldest_commit), Some(newest_commit)) = (revspec.from(), revspec.to()) {
        options
            .oldest_commit(oldest_commit.id())
            .newest_commit(newest_commit.id());
    }
    let blame = match repo.blame_file(&path, None) {
        Ok(value) => value,
        Err(_) => panic!("Unable to blame path")
    };
    let num_lines = blame.len();
    println!("num lines: {}", num_lines);
    for index in 0..num_lines {
        if let Some(hunk) = blame.get_index(index) {
            println!(
                "{} - {} - {}",
                index + 1,
                hunk.orig_commit_id(),
                hunk.orig_signature()
            );
        }
    }
    Ok(())
}


fn get_traceback(url: &str) -> hyper::Result<String> {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut headers = Headers::new();
    headers.set(
        Authorization(
            Bearer{
                token: "<token>".to_owned()
            }
        )
    );
    let mut response = client
        .get(url)
        .headers(headers)
        .send()?;
    let mut buf = String::new();
    println!("{}", response.status);
    response.read_to_string(&mut buf)?;
    //    println!("{}", buf);
    Ok(buf)
}


fn to_json_obj (str: String)  {
    let data: Value = serde_json::from_str(&str).unwrap();
    let obj = data.is_array();

    println!("{:?}", obj)
}


fn main() {
    //    match blame() {
    //        Ok(_) => {},
    //        Err(r) => println!("error {}", r)
    //    }
    let str = match get_traceback("https://sentry.io/api/0/issues/176478469/events/") {
        Ok(str) => str,
        Err(r) => panic!("{}", r)
    };
    //    match to_json_obj(str) {
    //        Some(obj) => println!("{:?}", &obj),
    //        None => println!("No json object")
    //    }
    //    println!("{:?}", to_json_obj(str));
    to_json_obj(str)
}

