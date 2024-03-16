use yaserde::*;

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct ExternalTextCollection{
    #[yaserde(rename = "PrimaryLanguage")]
    pub language : Vec<Language>
}

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct Language {
    #[yaserde(rename = "lang", attribute)]
    pub language : String,
    #[yaserde(rename = "Text")]
    pub text: Vec<Text>
}

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct Text{
    #[yaserde(rename = "id",attribute)]
    pub id : String,
    #[yaserde(rename = "value", attribute)] 
    pub value : String
}