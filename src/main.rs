pub mod cli;
pub mod recipe;
pub mod notion;

use scraper::{Html, Selector};
use serde_json::Value;
use clap::Parser;
use dotenv::dotenv;
use std::env;
use reqwest::header::HeaderMap;
use crate::notion::{Color, MultiSelectProperty, NumberProperty, RichTextAnnotations, RichTextPart, RichTextProperty, RichTextText, SelectProperty, SelectPropertyInner, TitleProperty, UrlProperty};

struct RecipeReaderEnvironmentVariables {
    notion_api_token: String,
    recipes_db_id: String
}

fn get_env_vars() -> RecipeReaderEnvironmentVariables {
    RecipeReaderEnvironmentVariables {
        notion_api_token: env::var("NOTION_API_TOKEN").unwrap(),
        recipes_db_id: env::var("RECIPES_DB_ID").unwrap()
    }
}

fn main() {
    dotenv().expect("Unable to load .env file");
    let env = get_env_vars();

    let args = cli::Cli::parse();
    let recipe_html = {
        let html_text = reqwest::blocking::get(&args.url).unwrap().text().unwrap();
        Html::parse_document(&html_text)
    };

    let recipe = get_recipe(recipe_html);
    println!("{}", recipe);

    let notion_client = {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("Authorization",
                               format!("Bearer {}", env.notion_api_token).parse().unwrap());
        default_headers.insert("Notion-Version", "2022-02-22".parse().unwrap());
        reqwest::blocking::Client::builder().default_headers(default_headers).build().unwrap()
    };

    let notion_url = "https://api.notion.com/v1";

    let body = notion::CreatePageBody {
        parent: notion::Parent {
            database_id: env.recipes_db_id
        },
        properties: notion::RecipePageProperties {
            rating: NumberProperty {
                number: 0
            },
            servings: RichTextProperty {
                rich_text: vec![]
            },
            cuisine: SelectProperty {
                select: SelectPropertyInner {
                    name: "Tmp".to_string(),
                    color: Option::from(Color::Default)
                }
            },
            tags: MultiSelectProperty {
                multi_select: vec![]
            },
            difficulty: SelectProperty {
                select: SelectPropertyInner {
                    name: "Tmp".to_string(),
                    color: Option::from(Color::Default)
                }
            },
            link: UrlProperty {
                url: args.url
            },
            course: MultiSelectProperty {
                multi_select: vec![]
            },
            name: TitleProperty {
                title: vec![RichTextPart {
                    part_type: "text".to_string(),
                    text: RichTextText { content: recipe.name },
                    annotations: RichTextAnnotations {
                        italic: Option::from(true),
                        bold: Option::from(true),
                        color: Option::from(Color::Pink)
                    }
                }]
            }
        }
    };

    let res = notion_client.post(format!("{}/pages", notion_url)).json(&body).send().unwrap();

    if res.status().is_success() {
        println!("Recipe added to notion!");
    } else {
        let notion_error: notion::NotionError = res.json().unwrap();
        println!("{}", notion_error);
    }
}

fn get_recipe(html: Html) -> recipe::Recipe {
    let script_selector = Selector::parse("script").unwrap();
    for script_element in html.select(&script_selector) {
        // check their type attribute
        let script_element_type_opt = script_element.value().attr("type");

        let script_element_type = match script_element_type_opt {
            Some (el_type) => el_type,
            None => continue
        };

        // we only care about the application/ld+json type
        if script_element_type != "application/ld+json" { continue; }

        // Parse the json
        let json_string = script_element.inner_html();

        // Determine if the json is an object or array
        let first_char = json_string.chars().next().unwrap();

        return match first_char {
            '[' => parse_array_schema(&json_string),
            '{' => parse_object_schema(&json_string),
            _ => panic!("wut")
        };
    }

    panic!("Recipe not found in HTML")
}

fn parse_array_schema(json_string: &String) -> recipe::Recipe {
    let json: Vec<Value> = serde_json::from_str(&*json_string).unwrap();

    // look at all elements in the array
    for obj in json {
        // we only care about "@type": "Recipe",
        if obj["@type"] == "Recipe" {
            return serde_json::from_value(obj).unwrap();
        }
    }

    panic!("Recipe not found in array schema")
}

fn parse_object_schema(json_string: &String) -> recipe::Recipe {
    let json: recipe::Schema = serde_json::from_str(&*json_string).unwrap();

    // look at all elements in the array
    for obj in json.graph {
        // we only care about "@type": "Recipe",
        if obj["@type"] == "Recipe" {
            return serde_json::from_value(obj).unwrap();
        }
    }

    panic!("Recipe not found in object schema")
}