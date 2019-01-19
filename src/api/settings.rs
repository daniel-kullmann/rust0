use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json;
use std::collections::HashMap;

use crate::state::State;
use crate::util::handle_error;

#[derive(Debug)]
pub struct Setting {
    name: String,
    value: String
}

pub fn serve_settings(req: &mut Request, uri: &String, state: &mut State) -> IronResult<Response> {
    if uri.starts_with("/api/settings/set_all_settings/") {
        serve_set_all_settings(req, state)
    } else if uri.starts_with("/api/settings/") {
        serve_get_all_settings(state)
    } else {
        handle_error(status::NotFound, &format!("Wrong settings uri: {}", uri))
    }
}

pub fn serve_get_all_settings(state: &mut State) -> IronResult<Response> {
    let json = get_all_settings(&mut state.connection);
    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, json)))
}

pub fn serve_set_all_settings(req: &mut Request, state: &mut State) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => {
            let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
            match set_all_settings(&body, &mut state.connection) {
                Ok(_) => Ok(Response::with((content_type, status::Ok, "[]"))),
                Err(err) => handle_error(status::NotFound, &err)
            }
        },
        Ok(None) => handle_error(status::NotFound, &"No body"),
        Err(err) => handle_error(status::NotFound, &err)
    }
}

fn get_all_settings(connection: &mut PooledConnection<SqliteConnectionManager>) -> String {
    let mut stmt = connection
        .prepare("SELECT name, value FROM setting ORDER BY name")
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
    serde_json::to_string(&result).unwrap()
}

fn set_all_settings(body: &String, connection: &mut PooledConnection<SqliteConnectionManager>) -> Result<(), serde_json::Error> {
    let body: Result<HashMap<String, String>, serde_json::Error> = serde_json::from_str(body);
    match body {
        Ok(map) => {
            for (key, value) in &map {
                let sql = "REPLACE INTO setting (name, value) VALUES (?, ?)";
                connection.execute(sql,&[key, value]).unwrap();
            }
            Ok(())
        },
        Err(err) => Err(err)
    }
}
