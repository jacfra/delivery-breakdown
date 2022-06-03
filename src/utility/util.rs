use rstring_builder::StringBuilder;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write_file(path: PathBuf, builder: StringBuilder) {
    let mut output = File::create(&path).unwrap();
    match write!(output, "{}", builder.to_string()) {
        Ok(_) => println!("{:?} created", path),
        Err(e) => println!("{:?}", e),
    };
}
pub fn create_directory(dir_path: PathBuf) {
    match fs::create_dir(&dir_path) {
        Ok(_) => println!(" dir at {:?} created", &dir_path),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => println!("dir at {:?} {}", dir_path, e.kind()),
            _ => println!("{:?}", e),
        },
    };
}
