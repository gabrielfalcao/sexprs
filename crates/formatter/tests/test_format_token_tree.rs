use k9::assert_equal;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenTree};
use quote::quote;
use sexprs_formatter::{format_token_tree, Result};

#[test]
fn test_enum_variant() -> Result<()> {
    // Info()
    assert_equal!(
        format_token_tree(
            &TokenTree::Group(Group::new(Delimiter::Parenthesis, quote! {Info{id:1,name:"G"}})),
            0,
            Delimiter::None,
            Some(TokenTree::Ident(Ident::new("Info", Span::call_site()))),
            None,
        )
        .join("")
        .trim(),
        r#"
(
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
