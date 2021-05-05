use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get_file(path: &Path) -> File {
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    file
}

pub fn get_file_as_string(path: &Path) -> String {
    let mut content = String::new();
    let mut file = get_file(path);

    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(data) => data,
    };
    content
}
