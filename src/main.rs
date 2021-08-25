
use std::fs::File;
use std::io::Read;

fn get_file_iterator(file_path: String) -> std::iter::Map<std::io::Bytes<std::fs::File>, fn(std::result::Result<u8, std::io::Error>) -> char > {

    let file = File::open(file_path).expect("Failed opening file");
    let bytes = file.bytes();
    bytes.map(|x| x.expect("Error reading byte") as char)
}

fn main() {
    println!("Hello, world!, I have a {}", 23);

    let file_path = String::from("/home/alejandro/Documents/Programming/Rust/svg-tools/files/test.txt");

    let bytes = get_file_iterator(file_path);

    
    for letter in bytes {
        println!("{}", letter);
    }

}
