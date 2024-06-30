use std::path::PathBuf;
use clap::{Parser, Subcommand};

use hledger_helper::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
    CSV {
        #[arg(value_name = "PATH")]
        path: String,

        #[arg(value_name = "STATEMENT_TYPE")]
        statement_type: String,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Test {
            list,
        } => {
            println!("matched test, {}", list);
        },

        Commands::CSV {
            path,
            statement_type,
        } => {
            let st_type = match statement_type.to_lowercase().as_str() {
                "clcard" => StatementType::ClCardCSV,
                "chase" => StatementType::ChaseCSV,
                _ => StatementType::Unknown,
            };

            let statement = BankStatement::from_csv(
                &path, st_type
            ).unwrap();
            println!("{}", statement);
        }
    }
}
