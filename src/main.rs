use clap::Parser;

mod cli;
pub mod command;

pub const DEFAULT_DB_NAME: &str = "db";

/// Simple storage with command line interface
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Name of the Database
    #[arg(short, long, default_value = DEFAULT_DB_NAME)]
    database: String,

    /// Path to store data
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Starting for: {} at {:?}", args.database, args.path);
    match cli::run_cli_exec_loop(&args) {
        Ok(_) => println!("Exit normally"),
        Err(e) => println!("Exit with error: {:?}", e),
    }
}
