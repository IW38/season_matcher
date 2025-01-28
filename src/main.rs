use clap::{Arg, Command};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Countries {
    winter: Vec<String>,
    spring: Vec<String>,
    summer: Vec<String>,
    autumn: Vec<String>,
}

fn main() {
    let matches = Command::new("Seasonal Countries")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Pick countries based on the season")
        .arg(
            Arg::new("season")
                .short('s')
                .long("season")
                .value_name("SEASON")
                .help("Specifies the season")
                .required(true),
        )
        .get_matches();

    let season = matches.get_one::<String>("season").unwrap().as_str();

    // Read the JSON file
    let data = fs::read_to_string("countries.json").expect("Unable to read file");
    let countries: Countries = serde_json::from_str(&data).expect("JSON was not well-formatted");

    let season_countries = match season {
        "Winter" => Some(&countries.winter),
        "Spring" => Some(&countries.spring),
        "Summer" => Some(&countries.summer),
        "Autumn" => Some(&countries.autumn),
        _ => None,
    };

    match season_countries {
        Some(countries_list) => {
            println!("Countries to visit in {}: {:?}", season, countries_list);
        }
        None => {
            println!("Invalid season. Please choose from Winter, Spring, Summer, or Autumn.");
        }
    }
}
