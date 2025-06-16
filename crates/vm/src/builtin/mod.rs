pub mod identity;
pub mod list;
pub mod math;
pub mod state;
pub mod string;

use sexprs_data_structures::Value;
use unique_pointer::UniquePointer;

use crate::{Context, Result};

pub type BuiltinFunction =
    for<'c> fn(UniquePointer<Context<'c>>, Value<'c>) -> Result<Value<'c>>;
