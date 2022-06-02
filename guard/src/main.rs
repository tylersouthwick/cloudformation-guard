use std::collections::HashMap;

mod rules;
mod commands;
mod command;
mod migrate;
mod cli;

use rules::errors::Error;
use std::process::exit;
use cli::{build_app, COMMANDS};

fn main() -> Result<(), Error>{
    let mut app = build_app();

    let mappings = COMMANDS.iter()
        .map(|s| (s.name(), s)).fold(
        HashMap::with_capacity(COMMANDS.len()),
        |mut map, entry| {
            map.insert(entry.0, entry.1.as_ref());
            map
        }
    );

    let matches = app.clone().get_matches();
    match matches.subcommand() {
        Some((name, value)) => {
            if let Some(command) = mappings.get(name) {
                match (*command).execute(value) {
                    Err(e) => {
                        println!("Error occurred {}", e);
                        exit(-1);
                    },
                    Ok(code) => {
                        exit(code)
                    }
                }
            }
            else {
                app.write_help(&mut std::io::stdout());
            }
        },
        None => {
            app.write_long_help(&mut std::io::stdout());
        }
    }
    Ok(())
}

