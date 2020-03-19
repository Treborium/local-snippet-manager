use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::process::Command;

use clap::{load_yaml, App, ArgMatches};

mod display;

type LSM = HashMap<String, HashMap<String, String>>;

const DEFAULT_TERMINAL: &str = "/usr/bin/kitty";

fn execute(cmd: &str, hold: bool) -> Result<std::process::Child, std::io::Error> {
    let mut args: Vec<String> = Vec::new();

    if hold {
        args.push(String::from("--hold"));
    }

    Command::new(DEFAULT_TERMINAL).args(&args).arg(cmd).spawn()
}

fn run(commands: &HashMap<String, String>, matches: &ArgMatches) -> Result<(), std::io::Error> {
    let input = matches.value_of("command").unwrap();

    match commands.get(input) {
        Some(cmd) => {
            let _ = execute(cmd, matches.is_present("hold"));
            Ok(())
        }
        None => return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("<{}> not found! Please make sure there are no typos.", input)
        )),
    }
}

fn sel(commands: &HashMap<String, String>, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
   let choice = make_choice(&commands);

    if !choice.is_empty() {
        let _ = execute(
            commands.get(&choice[..choice.len() - 1]).unwrap(),
            matches.is_present("hold"),
        );
    }

    Ok(())
}

fn print(commands: &HashMap<String, String>) {
    let choice = make_choice(&commands);
    if !choice.is_empty() {
        println!("{}", commands.get(&choice[..choice.len() - 1]).unwrap());
            
    }
}

fn make_choice(commands: &HashMap<String, String>) -> String {
    let mut fzf = Command::new("fzf")
    // .args(&["--preview=", "hello world"])
    // .arg("--tac")
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .spawn()
    .expect("Error: fzf failed to start. Please make sure fzf is properly installed.");

    let options: Vec<String> = commands
        .iter()
        // .map(|(key, value)| display::format_line(key, value, 20, 60))
        .map(|(key, _)| key.clone())
        .collect();

    let stdin = fzf.stdin.as_mut().unwrap();
    stdin.write_all(options.join("\n").as_bytes())
        .expect("Error while listing available commands.");

    let out = fzf.wait_with_output().unwrap();
    String::from_utf8(out.stdout).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open("lsm.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `LSM`.
    let json: LSM = serde_json::from_reader::<BufReader<File>, LSM>(reader)?;
    let mut commands: HashMap<String, String> = HashMap::new();

    for (_, sub_map) in json {
        for (name, command) in sub_map {
            commands.insert(name, command);
        }
    }

    let yml = load_yaml!("cli.yaml");
    let matches = App::from(yml).get_matches();

    match matches.subcommand() {
        ("run", Some(run_matches)) => run(&commands, &run_matches)?,
        ("ls", Some(_ls_matches)) => println!("{:#?}", commands),
        ("sel", Some(sel_matches)) => sel(&commands, &sel_matches)?,
        ("print", Some(_print_matches)) => print(&commands),
        ("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(())
}
