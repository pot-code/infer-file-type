use std::{env, fs};
use std::collections::HashMap;
use std::path::Path;

use log::{error, info};
use log4rs::init_file;
use walkdir::WalkDir;

fn main() {
    init_file("log4rs.yml", Default::default()).unwrap();

    let folder = env::args().last().unwrap();
    let mut ext_counter: HashMap<&str, i32> = HashMap::new();

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if Path::new(path).extension().is_some() {
            continue;
        }

        if let Ok(Some(ext)) = infer::get_from_path(path).map(|t| t.map(|x| x.extension())) {
            ext_counter.entry(ext).and_modify(|x| *x += 1).or_insert(1);
            match fs::rename(path, path.with_extension(ext)) {
                Ok(_) => {
                    info!("renamed {} to {}.{}", path.display(), path.display(), ext);
                }
                Err(e) => {
                    error!("failed to rename {} to {}.{}: {}", path.display(), path.display(), ext, e);
                }
            }
        }
    }
}