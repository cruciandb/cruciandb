use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

pub(crate) fn run_cli_exec_loop(_args: &crate::Args) -> Result<()> {
    let mut liner = Editor::new()?;
    liner.set_helper(Some(()));
    liner.load_history(".history").ok();
    loop {
        let readline = liner.readline(">> ");
        match readline {
            Ok(line) => {
                liner.add_history_entry(line.as_str())?;
                println!("Got Line: {}", line);
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
