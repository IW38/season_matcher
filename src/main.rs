use clap::{Arg, Command};
use serde::Deserialize;
use std::fs;
use std::fmt;

// Define the Country struct
#[derive(Deserialize)]
struct Country {
    name: String,
    population: u64,
    area: f64, // in square kilometers
}

// Implement the Display trait for the Country struct
impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Define the Countries struct
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Countries {
    winter: Vec<Country>,
    spring: Vec<Country>,
    summer: Vec<Country>,
    autumn: Vec<Country>,
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
            let country_names: Vec<String> = countries_list.iter().map(|c| c.to_string()).collect();
            println!("Countries to visit in {}: {}", season, country_names.join(", "));
        }
        None => {
            println!("Invalid season. Please choose from Winter, Spring, Summer, or Autumn.");
        }
    }
}
