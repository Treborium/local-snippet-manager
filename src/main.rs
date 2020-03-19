use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::process::Command;

use clap::{load_yaml, App, ArgMatches};

type LSM = HashMap<String, HashMap<String, String>>;

const DEFAULT_TERMINAL: &str = "/usr/bin/kitty";

fn execute(cmd: &str, hold: bool) -> Result<std::process::Child, std::io::Error> {
    let mut shell_args: Vec<String> = Vec::new();
    let sub_commands: Vec<&str> = cmd.split(' ').collect();

    if hold {
        shell_args.push(String::from("--hold"));
    }

    Command::new(DEFAULT_TERMINAL)
        .args(&shell_args)  // This are arguments for the terminal emulator that is spawned
        .arg(sub_commands[0])  // This is the actual command
        .args(&sub_commands[1..]) // This are sub commands, flags, arguments, etc.
        .spawn()
}

fn run(commands: &HashMap<String, String>, matches: &ArgMatches) {
    match matches.value_of("command") {
        Some(input) => {
            match commands.get(input) {
                Some(cmd) => {
                    let _ = execute(cmd, matches.is_present("hold"));
                }
                None => {
                    eprintln!("'{}' not found! Please make sure there are no typos.", input);
                    std::process::exit(-1)
                }
            }
        }
        None => {
            let choice = make_choice(&commands);

            if !choice.is_empty() {
                let _ = execute(
                    commands.get(&choice[..choice.len() - 1]).unwrap(),
                    matches.is_present("hold"),
                );
            }
        }
    }
}

fn print(commands: &HashMap<String, String>) {
    let choice = make_choice(&commands);
    if !choice.is_empty() {
        println!("{}", commands.get(&choice[..choice.len() - 1]).unwrap());
            
    }
}

fn make_choice(commands: &HashMap<String, String>) -> String {
    let mut fzf = Command::new("fzf")
    // .arg(format!("--preview=echo \"{}\"", "Hello World!"))
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

fn main() {
    // Open the file in read-only mode with buffer.
    let file = File::open("lsm.json")
        .expect("Could not open lsm file.");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `LSM`.
    let json: LSM = serde_json::from_reader::<BufReader<File>, LSM>(reader)
        .expect("Could not parse lsm file.");
    let mut commands: HashMap<String, String> = HashMap::new();

    for (_, sub_map) in json {
        for (name, command) in sub_map {
            commands.insert(name, command);
        }
    }

    let yml = load_yaml!("cli.yaml");
    let matches = App::from(yml).get_matches();

    match matches.subcommand() {
        ("run", Some(run_matches)) => run(&commands, &run_matches),
        ("ls", Some(_ls_matches)) => println!("{:#?}", commands),
        ("print", Some(_print_matches)) => print(&commands),
        ("", None) => println!("No sub command was used"), // If no subcommand was usd it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
