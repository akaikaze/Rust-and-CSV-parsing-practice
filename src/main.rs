extern crate csv;
extern crate serde;

// #[macro_use]
// extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::io;

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Record {
//     latitude: f64,
//     longitude: f64,
//     #[serde(deserialize_with = "csv::invalid_option")]
//     population: Option<u64>,
//     city: String,
//     state: String,
// }

fn run() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(io::stdout());
        
    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    wtr.flush()?;
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        Some(file_path) => Ok(file_path),
        None => Err(From::from("expected 1 argument, but got none ")),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
