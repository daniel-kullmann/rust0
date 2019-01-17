use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use serde_json;
use std::collections::HashMap;

use crate::state::State;
use crate::util::handle_error;

#[derive(Debug)]
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
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => {
            let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
            let body: Result<HashMap<String, String>, serde_json::Error> = serde_json::from_str(&body);
            match body {
                Ok(map) => {
                    for (key, value) in &map {
                        let sql = "REPLACE INTO setting (name, value) VALUES (?, ?)";
                        state.connection.execute(sql,&[key, value]).unwrap();
                    }
                    Ok(Response::with((content_type, status::Ok, "[]")))
                },
                Err(err) => handle_error(status::NotFound, &err)
            }
        },
        Ok(None) => handle_error(status::NotFound, &"No body"),
        Err(err) => handle_error(status::NotFound, &err)
    }
}

