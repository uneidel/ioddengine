use yaserde::*;


use crate::ioddmodel11::IODevice::RoleMenu;
use crate::ioddmodel11::IODevice::RoleSet;
use crate::ioddmodel11::ProfileBody::common::*;
use crate::ioddmodel11::ProfileBody::MenuCollection::*;
use crate::ioddmodel11::ProfileBody::ProcessDataCollection::*;
use crate::ioddmodel11::ProfileBody::VariableCollection::*;

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "DeviceFunction")]
pub struct DeviceFunction {
    #[yaserde(rename = "Features")]
    pub features: Option<Features>,
    #[yaserde(rename = "DatatypeCollection")]
    pub datatypecollection: Option<DataTypeCollection>, //TODO
    #[yaserde(rename = "VariableCollection")]
    pub variablecollection: VariableCollection,
    #[yaserde(rename = "ProcessDataCollection")]
    pub processdatacollection: ProcessDataCollection,
    #[yaserde(rename = "ErrorTypeCollection")]
    pub errortypecollection: Option<ErrorTypeCollection>,
    #[yaserde(rename = "EventCollection")]
    pub eventcollection: Option<EventCollection>,
    #[yaserde(rename = "UserInterface")]
    pub userinterface: UserInterface,
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "ErrorTypeCollection")]
pub struct ErrorTypeCollection {}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "UserInterface")]
pub struct UserInterface {
    #[yaserde(child, rename = "MenuCollection")]
    pub menucollection: MenuCollection,
    #[yaserde(child, rename = "ObserverRoleMenuSet")]
    pub oberserverrolemenuset: RoleMenuSet,
    #[yaserde(child, rename = "MaintenanceRoleMenuSet")]
    pub maintenancerolemenuset: RoleMenuSet,
    #[yaserde(child, rename = "SpecialistRoleMenuSet")]
    pub specialistrolemenuset: RoleMenuSet,
}
impl UserInterface {
    // Method to return the RoleMenuSet based on the provided name
    pub fn get_role_menu_set_by_name(&self, name: &RoleSet) -> Option<&RoleMenuSet> {
        match name {
            RoleSet::Observer => Some(&self.oberserverrolemenuset),
            RoleSet::Maintenance => Some(&self.maintenancerolemenuset),
            RoleSet::Specialist => Some(&self.specialistrolemenuset),            
        }
    }
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "RoleMenuSet")]
pub struct RoleMenuSet {
    #[yaserde(rename = "IdentificationMenu")]
    pub identificationmenu: MenuSet,
    #[yaserde(rename = "ParameterMenu")]
    pub parametermenu: MenuSet,
    #[yaserde(rename = "ObservationMenu")]
    pub observermenu: MenuSet,
    #[yaserde(rename = "DiagnosisMenu")]
    pub diagnosismenu: MenuSet,
}
impl RoleMenuSet {
    // Method to return the MenuSet based on the provided name
    pub fn get_menu_by_name(&self, name: &RoleMenu) -> Option<&MenuSet> {
        match name {
            RoleMenu::Identification => Some(&self.identificationmenu),
            RoleMenu::Parameter => Some(&self.parametermenu),
            RoleMenu::Observation => Some(&self.observermenu),
            RoleMenu::Diagnosis => Some(&self.diagnosismenu),
            
        }
    }
}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "MenuSet")]
pub struct MenuSet {
    #[yaserde(rename = "menuId", attribute)]
    pub menuid: String,
}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "EventCollection")]
pub struct EventCollection {
    #[yaserde(child)]
    pub standardeventref: Vec<StandardEventRef>,
    #[yaserde(child, rename = "Event")]
    pub event: Vec<Event>,
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "Event")]
pub struct Event {
    #[yaserde(attribute)]
    pub code: u16,

    #[yaserde(attribute, rename = "type")]
    pub eventtype: String,

    #[yaserde(attribute, rename = "mode")]
    pub mode: String,

    #[yaserde(child, rename = "Name")]
    pub name: LookupValue,

    #[yaserde(child, rename = "Description")]
    pub description: LookupValue,
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "StandardEventRef")]
pub struct StandardEventRef {
    #[yaserde(attribute)]
    pub code: u8,
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "DatatypeCollection")]
pub struct DataTypeCollection {
    #[yaserde(rename = "Datatype")]
    pub datatype: Vec<DataType>,
}

#[derive(YaDeserialize, Default, Debug)]
pub struct DataType {
    #[yaserde(rename = "id", attribute)]
    pub id: String,

    #[yaserde(rename = "type", attribute)]
    pub datatype: String,

    #[yaserde(rename = "bitLength", attribute)]
    pub bitlength: u8,

    #[yaserde(rename = "Name")]
    pub name: LookupValue,

    #[yaserde(rename = "ValueRange")]
    pub valuerange: ValueRange,
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "Features")]
pub struct Features {
    #[yaserde(rename = "blockParameter", attribute)]
    pub blockparameter: bool,

    #[yaserde(rename = "dataStorage", attribute)]
    pub datastorage: bool,

    #[yaserde(rename = "profileCharacteristic", attribute)]
    pub profilecharacteristic: String,

    #[yaserde(rename = "SupportedAccessLocks")]
    pub supportedaccesslocks: SupportedAccessLocks,
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "SupportedAccessLocks")]
pub struct SupportedAccessLocks {
    #[yaserde(rename = "parameter", attribute)]
    pub parameter: bool,

    #[yaserde(rename = "dataStorage", attribute)]
    pub datastorage: bool,

    #[yaserde(rename = "localParameterization", attribute)]
    pub localparameterization: bool,

    #[yaserde(rename = "localUserInterface", attribute)]
    pub localuserinterface: bool,
}

/*************** */
