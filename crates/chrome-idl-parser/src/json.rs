use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeApi {
    pub namespace: String,
    pub description: String,
    #[serde(rename = "compiler_options")]
    pub compiler_options: CompilerOptions,
    pub types: Vec<Type>,
    pub functions: Vec<Function>,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    #[serde(rename = "implemented_in")]
    pub implemented_in: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "enum")]
    pub enum_field: Option<Vec<String>>,
    pub properties: Option<Properties>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub tab_id: Option<TabId>,
    pub is_on_toolbar: Option<IsOnToolbar>,
    pub window_id: Option<WindowId>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabId {
    #[serde(rename = "type")]
    pub type_field: String,
    pub optional: bool,
    pub minimum: i64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsOnToolbar {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowId {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub optional: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    #[serde(rename = "returns_async")]
    pub returns_async: Option<ReturnsAsync>,
    pub returns: Option<Returns>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Returns {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub properties: Option<Properties2>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub optional: Option<bool>,
    pub minimum: Option<i64>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties2 {
    pub color: Option<Color>,
    pub tab_id: TabId2,
    pub text: Option<Text>,
    pub popup: Option<Popup>,
    pub image_data: Option<ImageData>,
    pub path: Option<Path>,
    pub title: Option<Title>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub description: String,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabId2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub optional: bool,
    pub minimum: i64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    #[serde(rename = "type")]
    pub type_field: String,
    pub optional: bool,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Popup {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
    pub choices: Vec<Choice2>,
    pub optional: bool,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice2 {
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub additional_properties: Option<AdditionalProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperties {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub choices: Vec<Choice3>,
    pub optional: bool,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub additional_properties: Option<AdditionalProperties2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperties2 {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnsAsync {
    pub name: String,
    pub parameters: Vec<Parameter2>,
    pub optional: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter2 {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub parameters: Option<Vec<Parameter3>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter3 {
    pub name: String,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}
