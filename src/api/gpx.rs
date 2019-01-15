use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use iron::url::percent_encoding::percent_decode;
use serde_json;
use std::fs::{File, read_dir};
use std::io::prelude::*;

use crate::state::State;
use crate::util::handle_error;
use crate::util::json_value_to_string;

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

pub fn save_gpx(req: &mut Request, state: &State) -> IronResult<Response> {
    let body = req.get::<bodyparser::Json>();
    match body {
        Ok(Some(body)) => {
            println!("{:?}", body);
            match body {
                serde_json::Value::Object(map) => {
                    let mut content = String::new();
                    let name = json_value_to_string(&map["name"]);
                    let desc = json_value_to_string(&map["description"]);
                    let time = json_value_to_string(&map["date"]);
                    let track_points = if let serde_json::Value::Array(array) = &map["track_points"] {
                        array.iter().map(|v| {
                            if let serde_json::Value::Array(array) = v {
                                format!("<trkpt lat=\"{}\" lon=\"{}\"></trkpt>", array[0], array[1])
                            } else {
                                panic!("Wrong json request body")
                            }
                        })
                    } else {
                        panic!("Wrong json request body")
                    };
                    let track: Vec<String> = track_points.collect();
                    let track = track.join("\n      ");
                    content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
                    content.push_str("<gpx creator=\"maps0\" version=\"1.1\" xmlns=\"http://www.topografix.com/GPX/1/1\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd http://www.garmin.com/xmlschemas/TrackPointExtension/v1 http://www.garmin.com/xmlschemas/TrackPointExtensionv1.xsd http://www.garmin.com/xmlschemas/GpxExtensions/v3 http://www.garmin.com/xmlschemas/GpxExtensionsv3.xsd\" xmlns:gpxtpx=\"http://www.garmin.com/xmlschemas/TrackPointExtension/v1\" xmlns:gpxx=\"http://www.garmin.com/xmlschemas/GpxExtensions/v3\">\n");
                    content.push_str("  <metadata>\n");
                    content.push_str(&format!("    <name>{}</name>\n", name));
                    content.push_str(&format!("    <desc>{}</desc>\n", desc));
                    content.push_str(&format!("    <time>{}</time>\n", time));
                    content.push_str("  </metadata>\n");
                    content.push_str("  <trk>\n");
                    content.push_str(&format!("    <name>{}</name>\n", name));
                    content.push_str(&format!("    <desc>{}</desc>\n", desc));
                    content.push_str("    <trkseg>\n");
                    content.push_str(&format!("      {}", &track));
                    content.push_str("\n    </trkseg>\n");
                    content.push_str("  </trk>\n");
                    content.push_str("</gpx>\n");
	                  let file_name = format!("{}/{}-{}.gpx", state.config.gpx_base, time, name);
                    match File::create(&file_name) {
                        Err(why) => handle_error(status::NotFound, &why),
                        Ok(mut file) => {
                            file.write(content.as_bytes()).unwrap();
                            let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
                            Ok(Response::with((content_type, status::Ok, "[]")))
                        },
                    }
                }
                _ => handle_error(status::NotFound, &"Wrong json body")
            }
        },
        Ok(None) => handle_error(status::NotFound, &"No body"),
        Err(err) => handle_error(status::NotFound, &err)
    }
}
