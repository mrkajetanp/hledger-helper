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
        } => {
            let statement = BankStatement::from_csv(&path, StatementType::ClCardCSV).unwrap();
            println!("{}", statement);
        }
    }
}
