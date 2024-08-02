mod generate_from_schema;
mod generate_json_schema;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenerateJsonSchema(generate_json_schema::GenerateJsonSchemaArgs),
    GenerateFromSchema(generate_from_schema::GenerateFromSchemaArgs)
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    match args.command {
        Commands::GenerateJsonSchema(args) => Ok(generate_json_schema::run(args)),
        Commands::GenerateFromSchema(args) => generate_from_schema::run(args),
    }
}
