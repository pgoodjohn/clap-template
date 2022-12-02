use clap::{Parser, Subcommand};
use log;
use plogger;

#[derive(Parser)]
#[clap(about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    Template(template::TemplateCommand),
}

mod template;

fn main() {
    let cli = Cli::parse();

    plogger::init(cli.debug);

    log::info!("Welcome to the Advent of Code 2022 Solutions Program");

    match cli.command {
        Some(Commands::Template(command)) => {
            template::command(&command);
        }
        None => {} // Handled by Clap
    }
}
