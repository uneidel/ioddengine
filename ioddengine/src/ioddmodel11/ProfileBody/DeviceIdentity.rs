use yaserde::*;

use crate::ioddmodel11::ProfileBody::common::*;

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename = "DeviceIdentity")]
pub  struct DeviceIdentity {
    #[yaserde(rename = "vendorId", attribute)]
    pub vendorid : String,
    #[yaserde(rename = "vendorName", attribute)]
    pub vendorname : String,
    #[yaserde(rename = "deviceId", attribute)]
    pub deviceid : String,
    #[yaserde(rename = "additionalDeviceIds", attribute)]
    pub additionaldeviceids : String,

    #[yaserde(rename = "VendorText")]
    pub vendortext : LookupValue,

    #[yaserde(rename = "VendorUrl")]
    pub vendorurl : LookupValue,

    #[yaserde(rename = "VendorLogo")]
    pub vendorlogo : VendorLogo,

    #[yaserde(rename = "DeviceName")]
    pub devicename : LookupValue,
    
    #[yaserde(rename = "DeviceFamily")]
    pub devicefamily : LookupValue,
    #[yaserde(rename = "DeviceVariantCollection")]
    pub devicevariantcollection : DeviceVariantCollection
    
}

#[derive(YaDeserialize,Default, Debug)]
pub struct DeviceVariantCollection{
    #[yaserde(rename="DeviceVariant")]
    pub devicevariant : Vec<DeviceVariant> 
}

#[derive(YaDeserialize,Default, Debug)]
pub struct DeviceVariant{
    #[yaserde(attribute, rename="productId")]
    pub productid : String, 
    #[yaserde(attribute, rename="deviceSymbol")]
    pub devicesymbol : String,
    #[yaserde(attribute, rename="deviceIcon")]
    pub deviceicon : String,
    #[yaserde(rename="ProductName")] //TODO should be called name
    pub name : LookupValue,
    #[yaserde(rename="ProductText")] //TODO speific should be called description
    pub description : LookupValue
}


#[derive(YaDeserialize,Default, Debug)]
pub struct VendorLogo{
    #[yaserde(attribute, rename="name")]
    pub name : String 
}



