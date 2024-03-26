use std::fs::File;
use std::io::{self, Read};
use yaserde::*;

use log::info;

use std::fs;

use crate::{ioddmodel11, parser::*};

use self::units::UnitCollection;
use self::definitions::Definitions;
pub mod units;
pub mod definitions;




pub struct Standards {
    pub units : UnitCollection,
    pub definitions : Definitions
}
impl Default for Standards {
         fn default() -> Self {
             Self::new()
         }
}
impl Standards {

    pub fn new() -> Self {
        info!("Loading units");
        let units = Standards::parseunits();
        // TODO Consider lazy loading only if required.
        info!("Loading definitions");
        let definitions = Standards::parsedefinitions();
        Standards{ units, definitions }
    }
    

    pub fn get_abbr_by_code(&self, code : u32) -> String{
        let unit = self.units.units.iter().find(| u | u.code == code);
        match unit{
            Some(x) => x.abbr.clone(),
            _ => String::new(),
        }
    }

    pub fn parseunits() -> UnitCollection{
        let xml_data = Standards::load_file("../data/specs/IODD-StandardUnitDefinitions1.1.xml").unwrap();        
        let ex = Parser::findandprepare(xml_data.as_str(), "UnitCollection").unwrap();        
        let _unit: UnitCollection = yaserde::de::from_str(&ex).unwrap();
        _unit
    }
    ///TODO: harmonize with parser parse_languages
    pub fn parsedefinitions() -> Definitions{
        let xml_data = Standards::load_file("../data/specs/IODD-StandardDefinitions1.1.xml").unwrap();        
        let variablecollection = Standards::parse_section::<ioddmodel11::ProfileBody::VariableCollection::VariableCollection>(xml_data.as_str(), "VariableCollection");
        let language = Standards::parse_section::<ioddmodel11::ExternalTextCollection::Language>(
            &xml_data,
            "PrimaryLanguage",
        );
        
        let mut languages =vec![language];

        //load additional Languages 
        match Standards::read_files_to_vec("../data/specs/") {
            Ok(files) => {
                Parser::parse_languages("IODD-StandardDefinitions1.1".to_string(), files, &mut languages);
                info!("Standard Languages files parsed.");
            }
           
            Err(e) => println!("Error: {}", e),
        }
        info!("Len Languages: {}", languages.len());
        Definitions{ variablecollection, externaltextcollection: languages}
        
    }

    fn read_files_to_vec(directory_path: &str) -> io::Result<Vec<(String, Vec<u8>)>> {
        let mut files_data: Vec<(String, Vec<u8>)> = Vec::new();
    
        // Iterate through the directory
        for entry in fs::read_dir(directory_path)? {
            let entry = entry?;
            let path = entry.path();
    
            // Check if the path is a file
            if path.is_file() {
                // Read the file into a byte vector
                let file_data = fs::read(&path)?;
    
                // Get the filename as a string
                let filename = path.file_name().unwrap().to_string_lossy().into_owned();
    
                // Push the filename and file data into the vector
                files_data.push((filename, file_data));
            }
        }
    
        Ok(files_data)
    }

    fn parse_section<T>(xml_data: &str, section_name: &str) -> T
    where
        T: YaDeserialize,
    {
        let section_xml = Parser::findandprepare(xml_data, section_name).unwrap();
        yaserde::de::from_str(&section_xml).unwrap()
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
}


#[cfg(test)]
mod tests{
    use super::*;

    // Define a test function
    #[test]
    fn load_definitions() {
        let _definitions = Standards::parsedefinitions();
        assert_eq!(true,true);
    }
}