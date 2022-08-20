use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePageBody {
    pub parent: Parent,
    pub properties: RecipePageProperties
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parent {
    pub database_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipePageProperties {
    #[serde(rename = "Rating")]
    pub rating: NumberProperty,

    #[serde(rename = "Servings")]
    pub servings: RichTextProperty,

    #[serde(rename = "Cuisine")]
    pub cuisine: SelectProperty,

    #[serde(rename = "Tags")]
    pub tags: MultiSelectProperty,

    #[serde(rename = "Difficulty")]
    pub difficulty: SelectProperty,

    #[serde(rename = "Link")]
    pub link: UrlProperty,

    #[serde(rename = "Course")]
    pub course: MultiSelectProperty,

    #[serde(rename = "Name")]
    pub name: TitleProperty
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NumberProperty {
    pub number: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RichTextProperty {
    pub rich_text: Vec<RichTextPart>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RichTextPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub text: RichTextText,
    pub annotations: RichTextAnnotations
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RichTextText {
    pub content: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RichTextAnnotations {
    pub italic: Option<bool>,
    pub bold: Option<bool>,
    pub color: Option<Color>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectProperty {
    pub select: SelectPropertyInner
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectPropertyInner {
    pub name: String,
    pub color: Option<Color>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Color {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "brown")]
    Brown,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "pink")]
    Pink
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSelectProperty {
    pub multi_select: Vec<SelectPropertyInner>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlProperty {
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TitleProperty {
    pub title: Vec<RichTextPart>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotionError {
    pub object: String,
    pub status: u16,
    pub code: String,
    pub message: String
}

impl fmt::Display for NotionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NotionError {{\n\tobject: {}\n\tstatus: {}\n\tcode: {}\n\tmessage: {}\n}}",
               self.object, self.status, self.code, self.message).unwrap();
        Ok(())
    }
}