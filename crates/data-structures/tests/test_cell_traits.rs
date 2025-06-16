#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{Cell, Value};

#[test]
fn test_iterator() {
    let mut head = Cell::new(Value::from("head"));
    head.add(&mut Cell::new(Value::integer(10)));
    head.add(&mut Cell::new(Value::symbol("x")));

    assert_equal!(
        head.into_iter().map(|value| value.to_string()).collect::<Vec<String>>(),
        vec![r#""head""#, "10", "x"]
    );
}
