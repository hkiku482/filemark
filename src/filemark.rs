use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use chrono::{DateTime, Local};
use sha2::{Digest, Sha256};

pub fn prepare(target_file: &PathBuf, folder: &PathBuf) {
    if !folder.is_dir() {
        fs::create_dir(folder).unwrap();
    }
    if !target_file.is_file() {
        panic!("target file does not exists");
    }
}

fn get_now() -> chrono::DateTime<Local> {
    Local::now()
}

const SEP: char = '_';

pub fn make_copy(target_filepath: &PathBuf, folder: &PathBuf) {
    let target_filename = target_filepath.file_name().unwrap().to_str().unwrap();
    let suffix = get_now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    let suffix = suffix.replace(":", ".");
    let mut dest_name = String::from(&suffix);
    dest_name.push(SEP);
    dest_name.push_str(target_filename);
    let mut dest_path = folder.clone();
    dest_path.push(dest_name);
    fs::copy(target_filepath, dest_path).unwrap();
}

// find most recent updated and calc hash
pub fn is_updated(target_filepath: &PathBuf, folder: &PathBuf) -> bool {
    let target_filename = target_filepath.file_name().unwrap().to_str().unwrap();

    let mut target_files = Vec::<PathBuf>::new();
    let e = fs::read_dir(folder).unwrap();

    // Read folder
    for entry in e {
        match entry {
            Ok(e) => {
                if match e.file_type() {
                    Err(_) => false,
                    Ok(t) => t.is_file(),
                } {
                    target_files.push(PathBuf::from(e.path()));
                }
            }
            Err(_) => {}
        };
    }

    // Look up copied file (`<RFC3339><SEP><filename any>`).
    let mut recent: Option<(&PathBuf, i64)> = None;
    for target_file in &target_files {
        let filename = match target_file.file_name() {
            None => continue,
            Some(v) => match v.to_str() {
                None => continue,
                Some(s) => s,
            },
        };
        let token: Vec<&str> = filename.split(SEP).collect();
        if token.len() < 2 {
            continue;
        }
        let timestamp = token[0].replace(".", ":");
        let date = match timestamp.parse::<DateTime<Local>>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if token[1] == target_filename {
            let ts = date.timestamp();
            match recent {
                None => {
                    recent = Some((target_file, ts));
                }
                Some(r) => {
                    if r.1 < ts {
                        recent = Some((target_file, ts));
                    }
                }
            }
        }
    }

    let recent = match recent {
        None => return false,
        Some(r) => r.0,
    };

    let mut file = File::open(target_filepath).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let s_hash = Sha256::digest(buffer);

    let mut file = File::open(recent).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let t_hash = Sha256::digest(buffer);

    if s_hash != t_hash {
        return false;
    }

    return true;
}
