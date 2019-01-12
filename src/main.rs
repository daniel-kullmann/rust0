extern crate iron;
extern crate r2d2_sqlite;
extern crate rust0;

use iron::prelude::*;
use std::ops::Deref;
use std::sync::Arc;
use r2d2_sqlite::SqliteConnectionManager;

use rust0::config::get_config;
use rust0::server::serve;
use rust0::state::State;


fn main() {

    let config = Arc::new(get_config());

    let pool = {
        let manager = SqliteConnectionManager::file(&config.db_file);
        r2d2::Pool::new(manager).unwrap()
    };

    let service = move |req: &mut Request| {
        let pool = pool.clone();
        let config = config.clone();
        let state = State {
            config: &config.deref(),
            connection: pool.get().unwrap()
        };
        serve(req, &state)
    };

    let _server = Iron::new(service).http("localhost:3000").unwrap();

    println!("server started at http://localhost:3000");
}
