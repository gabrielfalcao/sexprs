use sexprs_data_structures::{
    assert_display_equal, Cell, Value,
};

#[test]
fn test_nil() {
    assert_display_equal!(Value::Nil, "nil");
}
#[test]
fn test_t() {
    assert_display_equal!(Value::T, "t");
}
#[test]
fn test_string() {
    assert_display_equal!(Value::string("string"), r#""string""#);
}
#[test]
fn test_symbol() {
    assert_display_equal!(Value::symbol("symbol"), "symbol");
}
#[test]
fn test_quotedsymbol() {
    assert_display_equal!(Value::quoted_symbol("symbol"), "'symbol");
}
#[test]
fn test_byte() {
    assert_display_equal!(Value::byte(0xF1), "0xf1");
}
#[test]
fn test_unsignedinteger() {
    assert_display_equal!(Value::unsigned_integer(808u64), "808");
}
#[test]
fn test_integer() {
    assert_display_equal!(Value::integer(-808i64), "-808");
}
#[test]
fn test_float() {
    assert_display_equal!(
        Value::float(2.718281828459045),
        "2.718281828459045"
    );
}
#[test]
fn test_list() {
    assert_display_equal!(
        Value::list({
            let mut cell = Cell::from("head");
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "(head 10 x)"
    );

    assert_display_equal!(
        Value::list({
            let mut cell = Cell::nil();
            cell.add(&Cell::from("head"));
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "(head 10 x)"
    );
}

#[test]
fn test_quotedlist() {
    assert_display_equal!(
        Value::quoted_list({
            let mut cell = Cell::from("head");
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "'(head 10 x)"
    );

    assert_display_equal!(
        Value::quoted_list({
            let mut cell = Cell::nil();
            cell.add(&Cell::from("head"));
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "'(head 10 x)"
    );
}
#[test]
fn test_emptylist() {
    assert_display_equal!(Value::empty_list(), "()");
    assert_display_equal!(Value::EmptyList, "()");
}
#[test]
fn test_emptyquotedlist() {
    assert_display_equal!(Value::empty_quoted_list(), "'()");
    assert_display_equal!(Value::EmptyQuotedList, "'()");
}

#[test]
fn test_value_list_quoted() {
    let value = Value::from({
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    });
    assert_display_equal!(&value, "(a b c)");

    let value = value.quote();
    assert_display_equal!(&value, "'(a b c)");
}

#[test]
fn test_value_list_quoted_twice() {
    let value = Value::from({
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    });
    let value = value.quote();
    let value = value.quote();
    assert_display_equal!(&value, "'(a b c)");
}

#[test]
fn test_value_symbol_quoted() {
    let value = Value::symbol("a");
    assert_display_equal!(&value, "a");

    let value = value.quote();
    assert_display_equal!(&value, "'a");
}

#[test]
fn test_value_symbol_quoted_twice() {
    let value = Value::symbol("a");
    let value = value.quote();
    let value = value.quote();
    assert_display_equal!(&value, "'a");
}
