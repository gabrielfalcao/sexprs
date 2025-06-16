use std::borrow::{Borrow, Cow};


use pest::error::LineColLocation;
use pest::iterators::Pair;

use crate::{Rule, Source, SpanPosition};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span<'a> {
    pub input: Cow<'a, str>,
    pub name: Option<String>,
    pub start_pos: SpanPosition,
    pub end_pos: SpanPosition,
    pub source: Source<'a>,
    pub inner: Option<Vec<Span<'a>>>,
}
impl<'a> Span<'a> {
    pub fn from_pair(pair: Pair<'a, Rule>, source: Source<'a>) -> Span<'a> {
        let span = pair.as_span();
        let start_pos = SpanPosition::from_pest(span.start_pos());
        let end_pos = SpanPosition::from_pest(span.end_pos());

        Span {
            input: Cow::from(span.as_str()),
            name: Some(format!("{:#?}", pair.as_rule())),
            start_pos,
            end_pos,
            source: source.clone(),
            inner: {
                let inner = pair.clone().into_inner();
                if inner.peek().is_none() {
                    None
                } else {
                    Some(
                        inner
                            .map(|pair| {
                                Span::from_pair(
                                    pair.clone(),
                                    source.clone(),
                                )
                            })
                            .collect(),
                    )
                }
            },
        }
    }

    pub fn from_error(error: pest::error::Error<Rule>, source: Source<'a>) -> Span<'a> {
        let (start_pos, end_pos) = match error.line_col.clone() {
            LineColLocation::Pos(line_col) => (
                SpanPosition::from_tuple(line_col.clone()),
                SpanPosition::from_tuple(line_col.clone()),
            ),
            LineColLocation::Span(start_pos, end_pos) =>
                (SpanPosition::from_tuple(start_pos), SpanPosition::from_tuple(end_pos)),
        };
        Span {
            input: Cow::from(error.line().to_string()),
            name: None,
            start_pos,
            end_pos,
            source: source,
            inner: None,
        }
    }

    pub fn input(&'a self) -> &'a str {
        self.input.borrow()
    }

    pub fn inner(&self) -> Vec<Span<'a>> {
        if let Some(spans) = &self.inner {
            spans.clone()
        } else {
            Vec::new()
        }
    }

    pub fn filename(&self) -> Option<String> {
        self.source.filename()
    }

    pub fn with_input(&self, input: &'a str) -> Span<'a> {
        let mut info = self.clone();
        info.input = Cow::from(input);
        info
    }

    pub fn info(&self) -> Span<'a> {
        self.clone()
    }

    pub fn start_pos(&self) -> (usize, usize) {
        self.start_pos.to_tuple()
    }

    pub fn end_pos(&self) -> (usize, usize) {
        self.end_pos.to_tuple()
    }

    pub fn highlight_input(&self, indent: usize) -> String {
        sexprs_util::color::fore(self.highlight_input_chars(indent), 32)
    }

    fn highlight_input_chars(&self, indent: usize) -> String {
        let start_pos = self.start_pos.clone();
        let end_pos = self.end_pos.clone();
        self.input
            .lines()
            .enumerate()
            .map(|(no, line)| {
                (
                    no + 1,
                    line.chars()
                        .enumerate()
                        .map(|(no, column)| (no + 1, column.to_string()))
                        .collect::<Vec<(usize, String)>>(),
                )
            })
            .map(|(line, columns)| {
                sexprs_util::color::bg(
                    format!(
                        "{}{}",
                        " ".repeat(indent),
                        columns
                            .iter()
                            .map(|(column, text)| {
                                let column = column.clone();
                                if line == start_pos.line && column == start_pos.column {
                                    sexprs_util::color::bgfg(text, 235, 198)
                                } else if line == end_pos.line && column == end_pos.column {
                                    [
                                        sexprs_util::color::reset(""),
                                        sexprs_util::color::bg(text, 235),
                                    ]
                                    .join("")
                                } else {
                                    sexprs_util::color::bg(text, 235)
                                }
                            })
                            .collect::<String>()
                    ),
                    235,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<'a> std::fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                self.input.to_string(),
                if let Some(spans) = &self.inner {
                    format!(
                        ", [{}]",
                        spans
                            .iter()
                            .map(|span| span.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    String::new()
                }
            ]
            .join("")
            .trim()
        )
    }
}
impl<'a> std::fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                self.name.clone().map(String::from).unwrap_or_else(|| "Span".to_string()),
                " {".to_string(),
                [
                    format!("input: '{}'", self.input),
                    if let Some(spans) = &self.inner {
                        format!("spans: {:#?}", &spans)
                    } else {
                        String::new()
                    }
                ]
                .iter()
                .filter(|string| string.len() > 0)
                .map(String::from)
                .collect::<Vec<String>>()
                .join(", "),
                "}".to_string(),
            ]
            .join("")
        )
    }
}
