#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate iron;

use iron::prelude::*;
use iron::status;

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    status: String,
    content: String,
}

fn main() {
    fn greet_visitor(_: &mut Request) -> IronResult<Response> {
        let resp = Greeting {
            status: "success".to_string(),
            content: "Hello from Rust-Trends.com - size optimization rust binaries".to_string(),
        };
        let payload = serde_json::to_string(&resp).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    println!("Starting the Webserver on localhost:3000");
    Iron::new(greet_visitor).http("localhost:3000").unwrap();
}
