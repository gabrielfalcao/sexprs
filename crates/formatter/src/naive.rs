use proc_macro2::{Delimiter, Ident, Literal, TokenStream, TokenTree};
use quote::ToTokens;
use sexprs_util::try_result;
use syn::Item;

use crate::Result;

pub fn format_code_naive<C: std::fmt::Display>(source: C) -> String {
    match format_item(source.to_string()) {
        Ok(formatted) => formatted,
        Err(item_format_error) => match syn::parse_str::<TokenStream>(&source.to_string()) {
            Ok(stream) => format_token_stream(&stream),
            Err(to_token_stream_error) =>
                format!("Error({},\n{}\n)", item_format_error, to_token_stream_error),
        },
    }
}
pub fn format_item<C: std::fmt::Display>(source: C) -> Result<String> {
    Ok(format!(
        "{:#?}",
        try_result!(syn::parse_str::<Item>(&source.to_string())).to_token_stream()
    ))
}

pub fn format_token_stream(stream: &TokenStream) -> String {
    let trees = expand_token_stream(&stream);
    let count = trees.len();
    let mut result = Vec::<String>::new();
    let delimiter = Delimiter::None;
    let indent_level = 0;
    let mut index = 0;
    for tree in trees.clone() {
        let prev_sibling = if index == 0 { None } else { Some(trees[index - 1].clone()) };
        let next_sibling = if index < (count - 1) { Some(trees[index + 1].clone()) } else { None };
        let items = format_token_tree(&tree, indent_level, delimiter, prev_sibling, next_sibling);
        result.extend(items);
        index += 1;
    }
    result.join("")
}

pub fn indent<T: std::fmt::Display>(indent_level: usize, string: T) -> String {
    format!("{}{}", " ".repeat(indent_level), string.to_string())
}
pub fn indent_strings(indent_level: usize, strings: Vec<String>) -> Vec<String> {
    strings.iter().map(|string| indent(indent_level, string)).collect()
}

pub fn expand_token_stream(stream: &TokenStream) -> Vec<TokenTree> {
    stream.clone().into_iter().map(|tree| tree.clone()).collect()
}
pub fn format_ident(
    ident: &Ident,
    prev_sibling: Option<TokenTree>,
    next_sibling: Option<TokenTree>,
    indent_level: usize,
) -> String {
    let ident = ident.to_string().trim().to_string();

    let ident = if let Some(TokenTree::Punct(punct)) = prev_sibling {
        match punct.as_char() {
            ':' => ident,
            '\'' => format!("{} ", ident),
            _ => indent(indent_level, ident),
        }
    } else {
        indent(indent_level, ident)
    };
    let suffix = if let Some(TokenTree::Group(group)) = next_sibling {
        match group.delimiter() {
            Delimiter::None | Delimiter::Parenthesis => "",
            Delimiter::Brace | Delimiter::Bracket => " ",
        }
    } else {
        ""
    };
    let result = if is_capitalized(&ident) {
        format!("{}{}", &ident, suffix)
    } else {
        ident.to_string()
    };
    result
}
pub fn format_literal(
    literal: &Literal,
    prev_sibling: Option<TokenTree>,
    _next_sibling: Option<TokenTree>,
    indent_level: usize,
) -> String {
    let literal = literal.to_string().trim().to_string();
    if let Some(TokenTree::Punct(punct)) = prev_sibling {
        match punct.as_char() {
            ':' => literal,
            _ => indent(indent_level, literal),
        }
    } else {
        indent(indent_level, literal)
    }
}
pub fn is_capitalized(string: &str) -> bool {
    if string.is_empty() {
        false
    } else {
        let first_char = &string[0..1];
        first_char == first_char.to_uppercase()
    }
}
pub fn delimiter_wrappers(delimiter: Delimiter) -> (String, String) {
    let (open, close) = match delimiter {
        Delimiter::Parenthesis => ("(", ")"),
        Delimiter::Brace => ("{", "}"),
        Delimiter::Bracket => ("[", "]"),
        Delimiter::None => ("", ""),
    };
    (format!("{}", open), format!("{}", close))
}
pub fn format_token_tree(
    tree: &TokenTree,
    indent_level: usize,
    parent: Delimiter,
    prev_sibling: Option<TokenTree>,
    next_sibling: Option<TokenTree>,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let indent_level = match parent {
        Delimiter::None => indent_level,
        _ => indent_level + 4,
    };
    match tree {
        TokenTree::Group(group) => {
            let delimiter = group.delimiter();

            let trees = expand_token_stream(&group.stream());
            let count = trees.len();
            let line_break = if count > 0 { "\n" } else { "" };
            let indent_level = if count > 0 { indent_level } else { 0 };
            let mut index = 0;
            let (open, close) = delimiter_wrappers(delimiter);
            if !open.is_empty() {
                result.push(format!("{}{}", open, &line_break));
            }
            for tree in trees.clone() {
                let group_prev_sibling =
                    if index == 0 { None } else { Some(trees[index - 1].clone()) };
                let group_next_sibling =
                    if index < (count - 1) { Some(trees[index + 1].clone()) } else { None };
                let items = format_token_tree(
                    &tree,
                    indent_level,
                    delimiter,
                    group_prev_sibling.clone(),
                    group_next_sibling.clone(),
                );
                result.extend(items);
                index += 1;
            }
            if !close.is_empty() {
                result.push(format!("{}{}", &line_break, indent(indent_level, close)));
            }
        },
        TokenTree::Ident(ident) => {
            result.push(format_ident(&ident, prev_sibling, next_sibling, indent_level));
        },
        TokenTree::Punct(punct) => {
            result.push(format!(
                "{}",
                match punct.as_char() {
                    ',' => {
                        format!("{}{}", punct, if next_sibling.is_none() { "" } else { "\n" })
                    },
                    ':' => {
                        format!("{} ", punct)
                    },
                    _ => {
                        format!("{}", punct)
                    },
                }
            ));
        },
        TokenTree::Literal(literal) => {
            result.push(format_literal(&literal, prev_sibling, next_sibling, indent_level));
        },
    }

    result
}
