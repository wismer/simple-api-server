#![feature(custom_derive, plugin)]
#![feature(fs_walk)]

extern crate iron;
#[macro_use]
extern crate router;


extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::headers::AccessControlAllowOrigin;
use router::Router;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct ExpansionList {
    pub names: Vec<String>
}


fn main() {
    let router = router!(
        get "/expansions" => expansions,
        get "/cards" => cards
    );

    let mut chain = Chain::new(router);

    chain.link_after(|_: &mut Request, mut res: Response| {
        res.headers.set(AccessControlAllowOrigin::Any);
        Ok(res)
    });

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn expansions(_: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    let filename = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut body);
    let hearthstone: Value = serde_json::from_str(&body).unwrap();
    let mut json = "{\"data\":[".to_string();
    let mut idx: i32 = 0;
    let data: String = hearthstone.as_object().unwrap().keys().map(|x| {
        idx += 1;
        let name = format!("{}\"name\":\"{}\"{}", '{', x, '}');
        format!("{}\"type\":\"expansions\",\"id\":\"{}\",\"attributes\":{}{}", '{', idx, name, '}')
    }).collect::<Vec<_>>().join(",");
    // println!("{:?}", data);
    json.push_str(&data);
    json.push_str("]}");
    Ok(Response::with((status::Ok, json)))
}

fn cards(_: &mut Request) -> IronResult<Response> {
    let mut body = String::new();

    Ok(Response::with((status::Ok, "stuffffffff")))
}
