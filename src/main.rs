
use std::io;
use std::fs::File;
use std::process::exit;
use xml::reader::{XmlEvent, EventReader};


fn read_entire_xml_file(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
        }
    }
    Ok(content)
}

fn main() {
    let file_path = "docs.gl/gl4/glClear.xhtml";
    println!("{content}", content = read_entire_xml_file(file_path).expect("TODO"));
}
            
