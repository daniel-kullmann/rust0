use hyper::{Body, Response, StatusCode};
use std::fs::read_dir;

use crate::state::State;

pub fn serve_gpx(uri: &String, state: &State) -> Response<Body> {
    if uri.starts_with("/api/gpx/get") {
        Response::new(Body::from("gpx get"))
    } else if uri.starts_with("/api/gpx/save") {
        Response::new(Body::from("gpx save"))
    } else if uri == "/api/gpx/" {
        match read_dir(&state.config.gpx_base) {
            Err(_why) => {
                Response::new(Body::from("gpx list"))
            },
            Ok(paths) => {
                //let paths_slice : &[&str] = paths.map(|v| v.unwrap().path().file_name()).into();
                //let _a: Vec<&str> = paths.map(|v| v.unwrap().path().file_name().unwrap().to_str().unwrap()).collect();
                //for path in paths {
                //    println!("{}", path.unwrap().path().file_name().and_then(|v| v.to_str()).unwrap());
                //}
                Response::new(Body::from("gpx list"))
            }
        }
    } else {
        Response::new(Body::from(format!("ERROR: request not recognized: {}", uri)))
    }
}

