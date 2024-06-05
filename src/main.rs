use std::fs;
use std::process::exit;

use xml::reader::{EventReader, XmlEvent};


fn main() {
    let file_path = "docs.gl/gl4/glClear.xhtml";
    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {file_path}: {err}");
        exit(1)
    });

    println!("Length of {file_path} is {length}", length = content.len());
}



