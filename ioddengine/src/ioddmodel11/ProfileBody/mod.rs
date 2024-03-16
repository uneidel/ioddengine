
use yaserde::*;



pub mod DeviceIdentity;

pub mod DeviceFunction;
pub mod VariableCollection;
pub mod ProcessDataCollection;
pub mod common;
pub mod MenuCollection;


#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct ProfileBody{
    #[yaserde(rename = "DeviceIdentity")]
    pub device_identity : Option<DeviceIdentity::DeviceIdentity>,
    #[yaserde(rename = "DeviceFunction")]
    pub device_function: DeviceFunction::DeviceFunction,
}