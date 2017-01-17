extern crate iron;
extern crate router;
extern crate persistent;
extern crate postgres;

use iron::prelude::*;
use iron::status;
use persistent::*;
use router::Router;

#[derive(Copy, Clone)]
pub struct PGConn;


struct Filebox {
}

impl Filebox {
    pub fn new(cfgfile: &String) -> Filebox {
        return Filebox {
        };
    }

    pub fn handler(req: &mut Request) -> IronResult<Response> {
        return Ok(Response::with((status::Ok, "Hello, world!")));
    }
}

fn main() {
    let mut router = Router::new();
    let mut filebox = Filebox::new(&String::from("11"));

    router.get("/", Filebox::handler, "index");
    router.post("/files/", Filebox::handler, "upload");
    router.get("/files/:file/", Filebox::handler, "file_info");
    router.get("/files/:file/file", Filebox::handler, "download");

    let mut chain = Chain::new(router);

    Iron::new(chain).http("localhost:3000").unwrap();
}
