mod get_shutter;

use crate::get_shutter::get_shutter;
use std::path::Path;

fn main() {

    let args: Box<[String]> = std::env::args().collect();

    if args.len() < 4 {
        println!("Incorrect Usage: suisai-ingest [source path] [output path] [filename prefix]");
        return;
    }

    let source_dir: &Path = Path::new(&args[1]);
    let dest_dir: &Path = Path::new(&args[2]);

    if !source_dir.exists() {
        println!("Source path does not exist!");
        return;
    }

    if !dest_dir.exists() {
        println!("Destination path does not exist!");
        return;
    }

    println!("Shutter Count: {}", get_shutter(source_dir).unwrap());
    println!("Timestamp: {}", get_shutter(source_dir).unwrap());

}
