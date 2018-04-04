extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>>{
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records( ){
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                println!("{:?}", record)
            }
        }
    }
    Ok(())
}

