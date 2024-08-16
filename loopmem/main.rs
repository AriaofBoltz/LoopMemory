use std::fs::File;
use std::io::{BufReader, Read, Error};

fn read_file(filename: &str) -> Result<String, Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

fn display_content(content: &str) {
    println!("Content from .looplog file:\n{}", content);
}

fn main() {
    let filename = "filename.looplog"; // Replace with your actual filename
    match read_file(&filename) {
        Ok(content) => display_content(&content),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
