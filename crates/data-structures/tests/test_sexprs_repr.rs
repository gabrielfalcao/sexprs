#![allow(unused)]
use k9::assert_equal;
#[rustfmt::skip]
use sexprs_data_structures::{
    AsNumber, AsCell, AsValue, Quotable,
};
#[rustfmt::skip]
use sexprs_data_structures::{
    Cell, Value,
};
#[rustfmt::skip]
use sexprs_data_structures::{append, car, cdr, cons, list, setcar, setcdr};

#[test]
fn test_repr() {
    // (a b c)
    // (7 "foo")
    // ((4.12 31178))

    let list_1 = {
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        Value::List(cell)
    };
    let list_2 = {
        let mut cell = Cell::nil();
        cell.add(&Cell::from(Value::integer(7)));
        cell.add(&Cell::from(Value::string("foo")));
        // cell.add(&Cell::from("foo".to_string()));
        Value::List(cell)
    };
    let list_3 = {
        let mut cell = Cell::from(4.12);
        cell.add(&Cell::from(31178));
        Value::List(Cell::from(Value::List(cell)))
    };
    let list_of_list = {
        let mut cell = Cell::from(4.12);
        cell.add(&Cell::from(31178));
        Value::List(cell).wrap_in_list()
    };

    assert_equal!(list_1.to_string(), r#"(a b c)"#);
    assert_equal!(list_2, list([Value::integer(7), Value::string("foo")]));
    assert_equal!(list_2.to_string(), r#"(7 "foo")"#);
    assert_equal!(list_3.to_string(), r#"((4.12 31178))"#);
    assert_equal!(list_of_list.to_string(), r#"((4.12 31178))"#);
    assert_equal!(list_3, list_of_list);
}
