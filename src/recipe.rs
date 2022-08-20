use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    #[serde(rename = "recipeIngredient")]
    pub recipe_ingredient: Strings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Strings(pub Vec<String>);

impl fmt::Display for Strings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").unwrap();
        for string in self.0.iter() {
            write!(f, "{}, ", string).unwrap();
        };
        write!(f, "]").unwrap();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LdJson {
    #[serde(rename = "@type")]
    at_type: String
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Recipe {{\n\tname: {}\n\tdescription: {}\n\trecipe_ingredient: {}\n}}",
               self.name, self.description, self.recipe_ingredient)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "@graph")]
    pub graph: Vec<Value>
}