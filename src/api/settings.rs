use futures::{Future, Stream};
use hyper::{Body, Request, Response, StatusCode};
use serde_json::{Value};

use crate::state::State;
use crate::util::handle_error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    name: String,
    value: String
}

pub fn serve_get_all_settings(state: &State) -> Response<Body> {
    let mut stmt = state.connection
        .prepare("SELECT name, value FROM setting")
        .unwrap();
    let settings_iter = stmt
        .query_map(&[], |row| Setting {
            name: row.get(0),
            value: row.get(1),
        }).unwrap();
    let settings: Vec<Setting> = settings_iter.map(|item| item.unwrap()).collect();
    let json = serde_json::to_string(&settings).unwrap();
    let mut response = Response::builder();
    response.header("Content-Type", "application/json").status(StatusCode::OK);
    response.body(Body::from(json)).unwrap()
}

pub fn serve_set_all_settings(req: Request<Body>, state: &State) -> Response<Body> {
    let _response = req.into_body().concat2().and_then(move |body| {
        let vec = body.iter().cloned().collect();
        let stringify = String::from_utf8(vec).unwrap();
        println!("{}", stringify);
        let response = Response::new(&"");
        futures::future::ok(response)
    });
    _response.wait();
    //let body: Value = serde_json::from_slice(body.to_slice());
    //println!("{:?}", body);
    //match state.connection.execute("REPLACE INTO setting (name, value) VALUES (?, ?)",&[]) {
    //    Err(_why) => (),
    //    Ok(_result) => ()
    //}
    handle_error(StatusCode::NOT_FOUND, &"")
}
