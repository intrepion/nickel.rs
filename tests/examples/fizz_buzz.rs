use hyper::client::Response;
use hyper::{header, mime};
use hyper::status::StatusCode;
use std::collections::HashMap;
use util::{read_body_to_string, response_for_post, run_example};
use rustc_serialize::json;

fn send_body<F>(body: &str, f: F) where F: FnOnce(&mut Response) {
    run_example("fizz_buzz", |port| {
        let url = format!("http://localhost:{}", port);
        let ref mut res = response_for_post(&url, body);
        f(res)
    })
}

#[test]
fn rejects_invalid() {
    // Missing 'value'
    let body = r#"{}"#;
    send_body(body, |res| {
        assert_eq!(res.status, StatusCode::BadRequest);
    })
}

#[test]
fn serializes_valid_json() {
    let body = r#"{ "value": "15" }"#;
    send_body(body, |res| {
        let s = read_body_to_string(res);
        let map: HashMap<String, String> = json::decode(&s).unwrap();
        assert_eq!(map["fizz_buzz"], "Fizz Buzz");
    })
}

#[test]
fn sets_content_type_header() {
  let body = r#"{ "value": "15" }"#;
  send_body(body, |res| {
      let content_type = res.headers.get::<header::ContentType>().unwrap();
      let expected: mime::Mime = "application/json".parse().unwrap();
      assert_eq!(content_type, &header::ContentType(expected));
  })
}
