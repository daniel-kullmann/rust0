use iron::prelude::*;
use iron::status;
use iron::url::percent_encoding::percent_decode;
use serde_json;
use std::fs::{File, read_dir};
use std::io::prelude::*;

use crate::state::State;
use crate::util::{content_type_json, content_type_xml, handle_error};

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    name: String,
    date: String,
    description: String,
    track_points: Vec<(f64, f64)>
}

pub fn serve_gpx(req: &mut Request, uri: &String, state: &State) -> IronResult<Response> {
    if uri.starts_with("/api/gpx/get/") {
        serve_get_gpx(uri, state)
    } else if uri.starts_with("/api/gpx/save") {
        serve_save_gpx(req, state)
    } else if uri == "/api/gpx/" {
        serve_list_gpx(state)
    } else {
        Ok(Response::with((status::NotFound, format!("ERROR: request not recognized: {}", uri))))
    }
}

fn serve_list_gpx(state: &State) -> IronResult<Response> {
    match list_gpx(&state.config.gpx_base) {
        Ok(json) => {
            let content_type = content_type_json();
            Ok(Response::with((content_type, status::Ok, json)))
        },
        Err(why) => handle_error(status::NotFound, &why),
    }
}

fn serve_get_gpx(uri: &String, state: &State) -> IronResult<Response> {
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
                    let content_type = content_type_xml();
                    Ok(Response::with((content_type, status::Ok, content)))
                }
            }
        }
    }
}

fn serve_save_gpx(req: &mut Request, state: &State) -> IronResult<Response> {
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
                            let content_type = content_type_json();
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

fn list_gpx(gpx_base: &String) -> Result<String, std::io::Error> {
    match read_dir(gpx_base) {
        Err(why) => Err(why),
        Ok(paths) => {
            let mut paths : Vec<String> = paths
                .map(|v| {
                    v.unwrap()
                        .file_name()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect();
            paths.sort_unstable();
            let json = serde_json::to_string(&paths).unwrap();
            Ok(json)
        }
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

#[cfg(test)]
mod tests {

    use super::{Track, create_gpx_content, list_gpx};

    #[test]
    fn test_list_gpx_1() {

        let result = list_gpx(&String::from("./data/gpx/"));
        match result {
            Ok(result) => assert_eq!(result, "[\"2018-11-28T12:00:00.000Z-first-long-hike-in-aljezur-area.gpx\",\"2018-12-15T11:37:32.045Z-walk.with-kids-in-amoreira.gpx\",\"2018-12-15T18:43:03.923Z-walk-between-monteclerigo-and-arrifana.gpx\",\"2018-12-16T22:45:32.227Z-schiefes-quadrat.gpx\",\"2019-01-12T16:00:00.078Z-Wanderung-das-Tal-hoch-Rückweg.gpx\",\"2019-01-12T16:00:00.078Z-aaaa.gpx\"]"),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn test_list_gpx_2() {

        let result = list_gpx(&String::from("./data/empty"));
        match result {
            Ok(result) => assert_eq!(result, "[]"),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn test_create_gpx_content() {
        let track = Track {
            name: String::from("a name"),
            date: String::from("a date"),
            description: String::from("a description"),
            track_points: vec!((1.4, 2.3), (3.2, 4.1))
        };
        let content = create_gpx_content(&track);
        assert_eq!(content, String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gpx creator=\"simple-offline-rust-map\" version=\"1.1\" xmlns=\"http://www.topografix.com/GPX/1/1\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd\">\n  <metadata>\n    <name>a name</name>\n    <desc>a description</desc>\n    <time>a date</time>\n  </metadata>\n  <trk>\n    <name>a name</name>\n    <desc>a description</desc>\n    <trkseg>\n      <trkpt lat=\"1.4\" lon=\"2.3\"></trkpt>\n      <trkpt lat=\"3.2\" lon=\"4.1\"></trkpt>\n    </trkseg>\n  </trk>\n</gpx>\n"));
    }
}
