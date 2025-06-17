use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

use sexprs_data_structures::{AsValue, Symbol, Value};
use sexprs_util::try_result;
use unique_pointer::UniquePointer;

use crate::{builtin, BuiltinFunction, Context, Function, Result, Sym};

pub type SymTable<'c> = BTreeMap<Symbol<'c>, Sym<'c>>;

#[derive(Clone)]
pub struct SymbolTable<'c> {
    pub(crate) globals: SymTable<'c>,
    pub(crate) locals: SymTable<'c>,
    pub(crate) function_locals: SymTable<'c>,
}
impl<'c> Debug for SymbolTable<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SymbolTable {{
        globals: {:#?},
        locals: {:#?}
    }}",
            &self.globals, &self.locals,
        )
    }
}
impl<'c> Display for SymbolTable<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SymbolTable {{
        globals: {:#?},
        function_locals: {:#?},
        locals: {:#?}
    }}",
            &self.globals, &self.function_locals, &self.locals,
        )
    }
}

impl<'c> SymbolTable<'c> {
    pub fn new() -> SymbolTable<'c> {
        SymbolTable::with_locals(SymTable::new())
    }

    pub fn with_locals(locals: SymTable<'c>) -> SymbolTable<'c> {
        let mut globals = SymTable::<'c>::new();
        register_builtin_function(&mut globals, "t", builtin::identity::t);

        register_builtin_function(&mut globals, "setq", builtin::state::setq);
        register_builtin_function(&mut globals, "defun", builtin::state::defun);

        register_builtin_function(&mut globals, "car", builtin::list::car);
        register_builtin_function(&mut globals, "cdr", builtin::list::cdr);
        register_builtin_function(&mut globals, "cons", builtin::list::cons);
        register_builtin_function(&mut globals, "list", builtin::list::list);
        register_builtin_function(&mut globals, "append", builtin::list::append);
        register_builtin_function(&mut globals, "quote", builtin::list::quote);
        register_builtin_function(&mut globals, "print", builtin::string::print);
        register_builtin_function(&mut globals, "backquote", builtin::list::backquote);

        register_builtin_function(&mut globals, "if", builtin::r#if::r#if);

        register_builtin_function(&mut globals, "listp", builtin::r#type::listp);
        register_builtin_function(&mut globals, "null", builtin::r#type::null);

        register_builtin_function(&mut globals, "*", builtin::math::arithmetic::mul);
        register_builtin_function(&mut globals, "+", builtin::math::arithmetic::add);
        register_builtin_function(&mut globals, "-", builtin::math::arithmetic::sub);
        register_builtin_function(&mut globals, "/", builtin::math::arithmetic::div);

        let table = SymbolTable {
            globals: globals.clone(),
            function_locals: SymTable::new(),
            locals,
        };
        table
    }

    pub fn extend(&mut self, other: Self) {
        self.globals.extend(other.globals.clone());
        self.locals.extend(other.locals.clone());
    }

    pub fn set_global(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        let previous =
            try_result!(set_within_map(&mut self.globals, context, sym, item));
        Ok(previous)
    }

    pub fn set_local(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        let previous =
            try_result!(set_within_map(&mut self.locals, context, sym, item));
        Ok(previous)
    }

    pub fn set_function_local(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        let previous =
            try_result!(set_within_map(&mut self.function_locals, context, sym, item));
        Ok(previous)
    }

    pub fn get(
        &mut self,
        _vm: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
    ) -> Result<Sym<'c>> {
        if let Some(value) = self
            .function_locals
            .get(sym)
            .map(Clone::clone)
            .or_else(|| self.locals.get(sym).map(Clone::clone))
            .or_else(|| self.globals.get(sym).map(Clone::clone))
        {
            return Ok(value);
        } else {
            self.locals
                .insert(sym.clone(), Sym::Value(sym.as_value()));
            return Ok(Sym::Value(sym.as_value()));
        }
    }
}

fn register_builtin_function<'c>(
    table: &mut SymTable<'c>,
    sym: &str,
    function: BuiltinFunction,
) {
    let function = Sym::<'c>::Function(Function::Builtin {
        name: Symbol::new(sym),
        function: function,
    });
    table.insert(Symbol::new(sym), function.clone());
}

fn set_within_map<'c>(
    map: &mut SymTable<'c>,
    _context_: UniquePointer<Context<'c>>,
    sym: &Symbol<'c>,
    item: &Sym<'c>,
) -> Result<Value<'c>> {
    let previous = map.insert(sym.clone(), item.clone());

    Ok(match previous.unwrap_or_else(|| item.clone()) {
        Sym::Value(_) => item.clone(),
        Sym::Function(Function::Defun { .. }) => item.clone(),
        Sym::Function(Function::Builtin { .. }) => item.clone(),
    }
    .as_value())
}
