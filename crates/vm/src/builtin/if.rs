//BinaryHeap;

use sexprs_data_structures::{append, car, cdr, AsSymbol, Quotable, Value};
use sexprs_util::{try_result, warn};
use unique_pointer::UniquePointer;

use crate::helpers::runtime_error;
use crate::{Context, Result, Sym};

pub fn r#if<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    // let name = car(&list).as_symbol();
    // let list = append(list);
    let r#if = car(&list);
    let r#then = car(&cdr(&list));
    let r#else = cdr(&cdr(&list));

    let mut value = if Into::<bool>::into(try_result!(vm.clone().eval(r#if))) {
        try_result!(vm.clone().eval(r#then))
    } else {
        try_result!(vm.clone().eval(r#else))
    };
    while !value.is_quoted() && value.is_list() && !value.is_empty() {
        let new_value = try_result!(vm.clone().eval(value.clone()));
        if new_value == value {
            break
        } else {
            value = new_value;
        }
    }
    Ok(try_result!(vm.clone().eval(value)))
}
