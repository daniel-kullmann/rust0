use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use serde_json;

use crate::state::State;

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
    let json = serde_json::to_string(&settings).unwrap();
    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, json)))
}

pub fn serve_set_all_settings(req: &mut Request, state: &State) -> IronResult<Response> {
    println!("set all settings");
    let body = req.get::<bodyparser::Json>();
    match body {
        Ok(Some(body)) => {
            match body {
                serde_json::Value::Object(map) => {
                    for (key, value) in &map {
                        println!("{}: {}", key, value);
                        state.connection.execute("REPLACE INTO setting (name, value) VALUES (?, ?)",&[key, &value.to_string().as_str()]).unwrap();
                    }
                }
                _ => ()
            }
        },
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }
    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, "")))
}

