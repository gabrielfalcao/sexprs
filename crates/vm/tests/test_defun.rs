#![allow(unused)]
use k9::assert_equal;
use sexprs_data_structures::{list, Symbol, Value};
use sexprs_util::{dbg, step};
use sexprs_vm::{Result, VirtualMachine};

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

// // #[test]
// // fn test_eval_defun_recursive_flatten() -> Result<()> {
// //     let mut vm = VirtualMachine::new();
// //     vm.eval_string(r#"
// // (defun flatten (lst)
// //   (if (null lst)
// //       nil
// //     (if (listp (car lst))
// //         (append
// //          (flatten (car lst))
// //          (flatten (cdr lst)))
// //       (cons (car lst) (flatten (cdr lst))))))
// // "#)?;
// //     let value = vm.eval_string(r#"(flatten '( '(a '( b)) '( c d)))"#)?;
// //     assert_equal!(value.to_string(), "'( 'a 'b 'c 'd )");
// //
// //     let value = vm.eval_string(r#"(flatten '( a '( '( b ) c)))"#)?;
// //     assert_equal!(value.to_string(), "'( 'a 'b 'c )");
// // }
