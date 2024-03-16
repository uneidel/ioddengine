use yaserde::*;

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "ValueRange")]
pub struct ValueRange{
    #[yaserde(rename = "lowerValue", attribute)]
    pub lowervalue :f64,
    #[yaserde(rename = "upperValue", attribute)]
    pub uppervalue :f64
}


#[derive(YaDeserialize,Default, Debug)]
pub struct LookupValue{
    #[yaserde(attribute, rename="textId")]
    pub textid : String 
}

