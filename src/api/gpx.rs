use hyper::{Body, Request, Response, StatusCode};
use serde_json;
use std::fs::{File, read_dir};
use std::io::prelude::*;

use crate::state::State;

pub fn serve_gpx(req: &Request<Body>, uri: &String, state: &State) -> Response<Body> {
    if uri.starts_with("/api/gpx/get/") {
        let file_name = &uri[13..];
        let full_file = format!("{}/{}", &state.config.gpx_base, file_name);
        println!("{:?}", full_file);
        let fh = File::open(full_file);
        match fh {
            Err(why) => {
                println!("{:?}", why);
                let mut response = Response::builder();
                response.status(StatusCode::NOT_FOUND);
                Response::new(Body::from(""))
            },
            Ok(mut fh) => {
                let mut content = String::new();
                match fh.read_to_string(&mut content) {
                    Ok(_) => {
                        let mut response = Response::builder();
                        response.header("Content-Type", "text/xml").status(StatusCode::OK);
                        response.body(Body::from(content)).unwrap()
                    },
                    Err(why) => {
                        println!("{:?}", why);
                        let mut response = Response::builder();
                        response.status(StatusCode::NOT_FOUND);
                        Response::new(Body::from(""))
                    }
                }
            }
        }
    } else if uri.starts_with("/api/gpx/save") {
        // TODO finish code
        println!("{:?}", req);
        Response::new(Body::from("gpx save"))
    } else if uri == "/api/gpx/" {
        match read_dir(&state.config.gpx_base) {
            Err(why) => {
                println!("{:?}", why);
                let mut response = Response::builder();
                response.status(StatusCode::NOT_FOUND);
                Response::new(Body::from(""))
            },
            Ok(paths) => {
                // TODO finish code
                let paths : Vec<String> = paths.map(|v| v.unwrap().file_name().to_str().unwrap().to_string()).collect();
                let json = serde_json::to_string(&paths).unwrap();
                let mut response = Response::builder();
                response.header("Content-Type", "application/json").status(StatusCode::OK);
                response.body(Body::from(json)).unwrap()
            }
        }
    } else {
        Response::new(Body::from(format!("ERROR: request not recognized: {}", uri)))
    }
}

