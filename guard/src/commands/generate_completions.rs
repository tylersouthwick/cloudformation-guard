use clap::{App, Arg, ArgMatches};

use crate::rules::Result;
use crate::command::Command;
use crate::cli::build_app;

use clap_complete::{generate as generate_completions, Shell};

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct GenerateCompletions {}

impl GenerateCompletions {
    pub(crate) fn new() -> Self {
        GenerateCompletions{}
    }
}

impl Command for GenerateCompletions {
    fn name(&self) -> &'static str {
        "generate-completions"
    }


    fn command(&self) -> App<'static> {
        App::new(self.name())
            .about("Generate shell completions")
            .arg(Arg::with_name("shell")
                 .long("shell")
                 .takes_value(true)
                 .required(true)
                 .possible_values(Shell::possible_values())
                 .help("shell to generate completions for"))
    }

    fn execute(&self, app: &ArgMatches) -> Result<i32> {
        match app.value_of_t::<Shell>("shell") {
            Ok(shell) =>  {
                println!("generate completions {:?}", shell);
                let mut app = build_app();
                let name = app.get_name().to_string();
                generate_completions(shell, &mut app, name, &mut std::io::stdout());
                Ok(0)
            },
            Err(err) => {
                println!("err: {:?}", err);
                Ok(1)
            }
        }
    }
}


