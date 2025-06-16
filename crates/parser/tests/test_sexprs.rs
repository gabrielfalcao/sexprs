use sexprs_parser::{assert_parsed_display, Result};

#[test]
fn test_list_quoted_sexprs() -> Result<'static, ()> {
    assert_parsed_display!("(list 'a 'b 'c)");
    assert_parsed_display!("'(a b c)");
    assert_parsed_display!("(list '(x y z) 3)");
    assert_parsed_display!("'((x y z) 3)");
    assert_parsed_display!("(list 'a 'b 'c)");
    assert_parsed_display!("(a b c)");
    assert_parsed_display!("'(x y z)");
    assert_parsed_display!("'((x y z) 3)");
    assert_parsed_display!("(list '(x y z) 3)");
    assert_parsed_display!("'('(x y z) 3)");
    Ok(())
}
