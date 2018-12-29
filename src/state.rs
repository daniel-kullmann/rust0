use crate::config::FinalConfiguration;

use r2d2_sqlite::SqliteConnectionManager;

pub struct State<'a> {
    pub config: &'a FinalConfiguration,
    pub connection: r2d2::PooledConnection<SqliteConnectionManager>
}

