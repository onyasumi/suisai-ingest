use std::path::Path;
use infer::MatcherType::Image;
use crate::io::sendable::Sendable;

pub fn search_directory(src: &Path, dest: &Path, prefix: &String) {

    println!("Scanning Path: {}", src.to_str().unwrap());
    if src.is_file() {

        // Get file type
        let file_type = match infer::get_from_path(src).ok().flatten() {
            Some(val) => val.matcher_type(),
            None => return
        };

        if file_type == Image {
            println!(" -> Found Image: {}", src.to_str().unwrap());

            // Try to relocated image, print error on fail
            if let Err(e) = src.send_to(dest, prefix) {
                println!("    Failed to relocate image at {}: {}", src.to_string_lossy(), e);
                println!("    Skipping.");
            }
        }

    } else if src.is_dir() {

        if let Ok(read_dir) = src.read_dir() {
            for child in read_dir.flatten() {
                search_directory(child.path().as_path(), dest, prefix);
            }
        }

    }

}