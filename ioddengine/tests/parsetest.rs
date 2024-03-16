// Import the library code


use std::fs;
use std::fs::File;
use std::io::{self, Read};

use ioddengine::parser::Parser;
// Define a test function
#[test]
fn test_defaultfiles() {
    let dir_path = "/home/ulrich/Documents/ifm/iotcoreandiodd/data/Examples/"; // You can change this to the path of your desired directory
    let entries = fs::read_dir(dir_path).unwrap();
   
    for entry in entries {
        let file_name = entry.unwrap().file_name().into_string().unwrap();
        
        if file_name.ends_with(".xml") {
            println!("FileName: {}", file_name);
            let fullpath = format!("{}{}", dir_path, &file_name);
            let fc = load_file(&fullpath).unwrap();
            let _p = Parser::parse(fc.as_str());
        }
    }

    assert!(true)
}

fn load_file(fname: &str) -> Result<String, io::Error> {
    // Open the file in read-only mode
    let mut file = match File::open(fname) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // Read the contents of the file into a String
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}
