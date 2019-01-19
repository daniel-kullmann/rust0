use iron::prelude::*;

use crate::api::gpx::serve_gpx;
use crate::api::settings::serve_settings;
use crate::files::serve_static_content;
use crate::state::State;
use crate::tiles::serve_tile;

pub fn serve(req: &mut Request, state: &State) -> IronResult<Response> {
    let uri = "/".to_owned() + &req.url.path().join(&"/");
    if uri.starts_with("/tiles") {
        serve_tile(&uri, state)
    } else if uri.starts_with("/api/gpx/") {
        serve_gpx(req, &uri, state)
    } else if uri.starts_with("/api/settings/") {
        serve_settings(req, &uri, state)
    } else {
        serve_static_content(&uri)
    }
}

