use yaserde::*;




/* 

#[derive(Debug,  YaDeserialize)]
pub struct RecordItem {
    #[yaserde(rename = "subindex", attribute)]
    subindex: u8,
    #[yaserde(rename = "bitOffset", attribute)]
    bit_offset: u16,
    #[yaserde(rename = "SimpleDatatype")]
    datatype: SimpleDatatype,
    #[yaserde(rename = "Name")]
    name: Name,
}
impl Default for RecordItem {
    fn default() -> Self {
        // Define the logic to create a default RecordT instance
        RecordItem {
            subindex: 0,
            bit_offset:0,
            datatype: SimpleDatatype{ datatype:"".to_string(), bit_length:0},
            name: Name{text_id: "".to_string()}
            
        }
    }
}
*/
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(prefix = "xsi", rename = "SimpleDatatype")]
pub struct SimpleDatatype {
    #[yaserde(attribute, rename = "type")]
    pub datatype: String,
    #[yaserde(attribute, rename = "bitLength")]
    pub bit_length: u8,
}
#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename_all = "PascalCase")]
pub struct Name {
    #[yaserde(rename = "textId", attribute)]
    pub text_id: String,
}
