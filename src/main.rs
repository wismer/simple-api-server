extern crate iron;
extern crate hyper;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use std::io::Read;
use std::fs::File;
use hyper::header::{Headers, ContentType};

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

    fn four_oh_four(&self, body: &mut String) {
        body.clear();
        body.push_str("404 RESOURCE NOT FOUND");
    }

    fn panic(&self) {
        panic!("WHAT");
    }

    fn card_route(&self, body: &mut String, url: &mut Vec<String>) {
        if url.len() == 0 {
            let mut cards = File::open("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json").unwrap();
            cards.read_to_string(body);
        } else {
            let mut next_path = url.pop();
            while let Some(path) = next_path {
                let id = path.parse::<i32>();
                if id.is_ok() {
                    body.push_str("an id was pushed in");
                } else {
                    self.four_oh_four(body);
                    break;
                }

                next_path = url.pop();
            }
        }
    }

    fn get_contents_from_route(&self, body: &mut String, url: &Vec<String>) {
        let mut path = url.clone();
        path.reverse();
        let root = path.pop().unwrap();

        match root.as_ref() {
            "favicon.ico" => println!("favi"),
            "cards" => self.card_route(body, &mut path),
            "stuff" => println!("asdakjfsldkfjdslfj"),
            _ => println!("{}", root)
        }

        {}
    }
}

// request comes in. /cards/night-steed
// break it apart -> "cards" "night-steed"
// "cards" gets the type, so it will know that the following url part is a slug
// the slug is used as a key in a hashmap that will then return the data as json

fn main() {
    // let mut headers = Headers::new();
    // let json_application: ContentType = ContentType::json();
    // headers.set(json_application);


    Iron::new( |req: &mut Request| {
        let url = &req.url.path;
        let mut body = String::new();
        let router = Router::new();
        let mut response = Response::new();
        let json_application: ContentType = ContentType::json();
        req.headers.set(json_application.clone());
        response.headers.set(json_application);
        router.get_contents_from_route(&mut body, url);
        Ok(response.set(body))
    }).http("localhost:3000").unwrap();
}
