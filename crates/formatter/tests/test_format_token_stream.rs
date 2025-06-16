use k9::assert_equal;
use quote::quote;
use sexprs_formatter::{format_token_stream, Result};

#[test]
fn test_format_token_stream_enum_variant() -> Result<()> {
    let stream = quote! {
        Info(Info{id:1,name:"G"})
    };
    assert_equal!(
        format_token_stream(&stream),
        r#"
Info(
    Info {
        id: 1,
        name: "G"
    }
)
"#
        .trim()
    );
    Ok(())
}
