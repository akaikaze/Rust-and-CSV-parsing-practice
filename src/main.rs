extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn run() -> Result<(), Box<Error>>{
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.records( ){
        let record = result?;
        let city = &record[0];
        let state = &record[1];
        let pop: Option<u64> = record[2].parse().ok();
        let latitude: f64 = record[3].parse()?;
        let longitude: f64 = record[4].parse()?;
        println!(
            "city: {:?}, state: {:?}, \
             pop: {:?}, latitude: {:?}, longitude: {:?}",
            city, state, pop, latitude, longitude);
        }
    let headers = rdr.headers()?;
    println!("{:?}", headers);
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
