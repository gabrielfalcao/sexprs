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
use sexprs_vm::{assert_eval,assert_eval_display, Result};
#[rustfmt::skip]
use sexprs_data_structures::{append, car, cdr, cons, list, setcar, setcdr, assert_display_equal};

#[test]
fn test_list_quoted_sexprs() -> Result<()> {
    assert_eval_display!(
        "(cdr '('a 'b 'c))" => "('b 'c)"
    );
    assert_eval!(
        "(list 'a 'b 'c)",
        list([
            Value::quoted_symbol("a"),
            Value::quoted_symbol("b"),
            Value::quoted_symbol("c"),
        ])
    );
    assert_eval!(
        "(append '('a 'b) '('c))",
        list([
            Value::quoted_symbol("a"),
            Value::quoted_symbol("b"),
            Value::quoted_symbol("c"),
        ])
    );
    assert_eval_display!(
        "(list 'a 'b 'c)" => "('a 'b 'c)"
    );
    assert_eval_display!(
        "(list '(x y z) 3) " => "(x y z 3)"
    );
    assert_eval_display!(
        "(car '('a 'b 'c))" => "'a"
    );
    assert_eval_display!(
        "(list 'a 'b 'c) " => "('a 'b 'c)"
    );
    assert_eval_display!(
        "'(x y z)" => "'(x y z)"
    );
    assert_eval_display!(
        "(list '(x y z) 3) " => "(x y z 3)"
    );
    Ok(())
}
