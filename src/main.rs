extern crate iron;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use std::io::Read;
use std::fs::File;

// struct RoutePath<T> {
//     slug: T,
//     list: String
// }

struct Router;

enum RouteTree {
    Cards(i32),
    Expansions(String)
}

impl Router {
    fn new() -> Router {
        Router
    }

    fn cards(&self, path: &Vec<String>, body: &mut String) {
        let mut file = File::open("/Users/Matt/projects/ruby/hearthstone/data/AllSets.json").unwrap();
        if path.len() == 1 {
            file.read_to_string(body);
        }
    }

    fn expansions(&self, path: &Vec<String>, body: &mut String) {
        body.push_str("Expansions");
    }

    fn index(&self, body: &mut String) {
        body.push_str("INDEX");
    }

    fn panic(&self) {
        panic!("WHAT");
    }
}

// request comes in. /cards/night-steed
// break it apart -> "cards" "night-steed"
// "cards" gets the type, so it will know that the following url part is a slug
// the slug is used as a key in a hashmap that will then return the data as json


fn main() {
    Iron::new( |req: &mut Request| {
        let path = &req.url.path.first().unwrap();
        let router = Router::new();
        let mut body = String::new();
        match *path {
            "cards"      => router.cards(&req.url.path, &mut body),
            "expansions" => router.expansions(&req.url.path, &mut body),
            ""           => router.index(&mut body),
            _            => router.panic()
        }
        Ok(Response::with((status::Ok, body)))
    }).http("localhost:3000").unwrap();
}
