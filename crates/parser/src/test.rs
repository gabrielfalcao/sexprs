use std::borrow::Cow;

use crate::{Source, Span, SpanPosition};

#[macro_export]
macro_rules! assert_parsed_display {
    ($code:literal) => {{
        use sexprs_parser::parse_source;

        let value = match parse_source($code) {
            Ok(value) => value,
            Err(error) => {
                eprintln!(
                    "{}",
                    [
                        format!("when parsing:"),
                        $code.to_string(),
                        format!("error: {}", error)
                    ]
                    .join("\n")
                );
                std::process::exit(101);
            },
        };
        let code = value.to_string();
        k9::assert_equal!(code, $code);
    }};
}

pub fn stub_span_info<'a>(
    input: &'a str,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> Span<'a> {
    let span_info = Span {
        source: Source {
            source: Cow::from(input),
            filename: None,
        },
        name: None,
        input: Cow::from(input),
        start_pos: SpanPosition::from_tuple(start_pos),
        end_pos: SpanPosition::from_tuple(end_pos),
        inner: None,
    };
    span_info
}
pub fn stub_input<'a>(input: &'a str) -> (String, Span<'a>) {
    let span_info = stub_span_info(input, (1, 1), (1, input.len() + 1));
    (input.to_string(), span_info)
}
