use std::path::PathBuf;

use clap::{Arg, Command};

#[derive(Debug, Clone)]
pub struct OsArgs {
    filepath: PathBuf,
    temporary_folder: PathBuf,
    duration: u64,
}

impl OsArgs {
    pub fn parse() -> Self {
        let filepath_id = "filepath";
        let temporary_folder_id = "temporary_folder";
        let duration_id = "duration";
        let cmd = Command::new("filemark")
            .about("Monitors a file for changes and creates copies every durations.")
            .arg(Arg::new(filepath_id).help("Filepath to mark."))
            .arg(Arg::new(temporary_folder_id).help("Folder path to save marked file."))
            .arg(
                Arg::new(duration_id)
                    .value_parser(clap::value_parser!(u64))
                    .help("Mark duration."),
            )
            .get_matches();

        let filepath = cmd.get_one::<String>(&filepath_id).unwrap();
        let filepath = PathBuf::from(filepath);

        let temporary_folder = cmd.get_one::<String>(&temporary_folder_id).unwrap();
        let temporary_folder = PathBuf::from(temporary_folder);

        let duration = *cmd.get_one::<u64>(&duration_id).unwrap();

        OsArgs {
            duration,
            filepath,
            temporary_folder,
        }
    }

    pub fn get_filepath(&self) -> PathBuf {
        self.filepath.clone()
    }

    pub fn get_temporary_folder(&self) -> PathBuf {
        self.temporary_folder.clone()
    }

    pub fn get_duration(&self) -> u64 {
        self.duration
    }
}
