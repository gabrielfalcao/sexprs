use std::marker::PhantomData;

use k9::assert_equal;
use sexprs_decorators::ast;

#[ast]
pub enum Value {
    String(String),
    Integer(i8),
}

#[test]
fn test_ast_node_enum() {
    let string = Value::String(Default::default());
    assert_equal!(format!("{}", string), "Value::String");
    assert_equal!(format!("{:#?}", string), "Value::String(\"\")");
    let integer = Value::Integer(i8::MAX);
    assert_equal!(format!("{}", integer), "Value::Integer");
    assert_equal!(format!("{:#?}", integer), "Value::Integer(127)");
    let none = Value::None;
    assert_equal!(format!("{}", none), "Value::None");
    assert_equal!(format!("{:#?}", none), "Value::None");
}
