use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use iron::url::percent_encoding::percent_decode;
use serde_json;
use std::fs::{File, read_dir};
use std::io::prelude::*;

use crate::state::State;
use crate::util::handle_error;

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    name: String,
    date: String,
    description: String,
    track_points: Vec<(f64, f64)>
}



pub fn serve_gpx(req: &mut Request, uri: &String, state: &State) -> IronResult<Response> {
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
        save_gpx(req, state)
    } else if uri == "/api/gpx/" {
        match read_dir(&state.config.gpx_base) {
            Err(why) => handle_error(status::NotFound, &why),
            Ok(paths) => {
                let mut paths : Vec<String> = paths.map(|v| v.unwrap().file_name().to_str().unwrap().to_string()).collect();
                paths.sort_unstable();
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
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => {
            let track: Result<Track, serde_json::Error> = serde_json::from_str(&body);
            match track {
                Ok(track) => {
                    let content = create_gpx_content(&track);
                    let file_name = format!("{}/{}-{}.gpx", state.config.gpx_base, track.date, track.name);
                    match File::create(&file_name) {
                        Err(why) => handle_error(status::NotFound, &why),
                        Ok(mut file) => {
                            file.write(content.as_bytes()).unwrap();
                            let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
                            Ok(Response::with((content_type, status::Ok, "[]")))
                        },
                    }
                },
                Err(err) => handle_error(status::NotFound, &err),
            }
        },
        Ok(None) => handle_error(status::NotFound, &"No body"),
        Err(err) => handle_error(status::NotFound, &err)
    }
}

fn create_gpx_content(track: &Track) -> String {
    let track_points = track.track_points.iter().map(|(lat, lon)| {
        format!("<trkpt lat=\"{}\" lon=\"{}\"></trkpt>", lat, lon)
    });
    let track_points: Vec<String> = track_points.collect();
    let track_points = track_points.join("\n      ");
    let mut content = String::new();
    content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    content.push_str("<gpx creator=\"simple-offline-rust-map\" version=\"1.1\" xmlns=\"http://www.topografix.com/GPX/1/1\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd\">\n");
    content.push_str("  <metadata>\n");
    content.push_str(&format!("    <name>{}</name>\n", track.name));
    content.push_str(&format!("    <desc>{}</desc>\n", track.description));
    content.push_str(&format!("    <time>{}</time>\n", track.date));
    content.push_str("  </metadata>\n");
    content.push_str("  <trk>\n");
    content.push_str(&format!("    <name>{}</name>\n", track.name));
    content.push_str(&format!("    <desc>{}</desc>\n", track.description));
    content.push_str("    <trkseg>\n");
    content.push_str(&format!("      {}", &track_points));
    content.push_str("\n    </trkseg>\n");
    content.push_str("  </trk>\n");
    content.push_str("</gpx>\n");
    content
}
