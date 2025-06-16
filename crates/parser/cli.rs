#![allow(unused)]
use sexprs_parser::test::stub_input;
use sexprs_parser::{parse_source, Error, Result, GRAMMAR};
use sexprs_util::dbg;

fn rule_names() -> Result<'static, Vec<String>> {
    match pest_meta::parse_and_optimize(GRAMMAR) {
        Ok((_special_rules, optimized_rules)) => {
            return Ok(optimized_rules.iter().map(|rule| rule.name.to_string()).collect());
        },
        Err(errors) => {
            return Err(Error::new(errors[0].to_string(), None));
        },
    }
}

fn parse() -> Result<'static, ()> {
    let input = r#"

(cons "a" "b")

"#
    .trim();

    let _item = parse_source(&input)?;
    Ok(())
}

fn main() -> Result<'static, ()> {
    // parse()?;
    println!("rule names from grammar: {:#?}", rule_names()?);
    Ok(())
}
