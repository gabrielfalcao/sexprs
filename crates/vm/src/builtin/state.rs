//BinaryHeap;

use sexprs_util::{info, warn, admonition};
use sexprs_data_structures::{append, car, cdr, AsSymbol, Value};
use unique_pointer::UniquePointer;

use crate::helpers::runtime_error;
use crate::{Context, Result, Sym};

pub fn setq<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    // info!(202, "setq");
    // let list = vm.eval_list_as_items(list)?;
    if list.len() % 2 != 0 {
        return Err(runtime_error(
            format!("odd number of arguments ({}) in setq: {:#?}", list.len(), list),
            None,
        ));
    }
    let head = car(&list);
    if !head.is_symbol() {
        return Err(runtime_error(
            format!("setq invoked with non-symbol: {:#?}", head),
            None,
        ));
    }
    // dbg!(&head, &tail, &list);
    Ok(vm.set_local(&head.as_symbol(), &Sym::Value(car(&cdr(&list))))?)
}

pub fn defun<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    let name = car(&list).as_symbol();
    let args = car(&cdr(&list));
    let body = cdr(&cdr(&list));
    // info!(184, "defun");
    // dbg!(&name, &args, &body);
    Ok(vm.register_function(name, args, body))
}
