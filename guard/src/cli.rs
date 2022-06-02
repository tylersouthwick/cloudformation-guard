use clap::App;
use crate::command::Command;
use crate::commands;
use lazy_static::lazy_static;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

lazy_static! {
    pub(crate) static ref COMMANDS : Vec<Box<dyn Command + Sync>> = {
        vec![
            Box::new(commands::parse_tree::ParseTree::new()),
            Box::new(commands::test::Test::new()),
            Box::new(commands::validate::Validate::new()),
            Box::new(commands::rulegen::Rulegen::new()),
            Box::new(commands::migrate::Migrate::new()),
            Box::new(commands::generate_completions::GenerateCompletions::new()),
        ]
    };
}

pub fn build_app() -> App<'static> {
    let mut app =
        App::new(PKG_NAME)
            .version(VERSION)
            .about(r#"
  Guard is a general-purpose tool that provides a simple declarative syntax to define 
  policy-as-code as rules to validate against any structured hierarchical data (like JSON/YAML).
  Rules are composed of clauses expressed using Conjunctive Normal Form
  (fancy way of saying it is a logical AND of OR clauses). Guard has deep
  integration with CloudFormation templates for evaluation but is a general tool
  that equally works for any JSON- and YAML- data."#);

    for each in COMMANDS.iter() {
        app = app.subcommand(each.command());
    }

    app
}
