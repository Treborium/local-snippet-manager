use serde::{Deserialize, Serialize};
use serde_json::Value;
use structopt::StructOpt;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Write, BufReader};
use std::process::Command;

type LSM = HashMap<String, HashMap<String, String>>;

const DEFAULT_TERMINAL: &str = "/usr/bin/kitty";

fn run_cmd(cmd: &str) -> Result<std::process::Child, std::io::Error> {
    Command::new(DEFAULT_TERMINAL)
        // .arg("--hold")  // Keep the terminal emulator alive after command finished
        .arg(cmd)
        .spawn()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open("lsm.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let json = serde_json::from_reader::<BufReader<File>, LSM>(reader)?;
    println!("{:#?}", json);
    let _handle = run_cmd("ls");

    Ok(())
}
