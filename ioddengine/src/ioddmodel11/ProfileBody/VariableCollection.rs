use yaserde::*;

use crate::ioddmodel11::ProfileBody::common::*;

use super::ProcessDataCollection::RecordItem;

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "VariableCollection")]
pub struct VariableCollection {
    #[yaserde(rename = "StdVariableRef")]
    pub stdvariableref: Vec<StandardVariableRef>,
    #[yaserde(rename = "Variable")]
    pub variable: Vec<Variable>,
}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "StdVariableRef")]
pub struct StandardVariableRef {
    #[yaserde(rename = "id", attribute)]
    pub id: String,
    #[yaserde(rename = "defaultValue", attribute)]
    pub defaultvalue: String,
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(
    root = "Variable",
    prefix = "xsi",
    namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
)]
pub struct Variable {
    #[yaserde(attribute)]
    pub id: String,

    #[yaserde(attribute)]
    pub index: String,

    #[yaserde(rename="accessRights",attribute)]
    pub accessrights: String,

    #[yaserde(attribute)]
    pub dynamic: bool,

    #[yaserde(attribute)]
    pub mandatory: bool,

    #[yaserde(attribute)]
    pub modifiesOtherVariables: bool,

    #[yaserde(attribute)]
    pub excludedFromDataStorage: bool,

    #[yaserde(attribute)]
    pub defaultValue: String, //TODO anySimpleType

    #[yaserde(child, rename="Datatype")]
    pub datatype: Datatype,

    #[yaserde(child)]
    pub datatyperef: DataTypeRef,

    #[yaserde(child)]
    pub Name: LookupValue,

    #[yaserde(child)]
    pub Description: LookupValue,

    #[yaserde(child)]
    pub singlevalues: Option<SingleValues>,

    



}

#[derive(Debug, Default, YaDeserialize)]
pub struct DataTypeRef {}

#[derive(Debug, Default, YaDeserialize)]
pub struct RecordItemInfo {
    #[yaserde(attribute)]
    pub subindex: u8,

    #[yaserde(attribute)]
    pub defaultvalue: String, //TODO

    #[yaserde(attribute)]
    pub modifiesothervariables: bool,

    #[yaserde(attribute)]
    pub excludedfromdatastorage: bool,
}

#[derive(Debug, Default, YaDeserialize)]
pub struct Datatype {
    #[yaserde(attribute,rename="type" prefix = "xsi")]
    pub datatype: String,

    #[yaserde(attribute, rename="bitLength")]
    pub bitlength: i32,

    #[yaserde(child, rename = "ValueRange")]
    pub valuerange: Option<ValueRange>,

    #[yaserde(rename = "SingleValue")]
    pub singlevalue : Vec<SingleValue>,
    #[yaserde(child,rename="RecordItem")]
    pub recorditem : Vec<RecordItem>
}
#[allow(dead_code)]
#[derive(Debug, Default, YaDeserialize)]
pub struct SingleValues {
    #[yaserde(child)]
    pub singlevalue: Vec<SingleValue>,
}

#[derive(Debug, Default, YaDeserialize)]
pub struct SingleValue {
    #[yaserde(attribute)]
    pub value: String,

    #[yaserde(child,rename="Name")]
    pub name: LookupValue,
}
