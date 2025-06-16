use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::Zip;

use sexprs_data_structures::{
    append, AsSymbol, AsValue, Symbol, Value, ValueIterator,
};
use sexprs_util::{admonition, try_result, warn, with_caller, info};
use unique_pointer::UniquePointer;

use crate::{runtime_error, BuiltinFunction, Context, Result, Sym};

#[derive(Clone)]
pub enum Function<'c> {
    Builtin {
        name: Symbol<'c>,
        function: BuiltinFunction,
    },
    Defun {
        name: Symbol<'c>,
        args: Value<'c>,
        body: Value<'c>,
    },
}
impl<'c> Function<'c> {
    pub fn is_builtin(&self) -> bool {
        match self {
            Function::Builtin {..} => true,
            _ => false
        }
    }
    pub fn is_defun(&self) -> bool {
        match self {
            Function::Defun {..} => true,
            _ => false
        }
    }
    pub fn validate_args(
        &self,
        name: &Symbol<'c>,
        expected: &Value<'c>,
        received: &Value<'c>,
    ) -> Result<Zip<ValueIterator<'c>, ValueIterator<'c>>> {
        let expected_length = expected.len();
        let received_length = received.len();
        if expected_length != received_length {
            Err(with_caller!(runtime_error(
                format!(
                    "{} expected {} args [{:#?}] but received {}: {:#?}",
                    name, expected_length, expected, received_length, received
                ),
                None
            )))
        } else {
            Ok(expected
                .clone()
                .into_iter()
                .zip(received))
        }
    }

    pub fn bind_args_to_local_context(
        &self,
        mut vm: UniquePointer<Context<'c>>,
        name: &Symbol<'c>,
        expected: &Value<'c>,
        received: &Value<'c>,
    ) -> Result<Vec<(Symbol<'c>, Value<'c>)>> {
        let mut args = Vec::<(Symbol<'c>, Value<'c>)>::new();
        for (symbol, value) in try_result!(self.validate_args(name, expected, received))
        {
            args.push((symbol.as_symbol(), value.clone()));
            try_result!(vm
                .inner_mut()
                .set_function_local(&symbol.as_symbol(), &Sym::Value(value.clone())));
        }
        Ok(args)
    }

    pub fn call(
        &self,
        mut vm: UniquePointer<Context<'c>>,
        list: Value<'c>,
    ) -> Result<Value<'c>> {
        match self {
            Function::Defun { name, args, body } => {
                let bound_args = try_result!(self.bind_args_to_local_context(
                    vm.clone(),
                    name,
                    args,
                    &list
                ));

                let mut value = Value::nil();
                for (index, val) in body.into_iter().enumerate() {
                    admonition!(184, "evaluating {}'s #{} body {}", name, index+1, &val);
                    value = try_result!(vm.inner_mut().eval(val));
                }
                Ok(value)
            },
            Function::Builtin { name, function } => {
                //
                Ok(try_result!(function(vm, list)))
            },
        }
    }
}

impl<'c> Display for Function<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::Defun { name, args, body } =>
                    format!("(defun {} {} {})", name, args, body),
                Function::Builtin { name, function } =>
                    format!("builtin-function {} {:#?}", name, function),
            }
        )
    }
}

impl<'c> Debug for Function<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::Defun { name, args, body } =>
                    format!("(defun {} {} {})", name, args, body),
                Function::Builtin { name, function } =>
                    format!("builtin-function {} {:#?}", name, function),
            }
        )
    }
}
