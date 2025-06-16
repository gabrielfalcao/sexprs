use sexprs_data_structures as ds;
use sexprs_data_structures::{AsCell, Cell, Quotable, Value};
use sexprs_util::{dbg, try_result, info, warn, admonition};
use unique_pointer::UniquePointer;

use crate::helpers::runtime_error;
use crate::{Context, Result};

pub fn list<'c>(vm: UniquePointer<Context<'c>>, list: Value<'c>) -> Result<Value<'c>> {
    Ok(ds::list(try_result!(vm.clone().eval(list))))
}

pub fn cons<'c>(vm: UniquePointer<Context<'c>>, list: Value<'c>) -> Result<Value<'c>> {
    let cell = ds::cons(ds::car(&list), &mut ds::cdr(&list).as_cell());
    Ok(if list.is_quoted() {
        Value::quoted_list(cell)
    } else {
        Value::list(cell)
    })
}
pub fn quote<'c>(
    vm: UniquePointer<Context<'c>>,
    value: Value<'c>,
) -> Result<Value<'c>> {
    Ok(match &value {
        Value::List(_) => value.clone().quote(),
        Value::QuotedList(_) => value.clone().quote(),
        Value::Symbol(_) => value.clone().quote(),
        Value::QuotedSymbol(_) => value.clone().quote(),
        item =>
            return Err(runtime_error(
                format!("quote invoked with non-symbol and non-list: {:#?}", item),
                None,
            )),
    })
}
pub fn backquote<'c>(
    vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    Ok(list)
}

pub fn car<'c>(vm: UniquePointer<Context<'c>>, list: Value<'c>) -> Result<Value<'c>> {
    Ok(ds::car(&ds::append(list)))
}

pub fn cdr<'c>(vm: UniquePointer<Context<'c>>, list: Value<'c>) -> Result<Value<'c>> {
    Ok(ds::cdr(&ds::append(list)))
}
pub fn append<'c>(
    vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    Ok(ds::append(list))
}
