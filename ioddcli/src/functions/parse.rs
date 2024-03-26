use std::{fs, path::Path};


use ioddengine::parser::Parser;

pub fn parse_single_xml(filepath : &str){

    let content = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            panic!("cannot find file");
        }
    };
    let iodevice = Parser::parse(content.as_str());

    println!("IODevice: {iodevice:?}");
}

pub fn parse_zip_file(file_path : &str){
    let result = fs::read(file_path);
    let content = match result {
        Ok(file_content) => file_content,
        Err(err) => {
            // Handle error
            panic!("Error reading file: {}", err);
        }
    };
    let result = ioddengine::catalog::Catalog::extract_zip_to_vec(&content).unwrap_or_else(|err| panic!("error handling zip file"));
    let drivername = result.iter()
        .filter_map(|(filename, _)| {
            if filename.ends_with(".xml") {
                Some(filename)
            } else {
                None
            }
        })
        .min_by_key(|filename| filename.len())
        .map(|filename| filename.as_str()).expect("this is not a valid zip.");

        let driver = Path::new(drivername).file_stem().unwrap();
        println!("Drivername: {driver:?}");
        let p = Parser::new(driver.to_str().unwrap().to_string(), result);
        let iodevice = p.iodevice;
        println!("IODevice: {iodevice:?}");
}