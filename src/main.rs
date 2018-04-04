extern crate csv;

use std::io;
use std::process;

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records(){
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading CSV from <stdio>: {}", err);
                process::exit(1);
            },
        }
    }
}
