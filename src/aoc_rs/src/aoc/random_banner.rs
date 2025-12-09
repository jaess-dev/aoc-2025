use std::{fs, io, path::Path};

use rand::seq::IndexedRandom;

pub fn banner() {
    match read_random_banner("resources/banner") {
        Ok(banner) => println!("{}", banner),
        _ => {}
    }
}

fn read_random_banner(dir: &str) -> io::Result<String> {
    let path = Path::new(dir);

    // Collect all regular files
    let files: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    if files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No files found"));
    }

    // Pick a random file
    let mut rng = rand::rng();
    let entry = files.choose(&mut rng).expect("files list is non-empty");

    let file_path = entry.path();

    // Read file as string
    fs::read_to_string(file_path)
}
