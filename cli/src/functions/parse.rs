use std::fs;
use std::io;

use ioddengine::parser::Parser;
fn read_file_as_string(file_path: &str) -> Result<String, io::Error> {
    // Attempt to read the contents of the file
    fs::read_to_string(file_path)
}

pub fn parse(filepath : &str){

    let content = match read_file_as_string(filepath) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            panic!("cannot find file");
        }
    };
    let iodevice = Parser::parse(content.as_str());

    println!("IODeviceVariableColl: {:?}", iodevice.profilebody.device_function.variablecollection);
}