 //BinaryHeap;

use sexprs_data_structures::Value;
use unique_pointer::UniquePointer;

use crate::{Result, Context};

pub fn print<'c>(
    _vm_: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    println!(
        "{}",
        list.clone()
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    Ok(list)
}
