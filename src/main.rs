mod metadata;
mod io;


use std::path::Path;
use crate::io::search_directory::search_directory;


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

    search_directory(source_dir, dest_dir, &args[3]);

}
