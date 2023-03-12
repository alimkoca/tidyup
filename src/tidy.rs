use home::home_dir;
use std::path::{Path, PathBuf};
use std::fs::{remove_file, copy, read_dir};

enum FileType {
    Document,
    Software,
    Music,
    Picture,
}

fn file_type(file: &PathBuf) -> FileType {
    let mut file_ext = String::new();
    let file_str = file.to_str().unwrap();

    let mut x = false;

    for c in file_str.as_bytes() {
        if x == true {
            file_ext.push(*c as char);
        }
        if *c == '.' as u8 {
            x = true;
            continue;
        }
    }

    match file_ext.as_ref() {
        "pdf" | "doc" | "txt" => FileType::Document,
        "py" | "c" | "rs" => FileType::Software,
        "png" | "jpg" => FileType::Picture,
        "mp4" | "mp3" => FileType::Music,
        _ => unreachable!()
    }
}

fn search_files(folder: &String) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let path = Path::new(folder);
    let paths = read_dir(path).unwrap();

    for path in paths {
        files.push(path.unwrap().path());
    }

    files
}

pub fn tidy_up(folder: &String) {
    let files = search_files(&folder);

    for file in files {
        match file_type(&file) {
            FileType::Document => {
                let file_name = Path::new(file.file_name().unwrap());
                let new_path = home_dir().unwrap()
                    .join("Documents/")
                    .join(file_name);

                copy(&file, new_path).unwrap();
                remove_file(file).unwrap();
            },
            FileType::Software => {
                let file_name = Path::new(file.file_name().unwrap());
                let new_path = home_dir().unwrap()
                    .join("Softwares/")
                    .join(file_name);

                copy(&file, new_path).unwrap();
                remove_file(file).unwrap();
            },
            FileType::Music => {
                let file_name = Path::new(file.file_name().unwrap());
                let new_path = home_dir().unwrap()
                    .join("Music/")
                    .join(file_name);

                copy(&file, new_path).unwrap();
                remove_file(file).unwrap();
            },
            FileType::Picture => {
                let file_name = Path::new(file.file_name().unwrap());
                let new_path = home_dir().unwrap()
                    .join("Pictures/")
                    .join(file_name);

                copy(&file, new_path).unwrap();
                remove_file(file).unwrap();
            }
        }
    }
}