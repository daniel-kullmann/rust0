use iron::prelude::*;
use iron::status;
use std::any::Any;

pub fn handle_error(status_code: status::Status, why: &Any) -> IronResult<Response> {
    println!("ERROR: {:?}", why);
    //panic!("");
    Ok(Response::with((status_code, "")))
}

