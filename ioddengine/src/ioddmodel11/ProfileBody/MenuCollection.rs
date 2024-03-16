use yaserde::*;

use super::common::LookupValue;

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "MenuCollection")]
pub struct MenuCollection {
    #[yaserde(rename = "Menu")]
    pub menu: Vec<Menu>,
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "Menu")]
pub struct Menu {
    #[yaserde(rename = "id", attribute)]
    pub id: String,
    #[yaserde(rename = "Name")]
    pub name: LookupValue,
    #[yaserde(rename = "VariableRef")]
    pub variableref: Vec<VariableRef>,
    #[yaserde(rename = "RecordItemRef")]
    pub recorditemref: Vec<RecordItemRef>,
    #[yaserde(rename = "MenuRef")]
    pub menuref: Vec<MenuRef>,
}
impl Menu {
    pub fn getmenu_by_condition(
        menu: &Menu,
        variableid: &str,
        value: u8,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!("{:?}", menu);
        for mr in &menu.menuref {
            println!("xx<<<<xx");
            if let Some(c) = &mr.condition {
                println!("xxxx");
                if c.variableId == variableid && c.value == value {
                    return Ok(mr.menuid.clone());
                }
            }
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Menu not found",
        )))
    }
}

#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "MenuRef")]
pub struct MenuRef {
    #[yaserde(rename = "menuId", attribute)]
    pub menuid: String,
    #[yaserde(rename = "Condition")]
    pub condition: Option<Condition>,
}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "Condition")]
pub struct Condition {
    #[yaserde(rename = "variableId", attribute)]
    pub variableId: String,
    #[yaserde(rename = "value", attribute)]
    pub value: u8,
}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "RecordItemRef")]
pub struct RecordItemRef {
    #[yaserde(rename = "variableId", attribute)]
    pub variableId: String,
    #[yaserde(rename = "subindex", attribute)]
    pub subindex: u8,
    #[yaserde(rename = "displayFormat", attribute)]
    pub displayformat: Option<String>,
    #[yaserde(rename = "gradient", attribute)]
    pub gradient: Option<f32>,
    #[yaserde(rename = "offset", attribute)]
    pub offset: Option<u8>,
    #[yaserde(rename = "unitCode", attribute)]
    pub unitcode: Option<u32>,
}
impl Clone for RecordItemRef {
    fn clone(&self) -> Self {
        RecordItemRef {
            variableId: self.variableId.clone(),
            subindex: self.subindex,
            displayformat: self.displayformat.clone(),
            gradient: self.gradient,
            offset: self.offset,
            unitcode: self.unitcode,
        }
    }
}
#[derive(YaDeserialize, Default, Debug)]
#[yaserde(rename = "VariableRef")]
pub struct VariableRef {
    #[yaserde(rename = "variableId", attribute)]
    pub variableId: String,
    #[yaserde(rename = "accessRightRestriction", attribute)]
    pub accessrights: String,
}
