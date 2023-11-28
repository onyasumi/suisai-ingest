use std::path::Path;
use infer::MatcherType::Image;

pub fn search_directory(path: &Path) {

    println!("Scanning Path: {}", path.to_str().unwrap());
    if path.is_file() {

        let file_type = match infer::get_from_path(path).ok().flatten() {
            Some(val) => val.matcher_type(),
            None => return
        };

        if file_type == Image {
            println!(" - Found Image: {}", path.to_str().unwrap());
        }

    } else if path.is_dir() {

        if let Ok(read_dir) = path.read_dir() {
            for child in read_dir.flatten() {
                search_directory(child.path().as_path())
            }
        }

    }

}