use yaserde::*;

use super::DocumentInfo::DocumentInfo;
use super::ExternalTextCollection::ExternalTextCollection;

use super::ProfileBody::ProfileBody;
use super::ProfileHeader::ProfileHeader;
use super::CommNetworkProfile::CommNetworkProfile;



#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase",  namespace="http://www.io-link.com/IODD/2009/11")]
pub struct IODevice {
    /*
    #[yaserde(rename = "xmlns:xsi", default)]
    pub xmlns_xsi: Option<String>,
    #[yaserde(rename = "xmlns", default)]
    pub xmlns: Option<String>,
    #[yaserde(rename = "xsi:schemaLocation", default)]
    pub xsi_schema_location: Option<String>,
     */
    #[yaserde(rename = "DocumentInfo")]
    pub document_info: Option<DocumentInfo>,
    #[yaserde(rename = "ProfileHeader")]
    pub profileheader : Option<ProfileHeader>,
    #[yaserde(rename = "ProfileBody")]
    pub profilebody :ProfileBody,
    #[yaserde(rename = "CommNetworkProfile")]
    pub commnetworkprofile : Option<CommNetworkProfile>,
    #[yaserde(rename = "ExternalTextCollection")]
    pub externaltextcollection :ExternalTextCollection
}
pub enum RoleSet {
    Observer,
    Maintenance,
    Specialist,
}

pub enum RoleMenu {
    Identification,
    Parameter,
    Observation,
    Diagnosis,
}