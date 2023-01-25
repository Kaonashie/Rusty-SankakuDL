use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::Path;
use crate::DEFAULT_DOWNLOAD_DIRECTORY;

pub fn cmd_pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "Press any key to continue...").ok();
    stdout.flush().ok();
    let _ = stdin.read(&mut [0u8]).ok();
}

pub fn create_dl_directory() {
    let is_created: bool = Path::new(DEFAULT_DOWNLOAD_DIRECTORY).is_dir();

    if !is_created {
        println!("No downloads folder found.\nCreating the default one...");
        std::fs::create_dir(DEFAULT_DOWNLOAD_DIRECTORY).expect("Failed to created local downloads directory.");
        cmd_pause();
    }
}

pub fn parse_file_extension(file_name: &str) -> Option<&str> {
    Path::new(file_name).extension().and_then(OsStr::to_str)
}