

use crate::ioddmodel11::IODevice::{RoleMenu, RoleSet};
use crate::ioddmodel11::ProfileBody::DeviceFunction::MenuSet;
use crate::ioddmodel11::ProfileBody::MenuCollection::*;
use crate::ioddmodel11::ProfileBody::VariableCollection::SingleValue;
use crate::ioddmodel11::{self, ProfileBody::VariableCollection::Variable};
use crate::standards::Standards;
use crate::utils::decoder::{get_booleant, get_float32t, get_integert, get_uintegert};
use crate::utils::format::convert_to_humanformat;
use bitvec::prelude::*;
use log::{info, trace, warn};
use serde_json::Value;
use anyhow::{Result,Error};
pub struct Engine<'a> {
    iodevice: &'a ioddmodel11::IODevice::IODevice,
    standards: Standards,
    language: String,
}
impl<'a> Engine<'a> {
    pub fn new(iodevice: &'a ioddmodel11::IODevice::IODevice, languagetype: &str) -> Self {
        info!("Loading Standards");
        let standards = Standards::new();
        let language = languagetype.to_lowercase();

        Engine {
            iodevice,
            standards,
            language,
        }
    }
    pub fn get_standards(&self) -> &Standards {
        &self.standards
    }
    pub fn get_iodevice(&self) -> &ioddmodel11::IODevice::IODevice {
        self.iodevice
    }

    pub fn getmenu(&self, role: &RoleSet, menu: &RoleMenu) -> &MenuSet {
        let userinterface = &self.iodevice.profilebody.device_function.userinterface;

        let role = userinterface.get_role_menu_set_by_name(role).unwrap();
        trace!("RoleMenuSet: {:?}", role);
        let menuset = role.get_menu_by_name(menu).unwrap();
        trace!("MenuSet: {:?}", menuset);
        menuset
    }

    pub fn findvariablebyindex(&self, index: i32) -> &Variable {
        //TODO Subindex if exists
        let varcoll = &self.iodevice.profilebody.device_function.variablecollection;
        let var = varcoll
            .variable
            .iter()
            .find(|x| x.index == index.to_string())
            .unwrap();
        var
    }
    pub fn findvariablebyid(&self, id: &str) -> Result<&Variable> {
        let varcoll = &self.iodevice.profilebody.device_function.variablecollection;
        let var = varcoll
            .variable
            .iter()
            .find(|x| x.id.to_lowercase() == id.to_lowercase());
        
        match var{
            Some(v) => Ok(v),
            None => match self.get_standards().definitions.variablecollection.variable
            .iter().find(|vs| vs.id == id) {
                    Some(x) => Ok(x),
                    None =>  Err(Error::msg("variable not found.")),
            }       
        }
    }
    pub fn getmenubyid(&self, id: String) -> Vec<&Menu> {
        let menus = &self
            .iodevice
            .profilebody
            .device_function
            .userinterface
            .menucollection
            .menu;
        let mut ml: Vec<&Menu> = Vec::new();
        info!("Checking for ID : {}", id);
        let mut m = menus.iter().find(|m| m.id.eq(&id)).unwrap();
        
        if !m.menuref.is_empty() {
            for mr in &m.menuref {
                info!("XxX MenuRef: {:?}", mr);
                let c = match mr.condition.as_ref() {
                    Some(x) => x,
                    None => {
                        m = menus.iter().find(|m| m.id == mr.menuid).unwrap();
                        ml.push(m);
                        continue;
                    }
                };

                let variable = c.variableId.clone();
                info!("Condition variable Name: {}", variable);
                let val = c.value;
                let variables = &self
                    .iodevice
                    .profilebody
                    .device_function
                    .variablecollection
                    .variable;
                let v = variables.iter().find(|v| v.id == variable).unwrap();

                // TODO : currently the default value is used
                // Query Device for current Variable Settings

                if v.defaultValue.parse::<u8>().unwrap() == val {
                    m = menus.iter().find(|m| m.id == mr.menuid).unwrap();
                    info!(
                        "Menu selected by Variable : {} with Value {}",
                        v.id, v.defaultValue
                    );
                    ml.push(m);
                }
            }
        } else {
            ml = vec![m];
        }

        ml
    }

    pub fn parse(&self, hexdata: &str) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let hex_pattern = regex::Regex::new(r"^[0-9A-Fa-f]+$").unwrap();
        //Validate hexstring
        if !hex_pattern.is_match(hexdata) {
            return Err("unknown hex string".into());
        }

        let pdin = &self
            .iodevice
            .profilebody
            .device_function
            .processdatacollection
            .processdata
            .processdatain;

        let id = &self.getmenu(&RoleSet::Observer, &RoleMenu::Observation);
        info!("Oberservation Menu Id: {}", id.menuid);
        let menu = self.getmenubyid(id.menuid.clone());
        info!("Parse->Menu: {:?}", menu);
        Ok(self.parsedata(hexdata, pdin, menu))
    }

    fn parsedata(
        &self,
        hexstring: &str,
        pdin: &ioddmodel11::ProfileBody::ProcessDataCollection::ProcessDataIn,
        menu: Vec<&Menu>,
    ) -> Vec<DataPoint> {
        let mut dp: Vec<DataPoint> = Vec::new();
        let bitvec = Self::hex_string_to_bitvec(hexstring).unwrap();
        info!("HexString: {}, BitVec: {:?}", hexstring, bitvec);
        let _id = &pdin.Name.textid;
        let _datalength = pdin.bitlength;

        for dt in &pdin.datatype {
            match dt.datatype.as_str() {
                "RecordT" => {
                    dp.extend(self.parse_record_t(&bitvec, &dt.record_items, &menu));
                }
                "IntegerT" => {
                    //Engine::process_integert(&mut dp, self, rc, menu, bitandbytes::convert_slice(slice_bytes), &mut 0, &mut 0);
                }
                "UIntegerT" => {
                    //Engine::process_uintegert(&mut dp, self, rc, menu, bitandbytes::convert_slice(slice_bytes), &mut 0, &mut 0);
                }
                "Float32T" => {
                    panic!("Implement me");
                }
                "BooleanT" => {
                    panic!("Implement me");
                }
                "StringT" => {
                    panic!("Implement me");
                }
                "OctetStringT" => {
                    panic!("Implement me");
                }
                "TimeT" => {
                    panic!("Implement me");
                }
                "TimeSpanT" => {
                    panic!("Implement me");
                }
                "ArrayT" => {
                    panic!("Implement me - ArrayT")
                }
                _ => {
                    warn!("no corresponding DataType found.")
                }
            }
        }

        dp
    }

    fn parse_record_t(
        &self,
        bitvec: &BitVec,
        recorditm: &Vec<ioddmodel11::ProfileBody::ProcessDataCollection::RecordItem>,
        menu: &Vec<&Menu>,
    ) -> Vec<DataPoint> {
        let mut data: Vec<DataPoint> = Vec::new();
        let mut bitcounter: usize = 0;
        let mut counter = 0;

        for rc in recorditm {
            let _bitoffset: usize = rc.bit_offset as usize;

            match rc.datatype.datatype.as_str() {
                "UIntegerT" => {
                    info!("Processing UIntegerT");
                    Engine::process_uintegert(
                        &mut data,
                        self,
                        rc,
                        menu,
                        bitvec,
                        &mut bitcounter,
                        &mut counter,
                    );
                }
                "BooleanT" => {
                    info!("Processing BooleanT");
                    Engine::process_booleant(
                        &mut data,
                        self,
                        rc,
                        menu,
                        bitvec,
                        &mut bitcounter,
                        &mut counter,
                    );
                }
                "IntegerT" => {
                    info!("Processing IntegerT");
                    Engine::process_integert(
                        &mut data,
                        self,
                        rc,
                        menu,
                        bitvec,
                        &mut bitcounter,
                        &mut counter,
                    );
                }
                "Float32T" => {
                    info!("Processing Float32T");
                    Engine::process_float32t(
                        &mut data,
                        self,
                        rc,
                        menu,
                        bitvec,
                        &mut bitcounter,
                        &mut counter,
                    );
                }
                _ => {
                    println!("Found something else");
                }
            }
        }
        data
    }

    fn process_float32t(
        data: &mut Vec<DataPoint>,
        engine: &Engine,
        rc: &ioddmodel11::ProfileBody::ProcessDataCollection::RecordItem,
        menu: &Vec<&Menu>,
        bitvec: &BitVec,
        bitcounter: &mut usize,
        counter: &mut usize,
    ) {
        let bitlength = rc.datatype.bit_length as usize;
        let res = get_float32t(bitvec, rc.bit_offset as usize);
        *bitcounter += bitlength;

        let (translatedname, translateddesc) =
            engine.resolve_names(&rc.name.textid, &rc.description.textid);

        let rcitemref = Self::find_record_item(menu, rc.subindex as usize);

        let total = res as f64;
        let (abbr, total) = Self::process_recorditemref(engine, &rcitemref, total);

        *counter += 1;
        data.push(DataPoint {
            name: translatedname,
            description: translateddesc,
            value: total.into(),
            unit: abbr.to_string(),
        });
    }

    fn process_uintegert(
        data: &mut Vec<DataPoint>,
        engine: &Engine,
        rc: &ioddmodel11::ProfileBody::ProcessDataCollection::RecordItem,
        menu: &Vec<&Menu>,
        bitvec: &BitVec,
        bitcounter: &mut usize,
        counter: &mut usize,
    ) {
        let bitlength = rc.datatype.bit_length as usize;
        let res = get_uintegert(bitvec, rc.bit_offset as usize, bitlength);
        *bitcounter += bitlength;

        let (translatedname, translateddesc) =
            engine.resolve_names(&rc.name.textid, &rc.description.textid);

        let rcitemref = Self::find_record_item(menu, rc.subindex as usize);
        let total = res as f64;
        let (abbr, total) = Self::process_recorditemref(engine, &rcitemref, total);
        *counter += 1;
        data.push(DataPoint {
            name: translatedname,
            description: translateddesc,
            value: total.into(),
            unit: abbr.to_string(),
        });
    }

    fn process_booleant(
        data: &mut Vec<DataPoint>,
        engine: &Engine,
        rc: &ioddmodel11::ProfileBody::ProcessDataCollection::RecordItem,
        _menu: &[&Menu],
        bitvec: &BitVec,
        bitcounter: &mut usize,
        counter: &mut usize,
    ) {
        let _bitlength = rc.datatype.bit_length as usize;

        let boolean = get_booleant(bitvec, *bitcounter);
        *bitcounter += 1;

        let (translatedname, translateddesc) =
            engine.resolve_names(&rc.name.textid, &rc.description.textid);

        let mut res = boolean.to_string();
        if !rc.datatype.singlevalue.is_empty() {
            if let Some(x1) = rc
                .datatype
                .singlevalue
                .iter()
                .find(|s| s.value == boolean.to_string())
            {
                res = engine.resolve_textid(x1.name.textid.as_str(), engine.language.as_str());
            }
        }

        let dp = DataPoint {
            name: translatedname,
            description: translateddesc,
            value: res.into(),
            unit: String::new(),
        };
        *counter += 1;
        data.push(dp);
    }

    fn process_integert(
        data: &mut Vec<DataPoint>,
        engine: &Engine,
        rc: &ioddmodel11::ProfileBody::ProcessDataCollection::RecordItem,
        menu: &Vec<&Menu>,
        bitvec: &BitVec,
        bitcounter: &mut usize,
        counter: &mut usize,
    ) {
        let (translatedname, translateddesc) =
            engine.resolve_names(&rc.name.textid, &rc.description.textid);
        info!("Calculating {}", rc.name.textid.clone());

        let res = get_integert(
            bitvec,
            rc.bit_offset as usize,
            rc.datatype.bit_length as usize,
        );
        info!("Calculation done with Result: {}", res);

        let rcitemref = Self::find_record_item(menu, rc.subindex as usize);
        let total = res as f64;
        let (abbr, total) = Self::process_recorditemref(engine, &rcitemref, total);

        if let Some(x) = engine.validate_single_value(&rc.datatype.singlevalue, res) {
            info!("Single Value found.");
            let dp = DataPoint {
                name: translatedname.clone(),
                description: translateddesc.clone(),
                value: x.into(),
                unit: abbr.to_string(),
            };
            data.push(dp);
            return;
        }

        *bitcounter += rc.datatype.bit_length as usize;

        let dp = DataPoint {
            name: translatedname,
            description: translateddesc,
            value: total.into(),
            unit: abbr.to_string(),
        };
        data.push(dp);
        *counter += 1;
    }

    fn find_record_item(menu: &Vec<&Menu>, subindex: usize) -> Option<RecordItemRef> {
        for m in menu {
            for i in &m.recorditemref {
                if i.subindex as usize == subindex {
                    return Some(i.clone());
                }
            }
        }
        None
    }

    fn hex_string_to_bitvec(hex_string: &str) -> Option<BitVec> {
        let hex_string = hex_string.trim_start_matches("0x");

        // Parse the hexadecimal string
        match hex::decode(hex_string) {
            Ok(bytes) => {
                let mut bit_vec = BitVec::new();
                for byte in bytes.iter().rev() {
                    for i in 0..8 {
                        bit_vec.push((byte >> i) & 1 == 1);
                    }
                }
                Some(bit_vec)
            }
            Err(_) => None, // Invalid hexadecimal string
        }
    }
    fn process_recorditemref(
        engine: &Engine,
        rc: &Option<RecordItemRef>,
        total: f64,
    ) -> (String, f64) {
        info!("PROCESS_recorditemref: Total:{}, RC: {:?}", total, rc);

        let abbr = match rc {
            Some(rc) => match rc.unitcode {
                Some(uc) => engine.standards.get_abbr_by_code(uc),
                _ => String::new(),
            },
            _ => String::new(),
        };
        let total = match rc {
            Some(x) => convert_to_humanformat(x.gradient, x.displayformat.clone(), x.offset, total),
            _ => total,
        };
        (abbr, total)
    }

    fn validate_single_value(&self, singlevalues: &Vec<SingleValue>, res: i32) -> Option<String> {
        if singlevalues.is_empty() {
            return None;
        }
        info!("SingleValue Raw: {}", res);
        let x = singlevalues.iter().find(|sv| sv.value == res.to_string());

        x.map(|sv| self.resolve_textid(&sv.name.textid, self.language.as_str()))
    }

    fn resolve_names(&self, name_textid: &str, desc_textid: &str) -> (String, String) {
        (
            self.resolve_textid(name_textid, self.language.as_str()),
            self.resolve_textid(desc_textid, self.language.as_str()),
        )
    }
    pub fn resolve_textid(&self, textid: &str, languagetype: &str) -> String {
        if textid.is_empty() {
            return String::new();
        }
        trace!("Looking for {} in Language {}", textid, languagetype);
        let result = self.lookup_locale(textid, languagetype);
        match result {
            Some(x) => x,
            None => {
                // try fallback
                self.lookup_locale(textid, "en").unwrap()
            }
        }
    }

    fn lookup_locale(&self, textid: &str, languagetype: &str) -> Option<String> {
        let external_text_collection = &self.get_iodevice().externaltextcollection;
        let mut selected_language = external_text_collection
            .language
            .iter()
            .find(|language| language.language == languagetype);

        selected_language = match selected_language {
            Some(lang) => Some(lang),
            _ => None,
        };
        // Look in TargetLanguage in iodd
        let selected_text = selected_language
            .unwrap()
            .text
            .iter()
            .find(|id| id.id == textid);

        // Standard
        match selected_text {
            Some(x) => Some(x.value.clone()),
            None => {
                let selected_lang = self
                    .get_standards()
                    .definitions
                    .externaltextcollection
                    .iter()
                    .find(|language| language.language == languagetype)
                    .expect("Translation not found.");
                let selected_text = selected_lang.text.iter().find(|id| id.id == textid);

                selected_text.map(|x| x.value.clone())
            }
        }
    }
}

pub struct DataPoint {
    pub name: String,
    pub description: String,
    pub value: Value,
    pub unit: String,
}


/// this function should return IO Link specific errors 
/// there is no overview about the errorcodes
pub fn get_error(val : &str) -> String{

    match val{
        "8030" => "Value out of Range".to_string(),
        _ => "unknown error code".to_string()
    }

}