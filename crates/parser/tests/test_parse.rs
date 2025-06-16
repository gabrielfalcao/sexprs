#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{list, Value};
use sexprs_parser::test::stub_input;
use sexprs_parser::{parse_source, Result};
use sexprs_util::vec_deque;

#[test]
fn test_cons_of_literal_strings() -> Result<'static, ()> {
    // (cons "a" "b")
    let items = parse_source(r#"(cons "a" "b")"#)?;
    assert_equal!(
        items,
        list([
            Value::symbol("cons"),
            Value::from("a"),
            Value::from("b"),
        ])
    );
    Ok(())
}

#[test]
fn test_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b")
    let items = parse_source(r#"(list "a" "b")"#)?;
    assert_equal!(
        items,
        list([
            Value::symbol("list"),
            Value::from("a"),
            Value::from("b"),
        ])
    );
    Ok(())
}

#[test]
fn test_quoted_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b")
    let items = parse_source(r#"'("a" "b")"#)?;
    assert_equal!(items, list([Value::from("a"), Value::from("b"),]).quote());
    Ok(())
}

#[test]
fn test_call_to_function_add_two_numbers() -> Result<'static, ()> {
    // (+ 1 2)
    let items = parse_source(r#"(+ 1 2)"#)?;
    assert_equal!(
        items,
        list([
            Value::symbol("+"),
            Value::unsigned_integer(1u32),
            Value::unsigned_integer(2u32),
        ])
    );
    Ok(())
}

#[test]
fn test_list_of_literal_strings_and_quoted_list_of_literal_strings(
) -> Result<'static, ()> {
    // (list "a" "b" '("b" "c"))
    let items = parse_source(r#"(list "a" "b" '("c" "d"))"#)?;
    assert_equal!(
        items,
        list([
            Value::symbol("list"),
            Value::from("a"),
            Value::from("b"),
            list([
                Value::from("c"),
                Value::from("d"),
            ]).quote(),
        ])
    );
    Ok(())
}

#[test]
fn test_cons_of_car_literal_string_and_cdr_quoted_list_of_literal_strings(
) -> Result<'static, ()> {
    // (cons "a" '("b" "c"))
    let items = parse_source(r#"(cons "a" '("b" "c"))"#)?;
    assert_equal!(
        items,
        list([
            Value::symbol("cons"),
            Value::from("a"),
            list([
                Value::from("b"),
                Value::from("c"),
            ]).quote(),
        ])
    );
    Ok(())
}

#[test]
fn test_print() -> Result<'static, ()> {
    let items = parse_source(r#"(print "t")"#)?;
    assert_equal!(items, list([Value::symbol("print"), Value::from("t")]));
    Ok(())
}

#[test]
fn test_defun() -> Result<'static, ()> {
    // (defun myfun() (cons "a" '("b" "c")))
    let items = parse_source(r#"(defun myfun() (cons "a" '("b" "c")))"#)?;
    assert_equal!(
        items,
        list([
            Value::symbol("defun"),
            Value::symbol("myfun"),
            list([]),
            list([
                Value::symbol("cons"),
                Value::from("a"),
                list([
                    Value::from("b"),
                    Value::from("c"),
                ]).quote()
            ])
        ])
    );
    Ok(())
}
