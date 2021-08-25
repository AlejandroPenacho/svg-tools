mod utils;

fn main() {
    println!("Hello, world!, I have a {}", 23);

    let file_path = String::from("/home/alejandro/Documents/Programming/Rust/svg-tools/files/test.txt");

    let bytes = utils::file_handle::get_file_iterator(file_path);

    
    for letter in bytes {
        println!("{}", letter);
    }

}
