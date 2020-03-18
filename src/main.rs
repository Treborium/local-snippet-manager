use serde::{Deserialize, Serialize};
use serde_json::Value;
use structopt::StructOpt;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Write, BufReader};
use std::process::Command;

use clap::{load_yaml, App};

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
    let json: LSM = serde_json::from_reader::<BufReader<File>, LSM>(reader)?;
    let mut commands: Vec<String> = Vec::new();
    for (_, sub_map) in json {
        for (name, _command) in sub_map {
            commands.push(name.clone());
        }
    }

    let yml = load_yaml!("cli.yaml");
    let matches = App::from(yml).get_matches();
    
    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            // Now we have a reference to clone's matches
            let command = run_matches.value_of("command").unwrap();
            println!("Running '{}'...", command);
            let _handle = run_cmd(command);
        }
        ("ls", Some(_ls_matches)) => {
            println!("{:#?}", commands)
        }
        ("sel", Some(_sel_matches)) => {
            let mut fzf = Command::new("fzf")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()?;

            let stdin = fzf.stdin.as_mut().unwrap();
            stdin.write_all(commands.join("\n").as_bytes())
                .expect("Error listing available commands.");

            let out = fzf.wait_with_output().unwrap();
            let choice = String::from_utf8(out.stdout).unwrap();
            println!("{}",choice);
        }
        ("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(())
}
