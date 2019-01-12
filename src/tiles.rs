use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use reqwest;
use std::fs::{File, create_dir_all, metadata};
use std::io::prelude::*;
use std::path::Path;

use crate::state::State;
use crate::util::handle_error;

pub fn serve_tile(uri: &String, state: &State) -> IronResult<Response>
{
    match create_dir_all(Path::new(state.config.tile_base.as_str())) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }
    let rest: String = uri.chars().skip(7).collect();
    let full_file = format!("{}/{}", state.config.tile_base, rest);
    println!("file: {}", full_file);
    let len = metadata(&full_file).map(|v| v.len()).unwrap_or(0);
    let fh = File::open(&full_file);
    println!("{}, {:?}", len, fh);
    match (fh, len) {
        (Ok(ref mut file), len) if len > 0 => {
            let mut contents = vec![];
            match file.read_to_end(&mut contents) {
                Err(why) => handle_error(status::NotFound, &why),
                Ok(_) => {
                    println!("INFO: Served existing {}", full_file);
                    let content_type = "image/png".parse::<Mime>().expect("Failed to parse content type");
                    Ok(Response::with((content_type, status::Ok, contents)))
                }
            }
        }
        (_, _) => {
            let parts: Vec<&str> = uri.split("/").collect();
            match parts.as_slice() {
                ["", "tiles", s, z, x, y] => {
                    let osm_url: String = format!("https://{}.tile.openstreetmap.org/{}/{}/{}", s, z, x, y);
                    println!("INFO: Fetch from OSM: {:?}", osm_url);
                    let response = reqwest::get(osm_url.as_str());
                    match response {
                        Err(why) => handle_error(status::NotFound, &why),
                        Ok(mut response) => {
                            let mut buf: Vec<u8> = vec![];

                            let copy_to_result = response.copy_to(&mut buf);
                            match File::create(&full_file) {
                                Err(why) => println!("ERROR: could not create tile file {}: {:?}", full_file, why),
                                Ok(mut file) => {
                                    match file.write_all(&buf[..]) {
                                        Err(why) => println!("ERROR: could not save tile file: {:?}", why),
                                        Ok(_) => ()
                                    }

                                }
                            };

                            match copy_to_result {
                                Err(why) => handle_error(status::NotFound, &why),
                                Ok(_) => {
                                    println!("INFO: Served fetched {}", full_file);
                                    let content_type = "image/png".parse::<Mime>().expect("Failed to parse content type");
                                    Ok(Response::with((content_type, status::Ok, buf)))
                                }
                            }
                        }
                    }
                },
                _ => {
                    Ok(Response::with((status::NotFound, "ERROR: url wrong (get tile from osm)")))
                }
            }
        }
    }
}
