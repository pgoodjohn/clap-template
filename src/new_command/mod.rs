use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct NewCommandCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<NewCommandCommands>,
}

#[derive(Subcommand)]
pub enum NewCommandCommands {
    One {},
    Two {},
}

pub fn command(command: &NewCommandCommand) -> Result<String, Box<dyn std::error::Error>> {
    match command.command {
        Some(NewCommandCommands::One {}) => subcommand_one(),
        Some(NewCommandCommands::Two {}) => subcommand_two(),
        None => {
            panic!("Handled by clap");
        }
    }
}

fn subcommand_one() -> Result<String, Box<dyn std::error::Error>> {
    todo!("Implement subcommand one");
}

fn subcommand_two() -> Result<String, Box<dyn std::error::Error>> {
    todo!("Implement subcommand two");
}
