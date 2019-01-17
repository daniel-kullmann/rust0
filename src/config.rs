use clap::{App, Arg};
use ini::Ini;

#[derive(Debug)]
struct Configuration {
    config_file_name: Option<String>,
    tile_base: Option<String>,
    gpx_base: Option<String>,
    db_file: Option<String>,
    listen_port: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct FinalConfiguration {
    pub config_file_name: String,
    pub tile_base: String,
    pub gpx_base: String,
    pub db_file: String,
    pub listen_port: u32,
}

pub fn get_config() -> FinalConfiguration {
    let default_configuration_for_config_file = Configuration {
        config_file_name: expand_env(Some(&String::from("${HOME}/.config/simple-offline-map/config.ini"))),
        tile_base: None,
        gpx_base: None,
        db_file: None,
        listen_port: None,
    };

    let default_configuration_for_others = Configuration {
        config_file_name: None,
        tile_base: expand_env(Some(&String::from("${HOME}/.local/share/simple-offline-map/gpx"))),
        gpx_base: expand_env(Some(&String::from("${HOME}/.local/share/simple-offline-map/tiles"))),
        db_file: expand_env(Some(&String::from("${HOME}/.local/share/simple-offline-map/db.sqlite3"))),
        listen_port: Some(3000),
    };

    let config_cli = parse_command_line().merge(&default_configuration_for_config_file);
    let config_file = parse_config_file(&config_cli.config_file_name);
    let config = config_cli.merge(&config_file).merge(&default_configuration_for_others);
    FinalConfiguration::from(config)
}

impl From<Configuration> for FinalConfiguration {
    fn from(v: Configuration) -> FinalConfiguration {
        FinalConfiguration{
            config_file_name: v.config_file_name.expect("ConfigFileName should have been configured"),
            tile_base: v.tile_base.expect("TileBase should have been configured"),
            gpx_base: v.gpx_base.expect("GpxBase should have been configured"),
            db_file: v.db_file.expect("DbFile should have been configured"),
            listen_port: v.listen_port.expect("ListenPort should have been configured"),
        }
    }
}

fn expand_env(option: Option<&String>) -> Option<String> {
    option.map(|value| shellexpand::env(value).unwrap().into_owned())
}

fn parse_config_file(config_file_name: &Option<String>) -> Configuration {
    match config_file_name {
        None => {
            Configuration{ config_file_name: None, tile_base: None, gpx_base: None , db_file: None, listen_port: None }
        },
        Some(file_name) => {
            let config = Ini::load_from_file(file_name)
                .expect(&format!("Config file {} could not be found", file_name)[..]);
            let section: Option<String> = None;
            let section = config.section(section).unwrap();
            let tile_base: Option<&String> = section.get("TileBase");
            let gpx_base: Option<&String> = section.get("GpxBase");
            let db_file: Option<&String> = section.get("DbFile");
            let listen_port: Option<&String> = section.get("ListenPort");
            Configuration {
                config_file_name: None,
                tile_base: expand_env(tile_base),
                gpx_base: expand_env(gpx_base),
                db_file: expand_env(db_file),
                listen_port: expand_env(listen_port).map(|v| v.parse().expect("ListenPort should contain a number")),
            }
        }
    }
}

fn parse_command_line() -> Configuration {

    let matches = App::new("My Test Program")
        .version("")
        .author("")
        .about("")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .takes_value(true)
             .help("File name of config file"))
        .arg(Arg::with_name("tile-base")
             .short("t")
             .long("tile-base")
             .takes_value(true)
             .help("Base directory of tile cache"))
        .arg(Arg::with_name("gpx-base")
             .short("g")
             .long("gpx-base")
             .takes_value(true)
             .help("Base directory of GPX file store"))
        .arg(Arg::with_name("db-file")
             .short("d")
             .long("db-file")
             .takes_value(true)
             .help("File where Sqlite3 database is stored"))
        .arg(Arg::with_name("listen-port")
             .short("p")
             .long("listen-port")
             .takes_value(true)
             .help("Port that server listens on (default is 3000)"))
        .get_matches();

    let config_file_name = matches.value_of("config").map(|value| String::from(value));
    let tile_base = matches.value_of("tile-base").map(|value| String::from(value));
    let gpx_base = matches.value_of("gpx-base").map(|value| String::from(value));
    let db_file = matches.value_of("db-file").map(|value| String::from(value));
    let listen_port = matches.value_of("listen-port").map(|value| value.parse().expect("listen-port must be a number"));
    return Configuration{config_file_name, tile_base, gpx_base, db_file, listen_port};
}

impl Configuration {
    // merge two configurations; self takes precedence
    fn merge(&self, other: &Configuration) -> Configuration {
        Configuration{
            config_file_name: match self.config_file_name {
                Some(_) => self.config_file_name.clone(),
                None => other.config_file_name.clone()
            },
            tile_base: match self.tile_base {
                Some(_) => self.tile_base.clone(),
                None => other.tile_base.clone()
            },
            gpx_base: match self.gpx_base {
                Some(_) => self.gpx_base.clone(),
                None => other.gpx_base.clone()
            },
            db_file: match self.db_file {
                Some(_) => self.db_file.clone(),
                None => other.db_file.clone()
            },
            listen_port: match self.listen_port {
                Some(_) => self.listen_port.clone(),
                None => other.listen_port.clone()
            },
        }
    }
}

