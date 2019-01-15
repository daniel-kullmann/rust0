use iron::prelude::*;
use iron::status;
use serde_json::Value;
use std::any::Any;

pub fn handle_error(status_code: status::Status, why: &Any) -> IronResult<Response> {
    println!("ERROR: {:?}", why);
    //panic!("");
    Ok(Response::with((status_code, "")))
}

pub fn json_value_to_string(json: &Value) -> String {
    match json {
        serde_json::Value::String(val) => val.clone(),
        serde_json::Value::Number(val) => val.to_string(),
        serde_json::Value::Bool(true) => String::from("1"),
        serde_json::Value::Bool(false) => String::from("0"),
        serde_json::Value::Null => String::from(""), // TODO: rusqlite::types::Null,
        serde_json::Value::Array(_) => panic!(format!("can't store an object in database: {:?}", json)),
        serde_json::Value::Object(_) => panic!(format!("can't store an object in database: {:?}", json)),
    }
}

