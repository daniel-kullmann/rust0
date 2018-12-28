extern crate clap;
extern crate hyper;
extern crate ini;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde_json;
extern crate shellexpand;

use std::fs;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
//use serde_json::{Value, Error};
use r2d2_sqlite::SqliteConnectionManager;
use reqwest;
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

mod config;

pub struct State<'a> {
    pub config: &'a config::FinalConfiguration,
    pub connection: r2d2::PooledConnection<SqliteConnectionManager>
}

fn serve_tile(uri: &String, state: &State) -> Response<Body> {
    match create_dir_all(Path::new(state.config.tile_base.as_str())) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }
    let rest: String = uri.chars().skip(7).collect();
    let full_file = format!("{}/{}", state.config.tile_base, rest);
    println!("file: {}", full_file);
    match File::open(&full_file) {
        Ok(mut file) => {
            let mut contents = vec![];
            match file.read_to_end(&mut contents) {
                Ok(_) => {
                    println!("INFO: Served {}", full_file);
                    Response::new(Body::from(contents))
                },
                Err(_) => {
                    Response::new(Body::from("ooh no!"))
                }
            }
        }
        Err(_) => {
            let parts: Vec<&str> = uri.split("/").collect();
            match parts.as_slice() {
                ["", "tiles", s, z, x, y] => {
                    let osm_url: String = format!("https://{}.tile.openstreetmap.org/{}/{}/{}", s, z, x, y);
                    println!("INFO: Fetch from OSM: {:?}", osm_url);
                    let response = reqwest::get(osm_url.as_str());
                    match response {
                        Ok(mut response) => {
                            let mut buf: Vec<u8> = vec![];
                            match File::create(&full_file) {
                                Err(why) => println!("ERROR: could not create tile file: {:?}", why),
                                Ok(mut file) => {
                                    match file.write_all(&buf[..]) {
                                        Err(why) => println!("ERROR: could not save tile file: {:?}", why),
                                        Ok(_) => ()
                                    }

                                }
                            };

                            match response.copy_to(&mut buf) {
                                Ok(_) => Response::new(Body::from(buf)),
                                Err(_err) => Response::new(Body::from("ERROR: could not copy"))
                            }
                        },
                        Err(err) => {
                            println!("{:?}", err);
                            Response::new(Body::from("TODO: get tile from osm"))
                        }
                    }
                },
                _ => {
                    Response::new(Body::from("ERROR: url wrong (get tile from osm)"))
                }
            }
        }
    }
}

fn serve_gpx(uri: &String, state: &State) -> Response<Body> {
    if uri.starts_with("/api/gpx/get") {
        Response::new(Body::from("gpx get"))
    } else if uri.starts_with("/api/gpx/save") {
        Response::new(Body::from("gpx save"))
    } else if uri == "/api/gpx/" {
        match fs::read_dir(&state.config.gpx_base) {
            Err(_why) => {
                Response::new(Body::from("gpx list"))
            },
            Ok(paths) => {
                for path in paths {
                    println!("{}", path.unwrap().path().file_name().and_then(|v| v.to_str()).unwrap());
                }
                Response::new(Body::from("gpx list"))
            }
        }
    } else {
        Response::new(Body::from(format!("ERROR: request not recognized: {}", uri)))
    }
}

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

    let config = Arc::new(config::get_config());

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
