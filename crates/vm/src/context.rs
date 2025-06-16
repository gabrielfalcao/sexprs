use std::fmt::Debug;

use sexprs_data_structures::{
    append, car, cdr, AsSymbol, Cell, Quotable, Symbol, Value,
};
use sexprs_parser::parse_source;
use sexprs_util::try_result;
use unique_pointer::UniquePointer;

use crate::{Function, Result, Sym, SymbolTable, VirtualMachine};

#[allow(unused)]
#[derive(Clone)]
pub struct Context<'c> {
    pub(crate) symbols: SymbolTable<'c>,
    pub(crate) vm: UniquePointer<VirtualMachine<'c>>,
}

impl<'c> Debug for Context<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Context {{
    symbols: {:#?},
}}",
            &self.symbols,
        )
    }
}

impl<'c> Context<'c> {
    pub fn new(
        vm: UniquePointer<VirtualMachine<'c>>,
        symbols: SymbolTable<'c>,
    ) -> Context<'c> {
        let context = Context { symbols, vm };
        context
    }

    pub fn register_function(
        &mut self,
        name: Symbol<'c>,
        args: Value<'c>,
        body: Value<'c>,
    ) -> Result<Value<'c>> {
        let function = Sym::<'c>::Function(Function::Defun {
            name: name.clone(),
            args: args.clone(),
            body: body.clone(),
        });
        try_result!(self.symbols.set_global(
            UniquePointer::read_only(self),
            &name,
            &function.clone(),
        ));

        Ok(function.as_value())
    }

    pub fn symbol_is_function<T: AsSymbol<'c>>(&mut self, sym: T) -> Result<bool> {
        if !sym.is_symbol() {
            return Ok(false);
        }
        let symbol = &sym.as_symbol();
        let sym = try_result!(self
            .symbols
            .get(UniquePointer::read_only(self), &symbol));
        match sym {
            Sym::Value(_) => Ok(false),
            Sym::Function(_) => Ok(true),
        }
    }

    pub fn get_symbol_function<T: AsSymbol<'c>>(
        &mut self,
        sym: T,
    ) -> Result<Option<Function<'c>>> {
        let sym = sym.as_symbol();
        let symbol = try_result!(self
            .symbols
            .get(UniquePointer::read_only(self), &sym));
        let result = match symbol {
            Sym::Value(_) => Ok(None),
            Sym::Function(function) => Ok(Some(function)),
        };
        result
    }

    pub fn eval_symbol_function<T: AsSymbol<'c>>(
        &mut self,
        sym: T,
        list: Value<'c>,
    ) -> Result<Value<'c>> {
        let sym = sym.as_symbol();
        let vm = UniquePointer::read_only(self);

        match try_result!(self.get_symbol_function(&sym)) {
            Some(function) => {
                let result = try_result!(function.call(vm, list));
                Ok(result)
            },
            None => Ok(Value::from({
                let mut cell = Cell::nil();
                cell.push_value(Value::from(sym));
                for item in list.into_iter() {
                    cell.push_value(item);
                }
                cell
            })),
        }
    }

    pub fn eval_string(&mut self, string: &'c str) -> Result<Value<'c>> {
        Ok(try_result!(self.eval(try_result!(parse_source(string)))))
    }

    pub fn eval(&mut self, list: Value<'c>) -> Result<Value<'c>> {
        if list.is_quoted() {
            return Ok(list);
        }
        let head = car(&list);
        if try_result!(self.symbol_is_function(&head)) {
            Ok(try_result!(self.eval_symbol_function(head, cdr(&list))))
        } else {
            Ok(try_result!(self.eval_list(list)))
        }
    }

    pub fn eval_list(&mut self, list: Value<'c>) -> Result<Value<'c>> {
        let mut cell = Cell::nil();
        for value in list.into_iter() {
            cell.push_value(match value {
                Value::Symbol(ref sym) => try_result!(self
                    .symbols
                    .get(UniquePointer::read_only(self), &sym))
                .as_value(),
                _ => value,
            });
        }
        Ok(append(cell))
    }

    pub fn set_global(
        &mut self,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        Ok(try_result!(self.symbols.set_global(
            UniquePointer::read_only(self),
            sym,
            item
        )))
    }

    pub fn set_local(&mut self, sym: &Symbol<'c>, item: &Sym<'c>) -> Result<Value<'c>> {
        Ok(try_result!(self.symbols.set_local(
            UniquePointer::read_only(self),
            sym,
            item
        )))
    }

    pub fn set_function_local(
        &mut self,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        Ok(try_result!(self.symbols.set_function_local(
            UniquePointer::read_only(self),
            sym,
            item
        )))
    }
}
