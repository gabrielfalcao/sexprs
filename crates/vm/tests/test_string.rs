#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{list, Value};
use sexprs_vm::{Result, VirtualMachine};

#[test]
fn test_print() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("print"),
        Value::unsigned_integer(2u64),
        Value::unsigned_integer(2u64),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(
        val,
        list([
            Value::unsigned_integer(2u64),
            Value::unsigned_integer(2u64)
        ])
    );
    Ok(())
}
