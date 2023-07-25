use std::fs;
use std::path::{Path, PathBuf};

use clap::{command, Parser};
use log::{debug, error, info};
use log4rs::init_file;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CommandArgs {
    /// directory to scan
    dir: PathBuf,

    /// dry run
    #[arg(short, long, default_value_t = false)]
    dry: bool,
}

fn main() {
    init_file("log4rs.yml", Default::default()).unwrap();

    let args = CommandArgs::parse();
    let folder = args.dir;

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if Path::new(path).extension().is_some() {
            continue;
        }

        if let Ok(Some(t)) = infer::get_from_path(path) {
            debug!("inferred extension for {} is {}", path.display(), t.extension());
            debug!("inferred mimetype for {} is {}", path.display(), t.mime_type());

            if args.dry {
                info!("renamed {} to {}.{}", path.display(), path.display(), t);
                continue;
            }

            let ext = t.extension();
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