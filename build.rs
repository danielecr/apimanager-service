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
use phf;

fn main() {
    let _ = env_logger::try_init();
    let _ = dotenv::dotenv();
    let assets = env::var("ASSETS").unwrap();
    let assets = Path::new(&assets);
    // the path must exists
    if !assets.exists() {
        error!("The path {} must exists and contains files", assets.display());
        panic!("The path {} does not exists", assets.display());
    }
    // list all files in the static directory and its subdirectories
    let files = walkdir::WalkDir::new(assets)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();
    
    let out_dir = "src".to_string();
    let dest_path = Path::new(&out_dir).join("assets.rs");
    let mut file = File::create(&dest_path).unwrap();
    let mut phfm = phf_codegen::Map::new();
    writeln!(file, "pub mod static_files {{").unwrap();
    // loop over the files
    for f in files.iter() {
        //let f = assets.join(f);
        let name = format!("{}", f.file_name().unwrap().to_str().unwrap());
        // replace all . with _
        let symb = name.replace(".", "_").to_uppercase();
        let symbref = "static_files::".to_string() + &symb;
        //("static_files::");
        phfm.entry(name, &symbref);
        // read the file content into a string and write it to the file
        info!("Reading file {}", f.display());
        let content = std::fs::read_to_string(&f).unwrap();
        writeln!(file, "    pub static {}: &'static str = r###\"{}\"###;", symb, content).unwrap();
    }
    writeln!(file, "}}").unwrap();
    writeln!(file, "use phf;").unwrap();
    // write a static hash map with phf crate
    write!(
        &mut file,
        "pub static STATIC_FILEMAP: phf::Map<&'static str, &'static str> = {}",
        phfm.build()
    )
    .unwrap();
    writeln!(file, ";\n").unwrap();
    info!("Generated file {}", dest_path.display());
}
