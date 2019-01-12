use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use mime_guess::guess_mime_type;

use crate::api::gpx::serve_gpx;
use crate::api::settings::{serve_get_all_settings, serve_set_all_settings};
use crate::files::file_content;
use crate::state::State;
use crate::tiles::serve_tile;
use crate::util::handle_error;

fn serve_static_content(uri: &String) -> IronResult<Response> {
    let uri: String = if uri == "/" { String::from("/index.html") } else  { uri.to_string() };
    let static_file = file_content(&uri);
    match static_file {
        Some(content) => {
            let mime_type = guess_mime_type(&uri).to_string();
            let content_type = mime_type.parse::<Mime>().expect("Failed to parse content type");
            println!("Serving static file {} as {}", uri, mime_type);
            Ok(Response::with((content_type, status::Ok, content)))
        }
        None => handle_error(status::NotFound, &uri),
    }
}

pub fn serve(req: &mut Request, state: &State) -> IronResult<Response> {
    let uri = "/".to_owned() + &req.url.path().join(&"/");
    if uri.starts_with("/tiles") {
        serve_tile(&uri, state)
    } else if uri.starts_with("/api/gpx/") {
        serve_gpx(&req, &uri, state)
    } else if uri.starts_with("/api/settings/set_all_settings/") {
        serve_set_all_settings(req, state)
    } else if uri.starts_with("/api/settings/") {
        serve_get_all_settings(state)
    } else {
        serve_static_content(&uri)
    }
}

