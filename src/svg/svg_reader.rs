use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use super::svg_structs;

enum BlockReturn {
    Element(svg_structs::SVGgroup),
    Closure
}

pub fn read_svg(file: &mut BufReader<File>) -> svg_structs::SVGgroup {
    let mut buffer = Vec::new();
    file.read_until('<' as u8, &mut buffer).expect("Error starting read");
    let main_block = obtain_block(file);
    main_block
}

pub fn parse_data(text: String) -> HashMap<String, String>{
    let data = HashMap::new();
    data
}

pub fn obtain_block(file: &mut BufReader<File>) -> BlockReturn {

    let mut hasChildren = true;
    let mut returnType: BlockReturn = BlockReturn::element;

    let mut name_utf: Vec<u8> = Vec::new();
    let mut data_utf: Vec<u8> = Vec::new();

    file.read_until(' ' as u8, &mut name_utf).expect("XD");
    file.read_until('>' as u8, &mut data_utf).expect("XDDD");

    let name = String::from_utf8(name_utf).unwrap();
    let data = String::from_utf8(data_utf).unwrap();

    if data.chars().last().unwrap() == '/' {
        hasChildren = false;
    }
    if data.chars().next().unwrap() == '/' {
        returnType = BlockReturn::closure;

    }

    svg_structs::SVGgroup::new(name, parse_data(data))
}