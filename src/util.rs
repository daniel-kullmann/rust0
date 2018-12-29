use hyper::{Body, Response, StatusCode};
use std::any::Any;

pub fn handle_error(status_code: StatusCode, why: &Any) -> Response<Body> {
    println!("{:?}", why);
    let mut response = Response::builder();
    response.status(status_code);
    Response::new(Body::from(""))
}

