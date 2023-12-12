use crate::command::Command;
use clap::{Arg, ArgMatches, ValueHint};
use crate::utils::reader::Reader;
use crate::utils::writer::Writer;
use crate::rules::Result;

mod server;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct LspServer {}

const LSP_SERVER : &'static str = "lsp-server";
const LISTEN: (&str, char) = ("listen", 'l');

impl Command for LspServer {
    fn name(&self) -> &'static str {
        LSP_SERVER
    }

    fn command(&self) -> clap::Command {
        clap::Command::new(LSP_SERVER)
            .about("Run a language server for integration with IDEs")
            .arg(Arg::new(LISTEN.0)
                .long(LISTEN.0)
                .short(LISTEN.1)
                .action(clap::ArgAction::SetTrue)
                .help("port for the lsp")
                .required(false))
            .arg_required_else_help(false)
    }

    fn execute(&self, app: &ArgMatches, _: &mut Writer, _: &mut Reader) -> Result<i32> {
        let listen = app.get_flag(LISTEN.0);
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap()
            .block_on(server::run(listen));
        //let template_contents = fs::read_to_string(file)?;

        //let result = parse_template_and_call_gen(&template_contents, writer);
        //print_rules(result, writer)?;

        Ok(0_i32)
    }
}

impl LspServer {
    pub fn new() -> Self {
        LspServer {}
    }
}
