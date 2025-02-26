use std::str::FromStr;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use crate::command::Command;
use crate::storage::Storage;
use crate::storage::mem::MemStorage;

pub(crate) fn run_cli_exec_loop(storage: &mut MemStorage<String, String>) -> Result<()> {
    let mut liner = Editor::new()?;
    liner.set_helper(Some(()));
    liner.load_history(".history").ok();
    loop {
        let readline = liner.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.as_str();
                liner.add_history_entry(line)?;
                if let Ok(cmd) = Command::from_str(line) {
                    match cmd {
                        Command::Get(key) => {
                            println!("--- GET --->> {:?}", storage.get(&key));
                        }
                        Command::Put(key, value) => {
                            println!("--- PUT --->> {:?}", storage.set(&key, value));
                        }
                        Command::Delete(key) => {
                            println!("--- DELETE --->> {:?}", storage.delete(&key));
                        }
                        _ => println!("Default handler for the command: {:?}", cmd),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    liner.save_history(".history")?;
    Ok(())
}
