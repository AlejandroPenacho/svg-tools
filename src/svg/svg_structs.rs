use std::collections::HashMap;

enum BlockReturn {
    element,
    closure
}

pub struct SVGgroup {
    name: String,
    data: HashMap<String, String>,
    children: Vec<Box<dyn SVG>>
}

pub struct SVGelement {
    name: String,
    data: String
}

pub struct SVGtext {
    text: String
}

trait SVG {
    fn to_string(&self) -> String;
}

impl SVGgroup {
    pub fn new(name: String, data: HashMap<String, String>) -> SVGgroup {
        SVGgroup {
            name: name,
            data: data,
            children: Vec::new()
        }
    }
}