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
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct ExpansionList {
    pub names: Vec<String>
}


fn main() {
    let router = router!(
        get "/expansions" => expansions,
        get "/cards" => cards,
        get "/expansions/:expansion_slug" => expansions
    );

    let mut chain = Chain::new(router);

    chain.link_after(|_: &mut Request, mut res: Response| {
        res.headers.set(AccessControlAllowOrigin::Any);
        Ok(res)
    });

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn expansion(req: &mut Request) -> IronResult<Response> {
    // let mut body = String::new();
    // let filename = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    // let mut file = File::open(filename).unwrap();
    // file.read_to_string(&mut body);w
    // let hearthstone: Value = serde_json::from_str(&body).unwrap();
    //
    // let expansion_list_slugged = hearthstone.as_object().unwrap().keys().map(|x|
    //     x.to_lowercase().replace(" ", "-")
    // ).collect::<Vec<_>>();
    Ok(Response::with((status::Ok, "json")))
}

fn expansions(req: &mut Request) -> IronResult<Response> {
    let expansion_slug = req.extensions.get::<Router>().unwrap().find("expansion_slug").unwrap_or("");
    println!("{}", expansion_slug);
    let mut body = String::new();
    let filename = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut body);
    let hearthstone: Value = serde_json::from_str(&body).unwrap();
    let name_slugs = hearthstone.as_object().unwrap().keys().map(|x| {
        let name = x.clone();
        (name.replace(" ", "-").to_lowercase(), x)
    }).collect::<Vec<_>>();
    let mut json = "{\"data\":".to_string();
    if expansion_slug.is_empty() {
    //     json.push_str("[");
    //     let mut idx: i32 = 0;
    //     let data: String = hearthstone.as_object().unwrap().keys().map(|x| {
    //         let mut slug = format!("\"slug\":\"{}\"", x.replace(" ", "-").to_lowercase());
    //         idx += 1;
    //         let name = format!("{}\"name\":\"{}\",{}{}", '{', x, slug, '}');
    //         format!("{}\"type\":\"expansions\",\"id\":\"{}\",\"attributes\":{}{}", '{', idx, name, '}')
    //     }).collect::<Vec<_>>().join(",");
    //     // println!("{:?}", data);
    //     json.push_str(&data);
    //     json.push_str("]}");
    //     Ok(Response::with((status::Ok, json)))
    // } else {
    //
    } else {
        let matched_slug = name_slugs.find(|&x| {
            let (slug, name) = x;
            slug == expansion_slug
        }).unwrap();
    }
    Ok(Response::with((status::Ok, "")))
}

fn decamelize(slug: &mut String) {
    // slug.replace("-", " ").split_whitespace().map(|x| {
    //     let mut chars = x.chars();
    //     let mut letter = chars.next().unwrap().to_uppercase
    // })
}

fn cards(_: &mut Request) -> IronResult<Response> {
    let mut body = String::new();

    Ok(Response::with((status::Ok, "stuffffffff")))
}
