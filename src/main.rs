extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}

fn run() -> Result<(), Box<Error>> {
    let read_file_path = get_first_arg()?;
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut rdr = csv::Reader::from_path(read_file_path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        wtr.serialize(record)?;
    }


    // wtr.serialize(Record {
    //     city: "Davidsons Landing",
    //     state: "AK",
    //     population: None,
    //     latitude: 65.2419444,
    //     longitude: -165.2716667,        
    // })?;
    // wtr.serialize(Record {
    //     city: "Kenai",
    //     state: "AK",
    //     population: Some(7610),
    //     latitude: 60.5544444,
    //     longitude: -151.2583333,
    // })?;
    // wtr.serialize(Record {
    //     city: "Oakman",
    //     state: "AL",
    //     population: None,
    //     latitude: 33.7133333,
    //     longitude: -87.3886111,
    // })?;

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
