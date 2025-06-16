pub use errors::{Error, ErrorType, Result};
pub mod errors;
pub mod naive;
use std::io::{BufReader, Cursor};

use iocore::Path;
pub use naive::{
    delimiter_wrappers, format_code_naive, format_ident, format_item, format_literal,
    format_token_stream, format_token_tree, indent_strings, is_capitalized,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::Item;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn highlight_code(stream: &TokenStream) -> Result<String> {
    match format_code_rustfmt(stream.to_string()) {
        Ok(code) => Ok(try_result!(highlight(code, "rs"))),
        Err(_) => Ok(try_result!(highlight(format_code_string(stream), "rs"))),
    }
}

pub fn highlight_to_tokens<T: ToTokens>(stream: T) -> Result<String> {
    Ok(try_result!(highlight_token_stream(&stream.to_token_stream())))
}
pub fn highlight_token_stream(stream: &TokenStream) -> Result<String> {
    Ok(try_result!(highlight(format!("{:#?}", &stream), "rs")))
}
pub fn highlight_token_tree(tree: &TokenTree) -> Result<String> {
    Ok(try_result!(highlight(format!("{:#?}", tree), "rs")))
}

pub fn highlight(s: String, lang: &str) -> Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();

    let syntax = match ps.find_syntax_by_extension(lang) {
        Some(syntax) => syntax,
        None =>
            return Err(Error::new(
                format!("syntax not found for language {:#?}", lang),
                ErrorType::SyntectError,
            )),
    };
    let theme = try_result!(theme());
    let mut h = HighlightLines::new(syntax, &theme);
    let mut lines = Vec::<String>::new();
    for line in LinesWithEndings::from(&s) {
        let ranges: Vec<(Style, &str)> = try_result!(h.highlight_line(line, &ps));
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        lines.push(format!("{}", escaped));
    }
    Ok(lines.join(""))
}

pub fn format_code(stream: &TokenStream) -> String {
    let source = format!("{}", stream);
    format_code_string(source.as_str())
}

pub fn highlight_code_string<T: std::fmt::Display>(source: T) -> Result<String> {
    let source = source.to_string();
    match syn::parse_str::<Item>(source.as_str()) {
        Ok(source) => Ok(try_result!(highlight_code(&source.to_token_stream()))),
        Err(_) => Ok(try_result!(highlight(format_code_naive(source.as_str()), "rs"))),
    }
}

pub fn format_code_string<T: std::fmt::Display>(source: T) -> String {
    format_code_rustfmt(source.to_string())
        .unwrap_or_else(|_| format_code_naive(source.to_string()))
}

pub fn format_code_rustfmt<T: std::fmt::Display>(source: T) -> Result<String> {
    let source = source.to_string();
    let path = try_result!(Path::tmp_file().write(source.as_bytes()));
    let (status, _, stderr) =
        try_result!(iocore::shell_command_string_output(format!("rustfmt {}", &path), "."));
    if status != 0 {
        let error = stderr
            .lines()
            .filter(|line| line.len() > 3 && line[2..3] == *"|")
            .map(|line| line[4..].to_string())
            .collect::<Vec<String>>()
            .join("\n");
        return Err(Error::new(error, ErrorType::FormatError));
    }
    let formatted = try_result!(path.read());
    path.delete_unchecked();
    Ok(formatted)
}

pub fn theme_from_bytes(bytes: &[u8]) -> Result<Theme> {
    let f = Cursor::new(bytes.to_vec());
    let mut reader = BufReader::new(f);
    let theme = try_result!(ThemeSet::load_from_reader(&mut reader));
    Ok(theme)
}

pub fn theme() -> Result<Theme> {
    Ok(try_result!(theme_from_bytes(include_bytes!("./themes/DarkKorokai.tmTheme"))))
}
