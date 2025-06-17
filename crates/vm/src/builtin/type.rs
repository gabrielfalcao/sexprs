//BinaryHeap;

use sexprs_data_structures::{car, append, cdr, AsSymbol, Value};
use sexprs_util::try_result;
use unique_pointer::UniquePointer;

use crate::helpers::runtime_error;
use crate::{Context, Result, Sym};

pub fn listp<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    let value = try_result!(vm.eval(car(&list)));
    Ok(value.is_list().into())
}

pub fn null<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    let value = try_result!(vm.eval(car(&list)));
    Ok(value.is_nil().into())
}
