use crate::ioddmodel11;
use crate::ioddmodel11::ExternalTextCollection::Language;
use std::fs;
use crc32fast::Hasher;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::Writer;
use std::error::Error;
use std::io::BufRead;
use yaserde::YaDeserialize;
use log::info;

pub struct Parser {
   
    pub iodevice: ioddmodel11::IODevice::IODevice,
}

impl Parser {
  
    pub fn new(drivername: String, files: Vec<(String, Vec<u8>)>) -> Self {
        let basefilename = format!("{}.xml", drivername);
        let basefile = files
            .iter()
            .find(|(file_name, _)| file_name == &basefilename)
            .map(|(_, file_data)| file_data);
    
        let basefile_as_str = String::from_utf8(basefile.unwrap().to_vec()).unwrap();
        let mut iodevice = Parser::parse(basefile_as_str.as_str());
    
        // Extract externaltextcollection as a reference
        let langcoll = &mut iodevice.externaltextcollection; // mutable reference
    
        Parser::parse_languages(drivername.clone(), files,  &mut langcoll.language);
        info!("all parsed.");
        Parser {
            
            iodevice,
        }
    }

    pub fn parse_languages(
        drivername: String,
        files: Vec<(String, Vec<u8>)>,
        languagecoll: &mut Vec<Language>,
    ) {
        let filtered_files: Vec<(String, Vec<u8>)> = files
            .into_iter()
            .filter(|(filename, _)| filename.starts_with(&format!("{}-", drivername)))
            .collect();
    
        for languagefile in filtered_files {
            let xml = String::from_utf8(languagefile.1.to_vec()).unwrap();
            let language = Parser::parse_section::<ioddmodel11::ExternalTextCollection::Language>(
                &xml,
                "Language",
            );

            languagecoll.push(language);
        }
    }
    
    
    pub fn parse(xml_data: &str) -> ioddmodel11::IODevice::IODevice {
        let language = Parser::parse_section::<ioddmodel11::ExternalTextCollection::Language>(xml_data, "PrimaryLanguage");
        let communication = Parser::parse_section::<ioddmodel11::CommNetworkProfile::CommNetworkProfile>(xml_data, "CommNetworkProfile");
        let process_data_collection = Parser::parse_section::<ioddmodel11::ProfileBody::ProcessDataCollection::ProcessDataCollection>(xml_data, "ProcessDataCollection");
        let profile_header = Parser::parse_section::<ioddmodel11::ProfileHeader::ProfileHeader>(xml_data, "ProfileHeader");
        let user_interface = Parser::parse_section::<ioddmodel11::ProfileBody::DeviceFunction::UserInterface>(xml_data, "UserInterface");
        let variablecollection = Parser::parse_section::<ioddmodel11::ProfileBody::VariableCollection::VariableCollection>(xml_data, "VariableCollection");

        let df = ioddmodel11::ProfileBody::DeviceFunction::DeviceFunction {
            userinterface: user_interface,
            eventcollection: None,
            variablecollection,
            features: None,
            errortypecollection: None,
            datatypecollection: None,
            processdatacollection: process_data_collection,
        };

        let profile_body = ioddmodel11::ProfileBody::ProfileBody {
            device_identity: None,
            device_function: df,
        };

        let externaltc = ioddmodel11::ExternalTextCollection::ExternalTextCollection {
            language: vec![language],
        };

        ioddmodel11::IODevice::IODevice {
            commnetworkprofile: Some(communication),
            document_info: None,
            profileheader: Some(profile_header),
            externaltextcollection: externaltc,
            profilebody: profile_body,
        }
    }

    fn parse_section<T>(xml_data: &str, section_name: &str) -> T
    where
        T: YaDeserialize,
    {
        let section_xml = Parser::findandprepare(xml_data, section_name).unwrap();
        yaserde::de::from_str(&section_xml).unwrap()
    }
    /// Old Function for IODD 1.0
    #[allow(unused)]
    fn replace_in_process_data_collection(
        pdc: &mut ioddmodel11::ProfileBody::ProcessDataCollection::ProcessDataCollection,
        dtc: &ioddmodel11::ProfileBody::DeviceFunction::DataTypeCollection,
    ) {
        for item in &mut pdc.processdata.processdatain.datatype {
            for x in &mut item.record_items {
                if !x.datatyperef.datatypeid.is_empty() {
                    println!("Iam replacing.{}.", x.datatyperef.datatypeid);
                    if let Some(dt) = dtc
                        .datatype
                        .iter()
                        .find(|dt| dt.id == x.datatyperef.datatypeid)
                    {
                        x.datatype.datatype = dt.datatype.clone();
                        x.datatype.bit_length = dt.bitlength;
                    } else {
                        // Handle the case where the data type is not found
                        // You might want to log an error message or take other actions
                    }
                }
            }
        }
    }

    fn add_xmlns_xsi_to_comm_network_profile(xml_data: &str) -> String {
        // Find the index of the closing angle bracket (>) of the opening tag of CommNetworkProfile
        let mut modified_xml = String::new();
        if let Some(index) = xml_data.find('>') {
            // Insert the namespace declaration just before the first attribute
            modified_xml.push_str(&xml_data[..index]);
            modified_xml.push_str(" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"");
            modified_xml.push_str(&xml_data[index..]);
        } else {
            modified_xml.push_str(xml_data);
        }
        modified_xml
    }

    pub fn findandprepare(xml: &str, nodename: &str) -> Result<String, Box<dyn Error>> {
        let ns = Parser::getnodestring(nodename, xml);
        let xml_with_namespace = Parser::add_xmlns_xsi_to_comm_network_profile(&ns);
        Ok(xml_with_namespace)
    }
    
    fn getnodestring(nodename : &str, xml_data : &str) -> String{
        let mut reader = Reader::from_str(xml_data);
        let mut buf = Vec::new();
        let mut junk_buf: Vec<u8> = Vec::new();
    
        let nodeu8 = nodename.as_bytes();
        loop {
           match reader.read_event_into(&mut buf) {
                Err(e) => panic!(
                    "Error at position {}: {:?}",
                    reader.buffer_position(),
                    e
                ),
                Ok(Event::Eof) =>{ return "".to_string()},
                Ok(Event::Start(e)) => {
                    if e.name() == quick_xml::name::QName(nodeu8){
                       
                            let release_bytes = Parser::read_to_end_into_buffer(
                                &mut reader,
                                &e,
                                &mut junk_buf
                            ).unwrap();
                           
                            let str = std::str::from_utf8(&release_bytes)
                                .unwrap();
                            return str.to_string();
                        }
                        
                    }
                
                // other unimportantnodeu8 => { Events
                _ => (),
            }
            // clear buffer to avoid lstreaking memory
            buf.clear();
        }
        
    }
    fn read_to_end_into_buffer<R: BufRead>(
        reader: &mut Reader<R>,
        start_tag: &BytesStart,
        junk_buf: &mut Vec<u8>,
    ) -> Result<Vec<u8>, quick_xml::Error> {
        let mut depth = 0;
        let mut output_buf: Vec<u8> = Vec::new();
        let mut w = Writer::new(&mut output_buf);
        let tag_name = start_tag.name();
        w.write_event(Event::Start(start_tag.clone()))?;
        loop {
            junk_buf.clear();
            let event = reader.read_event_into(junk_buf)?;
            w.write_event(&event)?;

            match event {
                Event::Start(e) if e.name() == tag_name => depth += 1,
                Event::End(e) if e.name() == tag_name => {
                    if depth == 0 {
                        return Ok(output_buf);
                    }
                    depth -= 1;
                }
                Event::Eof => {
                    panic!("oh no")
                }
                _ => {}
            }
        }
    }

    fn crccheck(fname: &str) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(fname).expect("Unable to read file");
        // Process the XML content
        let mut modified_content = String::new();
        let mut crc_from_xml: Option<u32> = None;

        for line in contents.lines() {
            // Find and remove the value of crc attribute
            if let Some(index) = line.find("crc=\"") {
                let start_index = index + 5; // Start index of crc value
                let end_index = line[start_index..]
                    .find('"')
                    .map_or(line.len(), |pos| start_index + pos);
                let crc_value = &line[start_index..end_index]; // Extract the crc value
                if !crc_value.is_empty() {
                    crc_from_xml = Some(crc_value.parse()?); // Parse crc value if not empty
                }
                // Replace crc value with empty string
                let modified_line = format!("{}{}", &line[..start_index], "");
                modified_content.push_str(&modified_line);
                modified_content.push_str(&line[end_index..]); // Append the rest of the line
            } else {
                modified_content.push_str(line);
            }
            modified_content.push('\n'); // Add newline character
        }

        println!("{}", modified_content);

        let mut hasher = Hasher::new();
        hasher.update(modified_content.as_bytes());
        let crc_calculated = hasher.finalize();
        // Calculate the CRC checksum over modified XML content

        println!("Calculated CRC32 checksum: {}", crc_calculated);

        if let Some(crc_xml) = crc_from_xml {
            if crc_calculated == crc_xml {
                println!("CRC hashes match!");
            } else {
                println!("CRC hashes do not match!");
            }
        } else {
            println!("No CRC hash found in the XML.");
        }
        Ok(())
    }
}
