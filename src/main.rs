use clap::Parser;
use geoutils::Location as GeoLocation;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Clone)]
pub enum Commands {
    #[clap(name = "calc")]
    Calc {
        #[arg(
            long,
            required = true,
            name = "origin",
            help = "Origin location, e.g. city name or postal code. Accuracy depends on the short or full name of the location including postal code"
        )]
        origin: String,
        #[arg(
            long,
            required = true,
            name = "destination",
            help = "Destination location, e.g. city name or postal code. Accuracy depends on the short or full name of the location including postal code"
        )]
        destination: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseLocation {
    pub lat: String,
    pub lon: String,
    pub display_name: String,
}

pub fn get_location(input: &str) -> Result<Location, Box<dyn Error>> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=5&addressdetails=1",
        input
    );
    let client = reqwest::blocking::Client::new();
    let data = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .send()?
        .json::<Vec<ResponseLocation>>()?;
    if let Some(response_location) = data.first() {
        return Ok(Location {
            lat: response_location.lat.parse::<f64>()?,
            lon: response_location.lon.parse::<f64>()?,
            display_name: response_location.display_name.clone(),
        });
    }
    Err("failed to parse latitude and longitude. argument: {input}".into())
}

pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let origin = GeoLocation::new(lat1, lon1);
    let destination = GeoLocation::new(lat2, lon2);
    origin.haversine_distance_to(&destination).meters() / 1000.0
}

fn main() {
    let str = "- Welcome to Distance Calculator -";

    let cli = Cli::parse();
    println!("{str}");
    if let Some(command) = &cli.command {
        match command {
            Commands::Calc {
                origin,
                destination,
            } => match get_location(origin) {
                Ok(origin) => match get_location(destination) {
                    Ok(destination) => {
                        let distance = calculate_distance(
                            origin.lat,
                            origin.lon,
                            destination.lat,
                            destination.lon,
                        );
                        println!(
                            "Origin: {} -> Destination: {} \nDistance: {distance:.2} km",
                            origin.display_name, destination.display_name
                        );
                    }
                    Err(e) => eprintln!("Error Distance: {}", e),
                },
                Err(e) => eprintln!("Error Origin: {}", e),
            },
        }
    } else {
        println!("Use 'calc --help' for more information");
    }
}
