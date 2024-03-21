use yaserde::*;

use crate::ioddmodel11::ProfileBody::common::*;

use super::VariableCollection::SingleValue;


#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "ProcessDataCollection")]
pub struct ProcessDataCollection{
    #[yaserde(child,rename = "ProcessData")]
    pub processdata : ProcessData,
    pub id : u8,
}

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "ProcessData")]
pub struct ProcessData{
    #[yaserde(attribute, rename="id")]
    pub id : String,

    #[yaserde(child, rename="Condition")]
    pub condition : Condition,

    #[yaserde(child, rename="ProcessDataIn")]
    pub processdatain : ProcessDataIn,

    #[yaserde(child, rename="ProcessDataOut")]
    pub processdataout : ProcessDataOut,

}
#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "ProcessDataIn")]
pub struct ProcessDataIn{
    #[yaserde(attribute)]
    pub id : String,
    
    #[yaserde(attribute, rename="bitLength")]
    pub bitlength: u8,

    #[yaserde(child, rename="Datatype")]
    pub datatype : Vec<DataType>,

    #[yaserde(child)]
    pub datatyperef : DataTypeRef,

    #[yaserde(child)]
    pub Name : LookupValue,

}



#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "ProcessDataOut")]
pub struct ProcessDataOut{
    #[yaserde(attribute)]
    pub id : String,
    
    #[yaserde(attribute)]
    pub bitlength: u8,

    #[yaserde(child)]
    pub datatype : DataType,

    #[yaserde(child)]
    pub datatyperef : DataTypeRef,


    #[yaserde(child)]
    pub name : LookupValue,

}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename_all = "PascalCase", prefix = "xsi")]
pub struct DataType {
    #[yaserde(rename = "type", attribute)]
    pub datatype: String,
    #[yaserde(rename = "bitLength", attribute)]
    pub bitlength: u8,
    #[yaserde(rename = "count", attribute)]
    pub count: u8,
    #[yaserde(rename = "Name")]
    pub name: LookupValue,
    #[yaserde(rename = "RecordItem")]
    pub record_items: Vec<RecordItem>,
}
#[derive(Debug,  YaDeserialize)]
pub struct RecordItem {
    #[yaserde(rename = "subindex", attribute)]
    pub subindex: u8,
    #[yaserde(rename = "bitOffset", attribute)]
    pub bit_offset: u16,
    #[yaserde(rename = "SimpleDatatype")]
    pub datatype: SimpleDatatype,
    #[yaserde(rename = "Name")]
    pub name: LookupValue,
    #[yaserde(rename = "Description")]
    pub description : LookupValue,
    #[yaserde(rename = "DatatypeRef")]
    pub datatyperef : DataTypeRef
}

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "DataTypeRef")]
pub struct DataTypeRef{
    #[yaserde(attribute, rename = "datatypeId")]
    pub datatypeid : String
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(prefix = "xsi", rename = "SimpleDatatype")]
pub struct SimpleDatatype {
    #[yaserde(attribute, rename = "type")]
    pub datatype: String,
    #[yaserde(attribute, rename = "bitLength")]
    pub bit_length: u8,
    #[yaserde(rename = "encoding")]
    pub encoding: String,
    #[yaserde(rename = "ValueRange")]
    pub valuerange : Option<ValueRange>,
    #[yaserde(rename = "SingleValue")]
    pub singlevalue : Vec<SingleValue>
}


#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "Condition")]
pub struct Condition{

    #[yaserde(attribute)]
    pub variable : String,

    #[yaserde(attribute)]
    pub subindex : u8,
    
    #[yaserde(attribute)]
    pub value : u8,
    
}