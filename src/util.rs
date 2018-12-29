use hyper::{Body, Response, StatusCode};
use std::io::Error;

pub fn handle_io_error(status_code: StatusCode, why: Error) -> Response<Body> {
    println!("{:?}", why);
    let mut response = Response::builder();
    response.status(status_code);
    Response::new(Body::from(""))
}
