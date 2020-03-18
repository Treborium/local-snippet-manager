use structopt::StructOpt;
use serde_json::{Value};
use serde::{Serialize, Deserialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

type LSM = HashMap<String, HashMap<String, String>>;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open("lsm.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let json = serde_json::from_reader::<BufReader<File>, LSM> (reader)?;
    println!("{:#?}", json);

    Ok(())
}
