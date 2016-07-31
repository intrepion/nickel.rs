#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!("Goodbye World"));
    server.listen("127.0.0.1:6767").unwrap();
}
