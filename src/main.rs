use serde::{Deserialize, Serialize};
use serde_json::Value;
use structopt::StructOpt;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write, BufReader};
use std::process::{Command, Output};

type LSM = HashMap<String, HashMap<String, String>>;

fn run_cmd(cmd: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;

    std::io::stdout().write_all(&output.stdout)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open("lsm.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let json = serde_json::from_reader::<BufReader<File>, LSM>(reader)?;
    println!("{:#?}", json);
    run_cmd(&json["Navigation"]["List Content"])
}
