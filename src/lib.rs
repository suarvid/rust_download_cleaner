extern crate directories;
use directories::UserDirs;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

// TODO: make functions return Results, implement error-handeling in main.rs

pub fn run() {
    let download_path: &Path;
    if let Some(user_dirs) = UserDirs::new() {
        // Probably shouldn't use unwrap for this
        download_path = user_dirs.download_dir().unwrap().clone();
        let paths = get_directory_entry_paths(download_path).unwrap();
        for path in paths {
            match get_entry_extention(&path) {
                Some("jpg") | Some("jpeg") | Some("png") => move_image(&path, download_path),
                Some("pdf") | Some("epub") => move_book(&path, download_path),
                Some("mp4") | Some("avi") | Some("mpg") | Some("mpv") => {
                    move_video(&path, download_path)
                }
                Some("wav") | Some("mp3") => move_audio(&path, download_path),
                Some(_) => eprintln!("File extension not supported for move!"),
                None => {
                    eprintln!("Error getting file extension");
                }
            }
        }
    }
}

pub fn get_directory_entry_paths<'a>(directory: &Path) -> io::Result<Vec<String>> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(directory)? {
        let dir = entry?;
        let entry_path = dir.path();
        let entry_path = entry_path.into_os_string().into_string();
        match entry_path {
            Ok(path) => paths.push(path),
            Err(_) => {
                eprintln!("Error converting path to string!");
            }
        }
    }

    Ok(paths)
}

fn move_book(path: &String, download_path: &Path) {
    move_file(path, download_path, &String::from("books"));
}

fn move_image(path: &String, download_path: &Path) {
    move_file(path, download_path, &String::from("images"));
}

fn move_audio(path: &String, download_path: &Path) {
    move_file(path, download_path, &String::from("audio"));
}

fn move_video(path: &String, download_path: &Path) {
    move_file(path, download_path, &String::from("video"));
}

// Error handeling should probably not depend on the use of expect here
fn move_file(path: &String, download_path: &Path, directory: &String) {
    let download_path_len = download_path.to_str().unwrap().len();
    match fs::copy(
        path,
        download_path.to_str().unwrap().to_owned()
            + "/"
            + directory
            + "/"
            + &path[download_path_len..],
    ) {
        Ok(_) => fs::remove_file(path).expect("Error removing copied file!"),
        Err(_) => {}
    }
}

pub fn get_entry_extention(entry: &String) -> Option<&str> {
    Path::new(entry).extension().and_then(OsStr::to_str)
}

// So maybe there's a better way to do this, based on the actual extensions found
// But covers like 99% of all files I download even if implemented like this
fn create_extension_directories() -> io::Result<()> {
    fs::create_dir("$HOME/Downloads/Books")?;
    fs::create_dir("$HOME/Downloads/Audio")?;
    fs::create_dir("$HOME/Downloads/Video")?;
    fs::create_dir("$HOME/Downloads/Images")?;
    Ok(())
}
