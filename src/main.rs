#[macro_use]
extern crate nickel;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use nickel::{Nickel, HttpRouter, FormBody, JsonBody};
use nickel::status::StatusCode;
use rustc_serialize::json::{Json, ToJson};

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Event {
    id: u64,
    extra: String,
}

impl ToJson for Event {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), self.id.to_json());
        map.insert("extra".to_string(), self.extra.to_json());
        Json::Object(map)
    }
}

fn main () {
    let mut server = Nickel::new();

    server.post("/", middleware! { |request, response|
        let payload = try_with!(
            response, {
                request.json_as::<Event>()
                    .map_err(|e| (StatusCode::BadRequest, e))
            }
        );
        println!("{:?}", payload);
        format!("{:?}", payload)
    });

    server.listen("127.0.0.1:8999").unwrap();
}
