#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{append, car, cdr, cons, list, Value};
use sexprs_vm::{Result, VirtualMachine};

#[test]
fn test_multiply_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("*"),
        Value::integer(3i64),
        Value::integer(7i64),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, Value::integer(21));
    Ok(())
}

#[test]
fn test_add_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("+"),
        Value::unsigned_integer(2u64),
        Value::unsigned_integer(2u64),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, Value::unsigned_integer(4u64));
    Ok(())
}

#[test]
fn test_subtract_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("-"),
        Value::unsigned_integer(5u64),
        Value::unsigned_integer(2u64),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, Value::unsigned_integer(3u64));
    Ok(())
}

#[test]
fn test_divide_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("/"),
        Value::float(30.0),
        Value::float(3.0),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, Value::float(10.0));
    Ok(())
}

#[test]
fn test_compound_arithmetic() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("+"),
        list([
            Value::symbol("*"),
            Value::integer(3i64),
            Value::integer(7i64),
        ]),
        list([
            Value::symbol("*"),
            Value::integer(4i64),
            Value::integer(5i64),
        ]),
        Value::integer(1i64),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, Value::integer(42));
    Ok(())
}

#[test]
fn test_eval_add_symbols() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.setq("a".into(), Value::unsigned_integer(1u64));
    vm.setq("b".into(), Value::unsigned_integer(1u64));

    let val = vm.eval_string(r#"(+ a b)"#)?;
    assert_equal!(val, Value::unsigned_integer(2u64));
    Ok(())
}
