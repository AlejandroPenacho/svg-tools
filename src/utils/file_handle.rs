
use std::fs::File;
use std::io::Read;

pub fn get_file_iterator(file_path: String) -> std::iter::Map<std::io::Bytes<std::fs::File>, fn(std::result::Result<u8, std::io::Error>) -> char > {

    let file = File::open(file_path).expect("Failed opening file");
    let bytes = file.bytes();
    bytes.map(|x| x.expect("Error reading byte") as char)
}

