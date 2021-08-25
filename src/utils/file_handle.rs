
use std::fs::File;
use std::io::BufReader;


pub fn get_file_reader(file_path: String) -> BufReader<File> {

    let reader = BufReader::new(File::open(file_path).expect("Error opening file")) ;

    reader
}

