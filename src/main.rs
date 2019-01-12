extern crate iron;
extern crate mime_guess;
extern crate r2d2_sqlite;
extern crate rust0;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use mime_guess::guess_mime_type;
use std::ops::Deref;
use std::sync::Arc;
use r2d2_sqlite::SqliteConnectionManager;

use rust0::config::get_config;
use rust0::api::gpx::serve_gpx;
use rust0::api::settings::{serve_get_all_settings, serve_set_all_settings};
use rust0::state::State;
use rust0::tiles::serve_tile;
use rust0::util::handle_error;
use rust0::files::file_content;


fn serve(req: &mut Request, state: &State) -> IronResult<Response> {
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
        let static_file = file_content(&uri);
        match static_file {
            Some(content) => {
                let mime_type = guess_mime_type(&uri).to_string();
                let content_type = mime_type.parse::<Mime>().expect("Failed to parse content type");
                println!("Serving static file {} as {}", uri, mime_type);
                Ok(Response::with((content_type, status::Ok, content)))
            }
            None => handle_error(status::NotFound, &""),
        }
    }
}

fn main() {

    let config = Arc::new(get_config());

    let pool = {
        let manager = SqliteConnectionManager::file(&config.db_file);
        r2d2::Pool::new(manager).unwrap()
    };

    let service = move |req: &mut Request| {
        let pool = pool.clone();
        let config = config.clone();
        let state = State {
            config: &config.deref(),
            connection: pool.get().unwrap()
        };
        serve(req, &state)
    };

    let _server = Iron::new(service).http("localhost:3000").unwrap();

    println!("server started at http://localhost:3000");
}
