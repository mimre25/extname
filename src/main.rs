use std::env::args;
use std::path::Path;
use std::ffi::OsStr;
use std::process::exit;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn main() {
    let args = args();
    let filename = args.last().expect("");
    println!("filename {}", filename);
    match get_extension_from_filename(&filename){
        Some(extension) => {
            println!("Extension for {}: {}", filename, extension);
            exit(0)
        }
        _ => {
            println!("No extension {}", filename);
            exit(1)
        } 
    }   
}

