// The build script creates a file in the target directory
// src/static.rs that contains the contents of the static
// files coming from the static directory taken from the
// environment variable ASSETS
// Each file content is mapped to a static variable
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use log::{info, error};

fn main() {
    // This code expect there are 3 static files: index.html, index.bundle.js, and index.bundle.js.map
    // in the static directory
    let file_list = vec!["index.html", "index.bundle.js", "index.bundle.js.map"];

    let _ = env_logger::try_init();
    let _ = dotenv::dotenv();

    
    let assets = env::var("ASSETS").unwrap();
    let assets = Path::new(&assets);
    // the path must exists
    if !assets.exists() {
        error!("The path {} must exists and contains files {}", assets.display(), file_list.join(", "));
        error!("The path {} does not exists", assets.display());
        panic!("The path {} does not exists", assets.display());
    }
    // the files must exists
    for file in file_list.iter() {
        let file = assets.join(file);
        if !file.exists() {
            error!("The file {} must exists", file.display());
            panic!("The file {} must exists", file.display());
        }
    }
    
    let out_dir = "src".to_string();
    let dest_path = Path::new(&out_dir).join("assets.rs");
    let mut file = File::create(&dest_path).unwrap();
    writeln!(file, "pub mod static_files {{").unwrap();
    // loop over the files
    for f in file_list.iter() {
        let f = assets.join(f);
        let name = format!("{}", f.file_name().unwrap().to_str().unwrap());
        // replace all . with _
        let name = name.replace(".", "_").to_uppercase();
        // read the file content into a string and write it to the file
        info!("Reading file {}", f.display());
        let content = std::fs::read_to_string(&f).unwrap();
        writeln!(file, "    pub static {}: &'static str = r###\"{}\"###;", name, content).unwrap();
    }
    writeln!(file, "}}").unwrap();
}
