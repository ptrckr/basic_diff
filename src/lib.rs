use std::fs::{self, File};
use std::io::{Read, BufReader};
use std::path::Path;

pub fn diff(a_path: &Path, b_path: &Path) {
    let files = get_files(&a_path, &b_path);
    let mut iter = files.iter().enumerate();
    while let Some((idx, entry)) = iter.next() {
        let file_name = entry.file_name();
        let path = entry.path();
        let peek = files.get(idx + 1);

        if peek.is_none()
            || peek.unwrap().file_name() != file_name
            || !types_match(&peek.unwrap().path(), &path)
        {
            println!(
                "{} {}",
                if path.starts_with(a_path) { "<<" } else { ">>" },
                path.display()
            );
        } else {
            if path.is_file() {
                if !modified_times_match(&peek.unwrap().path(), &path)
                    && !has_same_contents(&peek.unwrap().path(), &path)
                {
                    println!(
                        "!= {}\n   {}",
                        path.display(),
                        peek.unwrap().path().display()
                    );
                }
            } else if path.is_dir() {
                diff(&path, &peek.unwrap().path());
            }
            iter.next();
        }
    }
}

fn types_match(a: &Path, b: &Path) -> bool {
    a.is_file() == b.is_file() || a.is_dir() == b.is_dir()
}

fn has_same_contents(a: &Path, b: &Path) -> bool {
    let mut a_reader = match File::open(a) {
        Ok(f) => BufReader::new(f),
        Err(err) => panic!("{}: {}", a.display(), err),
    };
    let mut b_reader = match File::open(b) {
        Ok(f) => BufReader::new(f),
        Err(err) => panic!("{}: {}", b.display(), err),
    };
    let mut a_buff = [0; 1024];
    let mut b_buff = [0; 1024];

    loop {
        let a_bytes_read = a_reader.read(&mut a_buff).unwrap();
        let b_bytes_read = b_reader.read(&mut b_buff).unwrap();

        if a_bytes_read != b_bytes_read || &a_buff[..a_bytes_read] != &b_buff[..b_bytes_read] {
            break false;
        } else if a_bytes_read == 0 {
            break true;
        }
    }
}

fn modified_times_match(a_path: &Path, b_path: &Path) -> bool {
    if let (Ok(a_meta), Ok(b_meta)) = (a_path.metadata(), b_path.metadata()) {
        if let (Ok(a_time), Ok(b_time)) = (a_meta.modified(), b_meta.modified()) {
            return a_time == b_time;
        }
    }

    false
}

fn remove_mac_files(entry: &fs::DirEntry) -> bool {
    let path = entry.path();
    let name = entry.file_name();

    match path.is_file() {
        true => {
            !(name == ".DS_Store"
                || name == ".iTunes Preferences.plist"
                || name == ".com.apple.timemachine.donotpresent"
                || name == ".com.apple.timemachine.supported"
                || name == ".VolumeIcon.icns")
        }
        false => {
            !(name == ".Spotlight-V100"
                || name == ".TemporaryItems"
                || name == ".Trashes"
                || name == ".fseventsd"
                || name == ".DocumentRevisions-V100")
        }
    }
}

fn get_files(a_path: &Path, b_path: &Path) -> Vec<fs::DirEntry> {
    let a_files = fs::read_dir(a_path).expect(&format!("{}", a_path.display()));
    let b_files = fs::read_dir(b_path).expect(&format!("{}", b_path.display()));

    let mut files = a_files
        .chain(b_files)
        .map(|entry| entry.unwrap())
        .filter(remove_mac_files)
        .collect::<Vec<_>>();

    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    files
}
