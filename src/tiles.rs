use hyper::{Body, Response};
use reqwest;
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::Path;

use crate::state::State;

pub fn serve_tile(uri: &String, state: &State) -> Response<Body>
{
    match create_dir_all(Path::new(state.config.tile_base.as_str())) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }
    let rest: String = uri.chars().skip(7).collect();
    let full_file = format!("{}/{}", state.config.tile_base, rest);
    println!("file: {}", full_file);
    match File::open(&full_file) {
        Ok(mut file) => {
            let mut contents = vec![];
            match file.read_to_end(&mut contents) {
                Ok(_) => {
                    println!("INFO: Served {}", full_file);
                    Response::new(Body::from(contents))
                },
                Err(_) => {
                    Response::new(Body::from("ooh no!"))
                }
            }
        }
        Err(_) => {
            let parts: Vec<&str> = uri.split("/").collect();
            match parts.as_slice() {
                ["", "tiles", s, z, x, y] => {
                    let osm_url: String = format!("https://{}.tile.openstreetmap.org/{}/{}/{}", s, z, x, y);
                    println!("INFO: Fetch from OSM: {:?}", osm_url);
                    let response = reqwest::get(osm_url.as_str());
                    match response {
                        Ok(mut response) => {
                            let mut buf: Vec<u8> = vec![];
                            match File::create(&full_file) {
                                Err(why) => println!("ERROR: could not create tile file: {:?}", why),
                                Ok(mut file) => {
                                    match file.write_all(&buf[..]) {
                                        Err(why) => println!("ERROR: could not save tile file: {:?}", why),
                                        Ok(_) => ()
                                    }

                                }
                            };

                            match response.copy_to(&mut buf) {
                                Ok(_) => Response::new(Body::from(buf)),
                                Err(_err) => Response::new(Body::from("ERROR: could not copy"))
                            }
                        },
                        Err(err) => {
                            println!("{:?}", err);
                            Response::new(Body::from("TODO: get tile from osm"))
                        }
                    }
                },
                _ => {
                    Response::new(Body::from("ERROR: url wrong (get tile from osm)"))
                }
            }
        }
    }
}
