#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{list, Value};
use sexprs_vm::{Result, VirtualMachine};

#[test]
fn test_car() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("car"),
        list([
            Value::symbol("a"),
            Value::symbol("b"),
            Value::symbol("c"),
        ]),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, Value::symbol("a"));
    Ok(())
}

#[test]
fn test_cdr() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("cdr"),
        list([
            Value::symbol("a"),
            Value::symbol("b"),
            Value::symbol("c"),
        ]),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, list([Value::symbol("b"), Value::symbol("c")]));
    Ok(())
}

#[test]
fn test_append() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("append"),
        list([Value::symbol("a")]),
        list([Value::symbol("b"), Value::symbol("c")]),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(
        val,
        list([
            Value::symbol("a"),
            Value::symbol("b"),
            Value::symbol("c")
        ])
    );
    Ok(())
}
