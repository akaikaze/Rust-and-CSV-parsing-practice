extern crate csv;
// extern crate serde;

// #[macro_use]
// extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
// use std::fs::File;
use std::process;
use std::io;

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
// struct Record {
//     city: String,
//     state: String,
//     latitude: f64,
//     longitude: f64,
//     #[serde(deserialize_with = "csv::invalid_option")]
//     population: Option<u64>,
// }

fn run() -> Result<u64, Box<Error>> {

    let read_file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(read_file_path)?;
    
    let mut count = 0;

    for result in rdr.byte_records() {
        let record = result?;
        if &record[0] == b"us" && &record[3] == b"MA" {
            count += 1;
        }
    }

    Ok(count)
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
