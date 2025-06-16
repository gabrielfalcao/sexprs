use iocore::Path;
use sexprs_formatter::{format_code_string, highlight_code_string, Result};

pub struct Cli {
    pub paths: Vec<Path>,
    pub highlight: bool,
}
fn parse_args() -> Cli {
    let mut args = iocore::env::args();
    let program = args.remove(0);
    if args.is_empty() {
        eprintln!("USAGE: {} [-hn] <FILE>", program);
        std::process::exit(1);
    };
    let mut highlight = true;
    let mut paths = Vec::<Path>::new();
    while args.len() > 0 {
        let arg = args.remove(0);
        if arg == "-h" {
            highlight = true;
            continue;
        }
        if arg == "-n" {
            highlight = false;
            continue;
        }
        let path = Path::raw(arg.to_string()).try_canonicalize();
        if path.is_file() {
            paths.push(path);
            continue;
        }
        eprintln!("unexpected argument: {:#?}", arg);
        std::process::exit(1);
    }
    Cli { paths, highlight }
}

fn main() -> Result<()> {
    let cli = parse_args();
    for path in cli.paths.clone() {
        let code = path.read()?;
        if cli.highlight {
            println!("{}", highlight_code_string(&code)?);
        } else {
            println!("{}", format_code_string(&code));
        }
    }
    Ok(())
}
