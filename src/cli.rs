use std::str::FromStr;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use crate::command::Command;

pub(crate) fn run_cli_exec_loop(_args: &crate::Args) -> Result<()> {
    let mut liner = Editor::new()?;
    liner.set_helper(Some(()));
    liner.load_history(".history").ok();
    loop {
        let readline = liner.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.as_str();
                liner.add_history_entry(line)?;
                let command = Command::from_str(line);
                println!("Line: {}, Command: {:?}", line, command);
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
