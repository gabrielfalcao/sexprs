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
use sexprs_data_structures::{append, car, cdr, cons, list, setcar, setcdr, assert_display_equal};

#[test]
fn test_list_quoted_sexprs() {
    // (list 'a 'b 'c) => '(a b c)
    // (list '(x y z) 3) => '((x y z) 3)
    let list_1 = {
        let mut cell = Cell::from(Value::symbol("a").quote());
        cell.add(&Cell::from(Value::symbol("b").quote()));
        cell.add(&Cell::from(Value::symbol("c").quote()));
        Value::QuotedList(cell)
    };
    let list_2 = {
        let mut cell = Cell::from(Value::symbol("x").quote());
        cell.add(&Cell::from(Value::symbol("y").quote()));
        cell.add(&Cell::from(Value::symbol("z").quote()));
        let mut cell = Cell::from(Value::QuotedList(cell));
        cell.add(&Cell::from(Value::integer(3)));
        Value::QuotedList(cell)
    };

    // (list 'a 'b 'c) => (a b c)
    assert_display_equal!(
        list([
            Value::symbol("a").quote(),
            Value::symbol("b").quote(),
            Value::symbol("c").quote(),
        ]),
        r#"('a 'b 'c)"#
    );
    // '(x y z) '((x y z) 3)
    assert_display_equal!(
        Value::quoted_list([
            Value::symbol("x"),
            Value::symbol("y"),
            Value::symbol("z"),
        ]),
        "'(x y z)"
    );
    // (list '(x y z) 3) => '('(x y z) 3)
    assert_display_equal!(
        list([
            Value::quoted_list([
                Value::symbol("x"),
                Value::symbol("y"),
                Value::symbol("z"),
            ]),
            Value::integer(3)
        ]),
        "('(x y z) 3)"
    );
}

// #[test]
// fn test_car_cdr() {
//     // '(a b c)
//     let list = Value::QuotedList({
//         let mut cell = Cell::from("a");
//         cell.add(&Cell::from("b"));
//         cell.add(&Cell::from("c"));
//         cell
//     });
//     // (car '(a b c)) => a
//     assert_equal!(car(&list), Value::symbol("a"));
//     assert_equal!(car(&list).to_string(), r#"a"#);
//     // (cdr '(a b c)) => (b cx)
//     assert_equal!(
//         cdr(&list),
//         {
//             let mut cell = Cell::from("b");
//             cell.add(&Cell::from("c"));
//             cell
//         }
//     );
//     assert_equal!(cdr(&list), Value::List({
//         let mut cell = Cell::from("b");
//         cell.add(&Cell::from("c"));
//         cell
//     }));
//     // (car (cdr '(a b c))) => b
//     assert_equal!(car(&cdr(&list)), Value::symbol("b"));
//     assert_equal!(car(&cdr(&list)).to_string(), r#"b"#);
// }

// #[test]
// fn test_cdr_nil_single_element_list() {
//     // (cdr '(x)) => nil
//     assert_equal!(cdr(&Value::List(Cell::from("x"))), Value::nil());
//     assert_equal!(
//         cdr(&Value::QuotedList(Cell::from("x"))),
//         Value::nil()
//     );
// }

// #[test]
// fn test_car_and_cdr_nil_empty_list() {
//     // (car '()) => nil
//     // (cdr '()) => nil
//     assert_equal!(car(&Value::EmptyList), Value::Nil);
//     assert_equal!(car(&Value::EmptyQuotedList), Value::Nil);
// }

// #[test]
// fn test_car_and_cdr_nil_single_nil_element_list() {
//     // (car '(nil)) => nil
//     // (cdr '(nil)) => nil
//     assert_equal!(car(&Value::List(Cell::nil())), Value::nil());
//     assert_equal!(car(&Value::QuotedList(Cell::nil())), Value::nil());
//     assert_equal!(cdr(&Value::List(Cell::nil())), Value::nil());
//     assert_equal!(cdr(&Value::QuotedList(Cell::nil())), Value::nil());
// }
