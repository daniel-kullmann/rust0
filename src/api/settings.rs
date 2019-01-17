use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use serde_json;
use std::collections::HashMap;

use crate::state::State;
use crate::util::handle_error;
use crate::util::json_value_to_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    name: String,
    value: String
}

pub fn serve_get_all_settings(state: &State) -> IronResult<Response> {
    let mut stmt = state.connection
        .prepare("SELECT name, value FROM setting")
        .unwrap();
    let settings_iter = stmt
        .query_map(&[], |row| Setting {
            name: row.get(0),
            value: row.get(1),
        }).unwrap();
    let settings: Vec<Setting> = settings_iter.map(|item| item.unwrap()).collect();
    let mut result = HashMap::new();
    for setting in settings {
        result.insert(setting.name, setting.value);
    }
    let json = serde_json::to_string(&result).unwrap();
    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, json)))
}

pub fn serve_set_all_settings(req: &mut Request, state: &State) -> IronResult<Response> {
    let body = req.get::<bodyparser::Json>();
    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    match body {
        Ok(Some(body)) => {
            match body {
                serde_json::Value::Object(map) => {
                    for (key, value) in &map {
                        let value: String = json_value_to_string(value);
                        let sql = "REPLACE INTO setting (name, value) VALUES (?, ?)";
                        state.connection.execute(sql,&[key, &value.as_str()]).unwrap();
                    }
                    Ok(Response::with((content_type, status::Ok, "[]")))
                }
                _ => handle_error(status::NotFound, &"json request body not recognized")
            }
        },
        Ok(None) => handle_error(status::NotFound, &"No body"),
        Err(err) => handle_error(status::NotFound, &err)
    }
}

