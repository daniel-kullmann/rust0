//extern crate compress;
extern crate walkdir;

//use compress::lz4;
use std::cmp::max;
use std::env;
use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::time::SystemTime;
use walkdir::WalkDir;

fn main() {
    let target_dir = env::var("OUT_DIR").unwrap();
    let target_file = format!("{}/{}", &target_dir, "binary_data.rs");
    let base_dir = "./frontend";

    let mut paths = Vec::new();
    let mut source_modified: SystemTime = metadata("build.rs").unwrap().modified().unwrap();
    for entry in WalkDir::new(base_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            let entry = entry.path().to_str().unwrap().to_string();
            source_modified = max(source_modified, metadata(&entry).unwrap().modified().unwrap());
            let local_path = String::from(&entry[base_dir.len()..]);
            if local_path != "" {
                paths.push(local_path);
            }
        }
    }

    let target_modified = match metadata(&target_file) {
        Ok(metadata) => metadata.modified().unwrap(),
        Err(_) => SystemTime::UNIX_EPOCH,
    };

    if target_modified < source_modified {
        println!("source files are modified; rebuilding binary_data.rs..");

        let mut file_content: Vec<String> = Vec::new();
        file_content.push(String::from("pub fn stored_files() -> Vec<&'static str> {"));
        file_content.push(format!("  vec![\"{}\"]", paths.join("\", \"")));
        file_content.push(String::from("}\n\n"));
        file_content.push(String::from("pub fn file_content(file_name: &str) -> Option<Vec<u8>> {"));
        for path in paths {
            let full_file_name = format!("{}{}", base_dir, path);
            let mut fh = File::open(full_file_name).unwrap();
            //let mut compressed: Vec<u8> = Vec::new();
            //lz4::Encoder::new(stream).write(&mut compressed);
            let mut content: Vec<u8> = Vec::new();
            fh.read_to_end(&mut content).unwrap();
            file_content.push(format!("if file_name == \"{}\" {{", path));
            file_content.push(format!("return Some(vec!{:?})", content));
            file_content.push(String::from("}"));
        }
        file_content.push(String::from("  None"));
        file_content.push(String::from("}\n\n"));

        let mut f = File::create(&target_file).unwrap();

        f.write_all(file_content.join("\n").as_bytes()).unwrap();
    } else {
        println!("source files are not modified");
    }
}
