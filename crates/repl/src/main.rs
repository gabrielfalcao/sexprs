#![allow(unused)]
use sexprs_formatter::highlight;
use sexprs_parser::parse_source;
use sexprs_repl::{Result, VirtualMachinePrompt};
use sexprs_util::color;
use sexprs_vm::VirtualMachine;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{
    Changeset, CompletionType, Config, Context, DefaultEditor, Editor, Helper,
};

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
    println!("sexprs VM version {}", env!("CARGO_PKG_VERSION"));
}
fn help() {
    println!("\tHELP:");
    println!("\ttype `@' to see the symbol table");
    println!("\ttry arithmetic expressions such as `(* 4 (+ 3 2))'");
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
    let history =
        rustyline::history::FileHistory::with_config(config.clone());
    let mut rl = Editor::<VirtualMachinePrompt, FileHistory>::with_history(
        config, history,
    )?;
    rl.set_helper(Some(vmp));
    header();
    if rl.load_history(".sexprs.history").is_err() {
        println!("No previous history.");
    }
    help();
    loop {
        let readline = rl.readline(": ");
        match readline {
            Ok(line) => {
                let line: &'a str = line.clone().leak();
                rl.add_history_entry(line)?;
                match line.trim() {
                    "@" => {
                        println!("{:#?}", vm.symbols());
                        continue;
                    },
                    _ => match parse_source(line) {
                        Ok(value) => {
                            println!(
                                "{}",
                                highlight(
                                    vm.eval(value)?.to_string(),
                                    "lisp"
                                )?
                            );
                        },
                        Err(error) => {
                            print_error(error);
                            continue;
                        },
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
