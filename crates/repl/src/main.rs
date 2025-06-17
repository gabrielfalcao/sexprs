#![allow(unused)]
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{
    Changeset, CompletionType, Config, Context, DefaultEditor, Editor, Helper,
};
use sexprs_formatter::highlight;
use sexprs_parser::parse_source;
use sexprs_repl::{Result, VirtualMachinePrompt};
use sexprs_util::color;
use sexprs_vm::VirtualMachine;

fn print_error<T: std::fmt::Display>(error: T) {
    eprintln!(
        "{}",
        [
            color::ansi("Error:", 196, 16),
            color::ansi(error.to_string(), 220, 16),
        ]
        .join(" ")
    );
}
fn main() -> Result<()> {
    Ok(repl()?)
}
fn clear_screen() {
    print!("\x1b[2J\x1b[3J\x1b[H");
}
fn header() {
    clear_screen();
    println!("\x1b[1;48;5;16m
\x1b[1;38;5;83m  ____    \x1b[1;38;5;206m __   __  _  _____   _ __   ____
\x1b[1;38;5;83m /',__\\  \x1b[1;38;5;206m/'__`\\/\\ \\/'\\/\\ '__`\\/\\`'__\\/',__\\
\x1b[1;38;5;83m/\\__, `\\\x1b[1;38;5;206m/\\  __/\\/>  </\\ \\ \\L\\ \\ \\ \\//\\__, `\\
\x1b[1;38;5;83m\\/\\____/\x1b[1;38;5;206m\\ \\____\\/\\_/\\_\\\\ \\ ,__/\\ \\_\\\\/\\____/
\x1b[1;38;5;83m \\/___/  \x1b[1;38;5;206m\\/____/\\//\\/_/ \\ \\ \\/  \\/_/ \\/___/
          \x1b[1;38;5;206m               \\ \\_\\
          \x1b[1;38;5;206m                \\/_/
\x1b[1;38;5;220m
\x1b[1;38;5;83mS\x1b[1;38;5;231m-\x1b[1;38;5;206mexprs \x1b[1;38;5;231mVM\x1b[1;38;5;83m version {}\x1b[0m", env!("CARGO_PKG_VERSION"));
}
fn repl<'a>() -> Result<()> {
    let config = Config::builder()
        // .history_ignore_dups(true)?
        // .history_ignore_space(false)
        .edit_mode(rustyline::config::EditMode::Emacs)
        .auto_add_history(true)
        .color_mode(rustyline::config::ColorMode::Enabled)
        .behavior(rustyline::config::Behavior::PreferTerm)
        .tab_stop(4)
        .check_cursor_position(true)
        .build();

    let mut vm = VirtualMachine::new();
    let vmp = VirtualMachinePrompt::new(&vm);
    let history = rustyline::history::FileHistory::with_config(config.clone());
    let mut rl =
        Editor::<VirtualMachinePrompt, FileHistory>::with_history(config, history)?;
    rl.set_helper(Some(vmp));
    header();
    if rl.load_history(".sexprs.history").is_err() {
        println!("\x1b[1;38;5;237mno previous history.\x1b[0m");
    }
    loop {
        let readline = rl.readline(": ");
        match readline {
            Ok(line) => {
                let line: &'a str = line.clone().leak();
                rl.add_history_entry(line)?;
                match parse_source(line) {
                    Ok(value) => match vm.eval(value) {
                        Ok(value) => {
                            println!("{}", highlight(value.to_string(), "lisp")?);
                        },
                        Err(error) => {
                            print_error(error);
                        },
                    },
                    Err(error) => {
                        print_error(error);
                        continue;
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }
    rl.save_history(".sexprs.history")?;
    Ok(())
}
