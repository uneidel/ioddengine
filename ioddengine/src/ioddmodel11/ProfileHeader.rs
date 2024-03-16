use yaserde::*;



#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase",  namespace="http://www.io-link.com/IODD/2009/11")]
pub struct ProfileHeader{
    #[yaserde(rename = "ProfileIdentification")]
    pub profile_identification: String, 
    #[yaserde(rename = "ProfileRevision")]
    pub profile_revision: String,
    #[yaserde(rename = "ProfileName")]
    pub profile_name: String, 
    #[yaserde(rename = "ProfileSource")]
    pub profile_source : String, 
    #[yaserde(rename = "ProfileClassID")]
    pub profile_classid: String, // Reconsider to introuce NNTOKEN
    #[yaserde(rename = "ISO15745Reference")]
    pub iso15745_reference: ISO15745Reference,
}   
#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase",  namespace="http://www.io-link.com/IODD/2009/11")]
pub struct ISO15745Reference{
    #[yaserde(rename = "ISO15745Part")]
    pub iso15745_part : u8,
    #[yaserde(rename = "ISO15745Edition")]
    pub iso15745_edition : i8,
    #[yaserde(rename = "ProfileTechnology")]
    pub profile_technology: String 
}
