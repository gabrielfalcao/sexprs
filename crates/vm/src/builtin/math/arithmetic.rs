//BinaryHeap;

use sexprs_data_structures::{
    append, car, cdr, AsFloat, AsInteger, AsUnsignedInteger, Cell, Value,
};
use sexprs_util::{admonition, info, try_result, unwrap_result, with_caller};
use unique_pointer::UniquePointer;

use crate::{
    impl_arithmetic_operation, runtime_error, unfold_numeric_values_from_cdr, Context,
    Result,
};

impl_arithmetic_operation!(+ add);
impl_arithmetic_operation!(-sub);
impl_arithmetic_operation!(*mul);
impl_arithmetic_operation!(/ div);

#[macro_export]
macro_rules! unfold_numeric_values_from_cdr {
    (
        $operator:expr,
        $lifetime:lifetime,
        $vm:expr,
        $list:expr,Value::
        $numeric_variant:ident,
        $as_value_fragment_name:ident,
        $is_value_fragment_name:ident $(,)?
    ) => {{
        let mut operands = Cell::nil();
        for value in cdr(&$list).into_iter() {
            if value.$is_value_fragment_name() {
                operands.add(&Cell::from(value));
            } else if value.is_list() {
                operands.add(&Cell::from(try_result!($vm.inner_mut().eval(value))));
            } else if value.is_symbol() {
                operands.add(&Cell::from(try_result!($vm.inner_mut().eval(value))));
            } else {
                return Err(with_caller!(runtime_error(
                    format!(
                        "called with unexpected, non-numerical value: {:#?}",
                        value
                    ),
                    None
                )));
            }
        }
        operands.into_iter().map(|value| {
            if value.is_list() {
                let value = unwrap_result!($vm.clone().inner_mut().eval(value));
                let value = if value.is_list() {
                    car(&value)
                } else  {
                    value
                };
                value
            } else if value.is_symbol() {
                value
            } else {
                value
            }
            .$as_value_fragment_name()
        })

    }};
}

#[macro_export]
macro_rules! impl_arithmetic_operation {
    (
        $operator:tt
            $function_name:ident
    ) => {
        pub fn $function_name<'c>(
            mut vm: UniquePointer<Context<'c>>,
            list: Value<'c>,
        ) -> Result<Value<'c>> {
            let argcount = list.len();
            if argcount < 2 {
                return Err(with_caller!(runtime_error(
                    format!(
                        "{:#?} takes at least 2 arguments, got: {}",
                        stringify!($operator),
                        argcount
                    ),
                    None
                )));
            }
            match &car(&list) {
                Value::UnsignedInteger(list_car)=> {
                    let list_car = list_car.clone();
                    let operands = unfold_numeric_values_from_cdr!("$operator", 'c, vm, list, Value::UnsignedInteger, as_unsigned_integer, is_unsigned_integer);
                    Ok(Value::UnsignedInteger(operands.fold(list_car, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Integer(list_car)=> {
                    let list_car = list_car.clone();
                    let operands = unfold_numeric_values_from_cdr!("$operator", 'c, vm, list, Value::Integer, as_integer, is_integer);
                    Ok(Value::Integer(operands.fold(list_car, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Float(list_car)=> {
                    let list_car = list_car.clone();
                    let operands = unfold_numeric_values_from_cdr!("$operator", 'c, vm, list, Value::Float, as_float, is_float);
                    Ok(Value::Float(operands.fold(list_car, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Symbol(sym) | Value::QuotedSymbol(sym) => {
                    let list_car = car(&try_result!(vm.inner_mut().eval(Value::from(sym))));
                    let list_cdr = cdr(&list);
                    let args = append([list_car.clone(), list_cdr.clone()]);
                    Ok(try_result!($function_name(vm.clone(), args)))
                },
                Value::List(tmp) | Value::QuotedList(tmp) => {
                    let list_car = car(&list);
                    let list_cdr = cdr(&list);
                    let list_car = try_result!(vm.clone().inner_mut().eval(list_car));
                    let args = append([list_car.clone(), list_cdr.clone()]);
                    Ok(try_result!($function_name(vm.clone(), args)))
                },
                value => Err(with_caller!(runtime_error(
                    format!(
                        "{:#?} called with non-numerical value: {:#?}",
                        stringify!($operator),
                        value
                    ),
                    None
                ))),
                _ => {
                    unreachable!()
                },
            }
        }
    };
}
