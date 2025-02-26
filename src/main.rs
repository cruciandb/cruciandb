use clap::Parser;
use storage::mem::MemStorage;

mod cli;
pub mod command;
pub mod storage;

pub const DEFAULT_DB_NAME: &str = "db";
pub const DEFAULT_DB_PATH: &str = "./data";

/// Simple storage with command line interface
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Name of the Database
    #[arg(short, long, default_value = DEFAULT_DB_NAME)]
    database: String,

    /// Path to store data
    #[arg(short, long, default_value = DEFAULT_DB_PATH)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Starting for: {} at {:?}", args.database, args.path);
    let mut storage: MemStorage<String, String> = MemStorage::new();
    match cli::run_cli_exec_loop(&mut storage) {
        Ok(_) => println!("Exit normally"),
        Err(e) => println!("Exit with error: {:?}", e),
    }
}
