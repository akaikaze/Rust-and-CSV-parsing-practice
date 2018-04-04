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
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(("Davidsons Landing", "AK", None::<u64>, 65.2419444, -165.2716667))?;
    wtr.serialize(("Kenai", "AK", Some(7610), 60.5544444, -151.2583333))?;
    wtr.serialize(("Oakman", "AL", None::<u64>, 33.7133333, -87.3886111))?;
    
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
