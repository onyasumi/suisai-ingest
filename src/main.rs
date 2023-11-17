use std::path::Path;

fn main() {

    let args: Box<[String]> = std::env::args().collect();

    assert!(args.len() > 3);

    let source_dir: &Path = Path::new(&args[1]);
    let dest_dir: &Path = Path::new(&args[2]);

    assert!(!source_dir.exists());
    assert!(!dest_dir.exists());

}
