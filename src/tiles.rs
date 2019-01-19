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
    let rest: String = uri.chars().skip(7).collect();
    let full_file = Path::new(&state.config.tile_base).join(rest);
    let parent_dir = full_file.parent().unwrap();
    match create_dir_all(parent_dir) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    };
    let len = metadata(&full_file).map(|v| v.len()).unwrap_or(0);
    let fh = File::open(&full_file);
    match (fh, len) {
        (Ok(ref mut file), len) if len > 0 => {
            let mut contents = vec![];
            match file.read_to_end(&mut contents) {
                Err(why) => handle_error(status::NotFound, &why),
                Ok(_) => {
                    println!("INFO: Served existing {}", full_file.to_str().unwrap());
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
                                Err(why) => println!("ERROR: could not create tile file {}: {:?}", full_file.to_str().unwrap(), why),
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
                                    println!("INFO: Served fetched {}", full_file.to_str().unwrap());
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
