use yaserde::*;

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct DocumentInfo {
    #[yaserde(rename = "version", attribute)]
    pub version: String,

    #[yaserde(rename = "releaseDate", attribute)]
    pub release_date: String,

    #[yaserde(rename = "copyright", attribute)]
    pub copyright: String,
}
