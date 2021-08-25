mod utils;
mod svg;

fn main() {
    println!("Hello, world!, I have a {}", 23);

    let file_path = String::from("/home/alejandro/Documents/Programming/Rust/svg-tools/files/test.txt");

    let mut reader = utils::file_handle::get_file_reader(file_path);

    let svg = svg::svg_reader::read_svg(&mut reader);
}
