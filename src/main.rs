extern crate clap;
extern crate hyper;
extern crate ini;
extern crate serde_json;
extern crate shellexpand;

use futures;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
//use serde_json::{Value, Error};
use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

mod config;


fn serve(req: Request<Body>, config: &config::FinalConfiguration) -> Response<Body> {
    println!("{:#?}", req.uri());
    let uri = req.uri().to_string();
    if uri.starts_with("/tiles") {
        match create_dir_all(Path::new(config.tile_base.as_str())) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e)
        }
        Response::new(Body::from("tiles"))
    } else if uri.starts_with("/api/gpx/") {
        Response::new(Body::from("gpx"))
    } else if uri.starts_with("/api/settings/") {
        Response::new(Body::from("settings"))
    } else {
        let mut response = Response::builder();
        response.status(StatusCode::NOT_FOUND);
        response.body(Body::empty()).unwrap()
    }
}

fn main() {

    let config = config::get_config();

    let state = Arc::new(config);

    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    //    let service = MapService::new();
    let service = move || {
        let state = state.clone();
        service_fn_ok(move |req| serve(req, state.deref()))
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("server started at http://localhost:3000");

    // Run this server for... forever!
    hyper::rt::run(server);

}
