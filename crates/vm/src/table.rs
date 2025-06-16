use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

use sexprs_data_structures::{AsValue, Cell, Quotable, Symbol, Value};
use sexprs_util::{admonition, info, try_result, unexpected, warn, with_caller};
use unique_pointer::UniquePointer;

use crate::{builtin, BuiltinFunction, Context, Function, Result, Sym, VirtualMachine};

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
            &self.globals,
            &self.locals,
            // &debug(&self.globals),
            // &debug(&self.locals),
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
            &self.globals,
            &self.function_locals,
            &self.locals,
            // &debug(&self.globals),
            // &debug(&self.locals),
        )
    }
}

impl<'c> SymbolTable<'c> {
    pub fn new() -> SymbolTable<'c> {
        // info!("SymbolTable.new");
        SymbolTable::with_locals(SymTable::new())
    }

    pub fn with_locals(locals: SymTable<'c>) -> SymbolTable<'c> {
        // info!("SymbolTable.with_locals");
        let mut globals = SymTable::<'c>::new();
        // identity functions
        register_builtin_function(&mut globals, "t", builtin::identity::t);

        // state side-effect functions
        register_builtin_function(&mut globals, "setq", builtin::state::setq);
        register_builtin_function(&mut globals, "defun", builtin::state::defun);

        // list functions
        register_builtin_function(&mut globals, "car", builtin::list::car);
        register_builtin_function(&mut globals, "cdr", builtin::list::cdr);
        register_builtin_function(&mut globals, "cons", builtin::list::cons);
        register_builtin_function(&mut globals, "list", builtin::list::list);
        register_builtin_function(&mut globals, "append", builtin::list::append);
        register_builtin_function(&mut globals, "quote", builtin::list::quote);
        register_builtin_function(&mut globals, "print", builtin::string::print);
        register_builtin_function(&mut globals, "backquote", builtin::list::backquote);

        // arithmetic functions
        register_builtin_function(&mut globals, "*", builtin::math::arithmetic::mul);
        register_builtin_function(&mut globals, "+", builtin::math::arithmetic::add);
        register_builtin_function(&mut globals, "-", builtin::math::arithmetic::sub);
        register_builtin_function(&mut globals, "/", builtin::math::arithmetic::div);

        let mut table = SymbolTable {
            globals: globals.clone(),
            function_locals: SymTable::new(),
            locals,
        };
        // dbg!(&globals, &table);
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
        // info!(184, "SymbolTable.set_global {} {}" , &sym, &item);
        let previous =
            try_result!(set_within_map(&mut self.globals, context, sym, item));
        let symbols = &self.globals;
        // dbg!(symbols);
        Ok(previous)
    }

    pub fn set_local(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        // info!(220, "SymbolTable.set_local {} {}" , &sym, &item);
        let previous =
            try_result!(set_within_map(&mut self.locals, context, sym, item));
        let symbols = &self.locals;
        // dbg!(symbols);
        Ok(previous)
    }

    pub fn set_function_local(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        // info!(220, "SymbolTable.set_function_local {} {}" , &sym, &item);
        let previous =
            try_result!(set_within_map(&mut self.function_locals, context, sym, item));
        let symbols = &self.function_locals;
        // dbg!(symbols);
        Ok(previous)
    }

    pub fn get(
        &mut self,
        mut vm: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
    ) -> Result<Sym<'c>> {
        // info!(51, "SymbolTable.get {:#?}" , &sym);
        // dbg!(&sym, &self.globals, &self.locals);
        if let Some(value) = self
            .function_locals
            .get(sym)
            .map(Clone::clone)
            .or_else(|| self.locals.get(sym).map(Clone::clone))
            .or_else(|| self.globals.get(sym).map(Clone::clone))
        {
            // warn!(202, "FOUND {:#?}" , &value);
            return Ok(value);
        } else {
            // warn!(33, "NOT FOUND {:#?}" , &sym);
            // trying to get a non-existing symbol places it into
            // the local context
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
    context: UniquePointer<Context<'c>>,
    sym: &Symbol<'c>,
    item: &Sym<'c>,
) -> Result<Value<'c>> {
    let previous = map.insert(sym.clone(), item.clone());

    Ok(match previous.unwrap_or_else(|| item.clone()) {
        Sym::Value(value) => {
            // info!(29, "set_within_map: Value");
            // dbg!(&(value,), &item, &sym);
            item.clone()
        },
        Sym::Function(Function::Defun { name, args, body }) => {
            // info!(184, "set_within_map: Function::Defun");
            // dbg!(&(name, args, body), &item, &sym);
            item.clone()
        },
        Sym::Function(Function::Builtin { name, function }) => {
            // info!(231, "set_within_map: Function::Builtin");
            // dbg!(&(name, function), &item, &sym);
            item.clone()
        },
    }
    .as_value())
}
