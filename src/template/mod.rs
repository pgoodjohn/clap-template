use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct TemplateCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<TemplateCommands>,
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    One {},
    Two {},
}

pub fn command(command: &TemplateCommand) -> Result<String, Box<dyn std::error::Error>> {
    match command.command {
        Some(TemplateCommands::One {}) => subcommand_one(),
        Some(TemplateCommands::Two {}) => subcommand_two(),
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
