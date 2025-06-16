use sexprs_data_structures::Value;
use k9::assert_equal;

#[test]
fn value_equals() {
    assert_equal!(Value::from("string"), Value::from("string"));
    assert_equal!(Value::from(0xF1u8), Value::from(0xF1u8));
    assert_equal!(Value::from(0xF1u64), Value::from(0xF1u64));
    assert_equal!(Value::from(7i64), Value::from(7i64));
}
#[test]
fn value_ref_equals() {
    assert_equal!(&Value::from("string"), &Value::from("string"));
    assert_equal!(&Value::from(0xF1u8), &Value::from(0xF1u8));
    assert_equal!(&Value::from(0xF1u64), &Value::from(0xF1u64));
    assert_equal!(&Value::from(7i64), &Value::from(7i64));
}
#[test]
fn value_option_ref_equals() {
    assert_equal!(Some(&Value::from("string")), Some(&Value::from("string")));
    assert_equal!(Some(&Value::from(0xF1u8)), Some(&Value::from(0xF1u8)));
    assert_equal!(Some(&Value::from(0xF1u64)), Some(&Value::from(0xF1u64)));
    assert_equal!(Some(&Value::from(7i64)), Some(&Value::from(7i64)));
}


#[test]
fn value_symbol() {
    assert_equal!(Value::symbol("sym"), Value::symbol("sym"));
}


#[test]
fn value_integer() {
    assert_equal!(Value::integer(1), Value::integer(1));
}
