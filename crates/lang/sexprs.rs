use clap::Parser;
use sexprs::cli::ParserDispatcher;
use sexprs::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "sexprs command-line")]
pub struct Cli {
    #[arg()]
    text: Vec<String>,
}
impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        println!("{}", &self.text.join(" "));

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
