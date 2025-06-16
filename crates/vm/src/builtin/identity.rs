#![allow(unused)]
use sexprs_data_structures::Value;

use crate::{Result, Context};
use unique_pointer::UniquePointer;

pub fn t<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    Ok(Value::T)
}
