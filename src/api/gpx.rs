use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use iron::url::percent_encoding::percent_decode;
use serde_json;
use std::fs::{File, read_dir};
use std::io::prelude::*;

use crate::state::State;
use crate::util::handle_error;

pub fn serve_gpx(req: &Request, uri: &String, state: &State) -> IronResult<Response> {
    if uri.starts_with("/api/gpx/get/") {
        let file_name = &uri[13..];
        let file_name = percent_decode(file_name.as_bytes()).decode_utf8().unwrap();
        let full_file = format!("{}/{}", &state.config.gpx_base, file_name);
        let fh = File::open(full_file);
        match fh {
            Err(why) => handle_error(status::NotFound, &why),
            Ok(mut fh) => {
                let mut content = String::new();
                match fh.read_to_string(&mut content) {
                    Err(why) => handle_error(status::NotFound, &why),
                    Ok(_) => {
                        let content_type = "text/xml".parse::<Mime>().expect("Failed to parse content type");
                        Ok(Response::with((content_type, status::Ok, content)))
                    }
                }
            }
        }
    } else if uri.starts_with("/api/gpx/save") {
        // TODO finish code
        println!("{:?}", req);
        Ok(Response::with((status::Ok, "gpx save")))
    } else if uri == "/api/gpx/" {
        match read_dir(&state.config.gpx_base) {
            Err(why) => handle_error(status::NotFound, &why),
            Ok(paths) => {
                // TODO finish code
                let paths : Vec<String> = paths.map(|v| v.unwrap().file_name().to_str().unwrap().to_string()).collect();
                let json = serde_json::to_string(&paths).unwrap();
                let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
                Ok(Response::with((content_type, status::Ok, json)))
            }
        }
    } else {
        Ok(Response::with((status::NotFound, format!("ERROR: request not recognized: {}", uri))))
    }
}

