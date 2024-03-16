use yaserde::*;


#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "ValueRange")]
pub struct Unit {
    #[yaserde(rename = "code", attribute)]
    pub code: u32,
    #[yaserde(rename = "abbr", attribute)]
    pub abbr: String,
    #[yaserde(rename = "textId", attribute)]
    pub textid: String,
}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "UnitCollection")]
pub struct UnitCollection {
    #[yaserde(rename = "Unit")]
    pub units: Vec<Unit>,
    
}
