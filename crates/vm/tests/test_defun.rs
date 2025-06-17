#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{list, Symbol, Value};
use sexprs_util::{dbg, step};
use sexprs_vm::{Result, VirtualMachine};

#[test]
fn test_eval_if_then() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let value = vm.eval_string(
        "
  (if t
      \"true\"
      \"false\")
",
    )?;
    assert_equal!(value.to_string(), "(\"true\")");
    Ok(())
}

#[test]
fn test_eval_defun_recursive_flatten() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.eval_string(
        "
(defun flatten (lst)
  (if (null lst)
      T
    (if (listp (car lst))
        (append
         (flatten (car lst))
         (flatten (cdr lst)))
      (cons (car lst) (flatten (cdr lst))))))
",
    )?;
    let value = vm.eval_string(r#"(flatten '( '(a '( b)) '( c d)))"#)?;
    assert_equal!(value.to_string(), "(a b c d)");

    let value = vm.eval_string(r#"(flatten '( a '( '( b ) c)))"#)?;
    assert_equal!(value.to_string(), "(a b c)");
    Ok(())
}

#[test]
fn test_eval_setq() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.eval_string(r#"(setq a 1)"#)?;
    let val = vm.eval_string(r#"(list a 2)"#)?;
    assert_equal!(
        val,
        list([
            Value::unsigned_integer(1u64),
            Value::unsigned_integer(2u64),
        ])
    );
    Ok(())
}

#[test]
fn test_eval_defun() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.eval_string(r#"(defun sum(a b) (+ a b))"#)?;
    let val = vm.eval_string(r#"(sum 1 1)"#)?;
    assert_equal!(val, Value::unsigned_integer(2u64));
    Ok(())
}

#[test]
fn test_eval_defun_recursive() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.eval_string(r#"(defun sum(a b) (+ a b))"#)?;
    let val = vm.eval_string(r#"(sum 40 (sum 1 1))"#)?;
    assert_equal!(val, Value::unsigned_integer(42u64));
    Ok(())
}

#[test]
fn test_eval_if_else() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let value = vm.eval_string(
        "
  (if nil
      \"true\"
      \"false\")
",
    )?;
    assert_equal!(value.to_string(), "(\"false\")");
    Ok(())
}

#[test]
fn test_eval_listp() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let value = vm.eval_string(
        "
  (listp (list 'a 'b))
",
    )?;
    assert_equal!(value, Value::T);
    Ok(())
}

#[test]
fn test_eval_null() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let value = vm.eval_string(
        "
  (null nil)
",
    )?;
    assert_equal!(value, Value::T);
    Ok(())
}
