use std::{thread::sleep, time::Duration};

use filemark::{is_updated, make_copy, prepare};

pub mod cli;

fn main() {
    let opt = cli::OsArgs::parse();
    let d = Duration::from_secs(opt.get_duration() * 60);
    loop {
        prepare(&opt.get_filepath(), &opt.get_temporary_folder());
        let m = is_updated(&opt.get_filepath(), &opt.get_temporary_folder());
        if !m {
            make_copy(
                &opt.get_filepath(),
                &opt.get_temporary_folder(),
                opt.get_show_result(),
            );
        }
        if d.is_zero() {
            break;
        }
        sleep(d);
    }
}
