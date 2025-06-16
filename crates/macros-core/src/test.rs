#[macro_export]
macro_rules! assert_streams_equal {
    ($left:expr, $right:expr) => {{
        use proc_macro2::TokenStream;
        use sexprs_formatter::format_code;
        use iocore::Path;
        let input_stream = TokenStream::from($left.clone());
        let output_stream = TokenStream::from($right.clone());

        let left = format_code(&input_stream);
        let right = format_code(&output_stream);

        if left != right {
            let left_file = Path::tmp_file().write(left.as_bytes()).unwrap();
            let left_file = left_file.rename(left_file.with_filename("left.rs"), true).unwrap();
            let right_file = Path::tmp_file().write(right.as_bytes()).unwrap();
            let right_file = right_file.rename(right_file.with_filename("right.rs"), true).unwrap();

            let (_, stdout, _) = iocore::shell_command_string_output(format!("diff --color=always -u {} {}", left_file, right_file), ".").unwrap();

            eprintln!("streams are not equal, see diff below:");
            panic!("{}", stdout);
        }
    }};
}
