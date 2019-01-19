use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use mime_guess::guess_mime_type;

use crate::util::handle_error;

include!(concat!(env!("OUT_DIR"), "/binary_data.rs"));

pub fn serve_static_content(uri: &String) -> IronResult<Response> {
    let uri: String = if uri == "/" { String::from("/index.html") } else  { uri.to_string() };
    let static_file = file_content(&uri);
    match static_file {
        Some(content) => {
            let mime_type = guess_mime_type(&uri).to_string();
            let content_type = mime_type.parse::<Mime>().expect("Failed to parse content type");
            println!("Serving static file {} as {}", uri, mime_type);
            Ok(Response::with((content_type, status::Ok, content)))
        }
        None => handle_error(status::NotFound, &uri),
    }
}
