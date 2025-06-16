use std::borrow::Cow;
pub mod errors;
pub use errors::{Caller, Error, Result};
pub mod macros;
pub mod test;

pub mod source;
use std::str::FromStr;

use sexprs_data_structures::{Cell, Value};
use sexprs_util::unexpected;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
pub use source::{Source, Span, SpanPosition};
pub const GRAMMAR: &'static str = include_str!("./grammar.pest");

#[derive(Parser, Debug, Clone)]
#[grammar = "src/grammar.pest"]
pub struct MinilispSource;
pub fn parse_source<'a>(input: &'a str) -> Result<'a, Value<'a>> {
    let source_info = Source {
        source: Cow::from(input),
        filename: None,
    };
    let mut pairs = MinilispSource::parse(Rule::file, input).map_err(|e| {
        Error::new(
            e.variant.message().to_string(),
            Some(Span::from_error(e, source_info.clone())),
        )
    })?;
    let file = pairs.next().unwrap();
    let nodes = (|pair: Pair<'a, Rule>| -> Option<Value<'a>> {
        Some(pair_to_value(
            pair.clone()
                .into_inner()
                .next()?
                .into_inner()
                .next()?,
        ))
    })(file)
    .unwrap_or_default();
    Ok(nodes)
}

pub fn map_pairs_to_list<'a>(pairs: Pairs<'a, Rule>) -> Value<'a> {
    pairs.map(|pair| pair_to_value(pair)).collect()
}
pub fn pair_to_value<'a>(pair: Pair<'a, Rule>) -> Value<'a> {
    match pair.as_rule() {
        Rule::float =>
            Value::float(f64::from_str(pair.as_span().as_str()).expect("float")),
        Rule::integer =>
            Value::integer(i64::from_str(pair.as_span().as_str()).expect("integer")),
        Rule::string => Value::string(Cow::from(pair.as_span().as_str())),
        Rule::symbol => Value::symbol(Cow::from(pair.as_span().as_str())),
        Rule::quoted_symbol => {
            let mut pairs = pair.clone().into_inner();
            pairs.next().expect("quote");
            let symbol = pairs.next().expect("symbol");
            Value::quoted_symbol(symbol.as_span().as_str())
        },
        Rule::t => Value::T,
        Rule::unsigned => Value::unsigned_integer(
            u32::from_str(pair.as_span().as_str()).expect("unsigned integer"),
        ),
        Rule::value => pair_to_value(pair.clone().into_inner().next().expect("value")),
        Rule::sexpr => {
            let mut items = Cell::nil();
            let mut pairs = pair.clone().into_inner();
            let mut quoted = false;
            loop {
                if let Some(pair) = pairs.peek() {
                    if pair.as_rule() == Rule::close_paren {
                        break;
                    }
                }
                let pair = pairs.next().expect("quote, open_paren or item");
                match pair.as_rule() {
                    Rule::quote => {
                        quoted = true;
                    },
                    Rule::open_paren => continue,
                    Rule::close_paren => continue,
                    _ => {
                        items.push_value(pair_to_value(pair));
                        continue;
                    },
                }
            }
            pairs.next().expect("close_paren");
            let value = Value::from_iter(items.into_iter());
            if quoted {
                value.quote()
            } else {
                value
            }
        },
        Rule::nil => Value::nil(),
        _ => unexpected!(pair),
    }
}
