extern crate hyper;

use std::fs::File;
use std::io::Write;
use std::io::Read;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;
use hyper::uri::RequestUri;

fn hello(req: Request, res: Response<Fresh>) {
    let Request { uri: uri, .. } = req;
    let mut msg = String::new();
    match uri {
        RequestUri::AbsolutePath(path) => msg.push_str(&path),
        RequestUri::AbsoluteUri(url) => println!("URL"),
        RequestUri::Authority(val) => println!("{}", val),
        RequestUri::Star => panic!("SDSKSDJFSDLKGJ")
    }
    let mut body: Vec<u8> = vec![];
    let file = File::open("/Users/Matt/Downloads/AllSets.json");
    let json = file.unwrap().read_to_end(&mut body);
    res.send(&body).unwrap();
}

fn main() {
    Server::http("127.0.0.1:3000").unwrap().handle(hello);
}
