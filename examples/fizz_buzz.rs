#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate intrepion_x_fizz_buzz;

use std::collections::BTreeMap;
use nickel::status::StatusCode;
use nickel::{Nickel, JsonBody, HttpRouter};
use rustc_serialize::json::{Json, ToJson};
use intrepion_x_fizz_buzz::fizz_buzz;

#[derive(RustcDecodable, RustcEncodable)]
struct Number {
    value: String,
}

#[derive(RustcDecodable, RustcEncodable)]
struct FizzBuzz {
    fizz_buzz: String,
}

impl ToJson for Number {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("value".to_string(), self.value.to_json());
        Json::Object(map)
    }
}

impl ToJson for FizzBuzz {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("fizz_buzz".to_string(), self.fizz_buzz.to_json());
        Json::Object(map)
    }
}

fn main() {
    let mut server = Nickel::new();

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    server.post("/", middleware! { |request, response|
        let number = try_with!(response, {
            request.json_as::<Number>().map_err(|e| (StatusCode::BadRequest, e))
        });
        let fizz_buzz = FizzBuzz {
            fizz_buzz: fizz_buzz(number.value.parse::<i64>().unwrap()).to_string(),
        };
        fizz_buzz.to_json()
    });

    server.listen("127.0.0.1:6767").unwrap();
}
