use hyper::{Body, Response};
use serde_json;
use std::fs::read_dir;

use crate::state::State;

pub fn serve_gpx(uri: &String, state: &State) -> Response<Body> {
    if uri.starts_with("/api/gpx/get") {
        // TODO finish code
        Response::new(Body::from("gpx get"))
    } else if uri.starts_with("/api/gpx/save") {
        // TODO finish code
        Response::new(Body::from("gpx save"))
    } else if uri == "/api/gpx/" {
        match read_dir(&state.config.gpx_base) {
            Err(_why) => {
                Response::new(Body::from("gpx list"))
            },
            Ok(paths) => {
                // TODO finish code
                let paths : Vec<String> = paths.map(|v| v.unwrap().file_name().to_str().unwrap().to_string()).collect();
                println!("{:?}", serde_json::to_string(&paths).unwrap());
                Response::new(Body::from("gpx list"))
            }
        }
    } else {
        Response::new(Body::from(format!("ERROR: request not recognized: {}", uri)))
    }
}

