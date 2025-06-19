#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{
    append, assert_debug_equal, assert_display_equal, car, cdr, cons, list, setcar,
    setcdr, AsValue, Cell, Value,
};

#[test]
fn test_cons() {
    let cell = cons("head", &mut Cell::from("tail"));
    assert_equal!(
        cell.values(),
        vec![
            Value::symbol("head"),
            Value::symbol("tail")
        ]
    );
}

#[test]
fn test_list() {
    let value = list([
        Value::symbol("head"),
        Value::symbol("middle"),
        Value::from(33u8),
        Value::from("tail"),
    ]);
    assert_display_equal!(value, r#"(head middle 0x21 "tail")"#);
}

#[test]
fn test_car() {
    let value = list([
        Value::from("head"),
        Value::from("middle"),
        Value::from(33u8),
        Value::from("tail"),
    ]);
    assert_equal!(value.head(), Some(Value::from("head")));
    assert_equal!(car(&value), Value::from("head"));
    let value = list([Value::symbol("head"), Value::from("tail")]).quote();
    assert_equal!(value.head(), Some(Value::symbol("head")));
    assert_eq!(car(&value), Value::quoted_symbol("head"));
}

#[test]
fn test_cdr() {
    let value = list([
        Value::symbol("a"),
        Value::symbol("b"),
        Value::symbol("c"),
    ]);
    assert_equal!(cdr(&value), list([Value::symbol("b"), Value::symbol("c")]));
}

#[test]
fn test_append() {
    let cell = append([
        list([
            Value::symbol("list1-head"),
            Value::symbol("list1-tail"),
        ]),
        Value::from("middle"),
        list([
            Value::symbol("list2-head"),
            Value::symbol("list2-tail"),
        ]),
    ]);
    assert_equal!(
        cell.values(),
        vec![
            Value::symbol("list1-head"),
            Value::symbol("list1-tail"),
            Value::from("middle"),
            Value::symbol("list2-head"),
            Value::symbol("list2-tail"),
        ]
    );
}

#[test]
fn test_setcdr() {
    let mut cell = cons(Value::from(1), &mut Cell::from(Value::from(2)));
    // create ring
    setcdr(cell.tail.inner_mut(), &cell);

    assert_equal!(car(&cell.as_value()), Value::from(1));
    assert_equal!(car(&cdr(&cell.as_value())), Value::from(2));
    assert_equal!(car(&cdr(&cdr(&cell.as_value()))), Value::from(1));
}
