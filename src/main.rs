extern crate hyper;
extern crate ini;
extern crate serde;
//#[macro_use]
//extern crate serde_derive;
extern crate serde_json;
extern crate shellexpand;
extern crate rust0;

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
//use serde_json::{Value, Error};
use reqwest;
//use serde_json::Error;
use std::ops::Deref;
use std::sync::Arc;
use r2d2_sqlite::SqliteConnectionManager;

use rust0::config::get_config;
use rust0::api::gpx::serve_gpx;
use rust0::state::State;
use rust0::tiles::serve_tile;

//#[derive(Serialize, Deserialize)]



fn serve(req: Request<Body>, state: &State) -> Response<Body> {
    let uri = req.uri().to_string();
    if uri.starts_with("/tiles") {
        serve_tile(&uri, state)
    } else if uri.starts_with("/api/gpx/") {
        serve_gpx(&uri, state)
    } else if uri.starts_with("/api/settings/") {
        Response::new(Body::from("settings"))
    } else {
        let mut response = Response::builder();
        let response = response.status(StatusCode::NOT_FOUND);
        response.body(Body::from("404 not found")).unwrap()
    }
}

fn main() {

    let config = Arc::new(get_config());

    let manager = SqliteConnectionManager::file(&config.db_file);
    let pool = r2d2::Pool::new(manager).unwrap();

    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    //    let service = MapService::new();
    let service = move || {
        let pool = pool.clone();
        let config = config.clone();
        service_fn_ok(move |req| {
            let state = State {
                config: &config.deref(),
                connection: pool.get().unwrap()
            };
            serve(req, &state)
        })
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("server started at http://localhost:3000");

    // Run this server for... forever!
    hyper::rt::run(server);

}
