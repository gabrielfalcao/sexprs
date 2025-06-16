#![allow(unused)]
pub mod errors;
use std::borrow::Cow;

pub use errors::{Error, Result};
use sexprs_formatter::highlight;
use sexprs_parser::parse_source;
use sexprs_util::dbg;
use sexprs_vm::VirtualMachine;
use rustyline::completion::{Candidate, Completer};
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::{Hint, Hinter};
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Changeset, CompletionType, Context, Helper};

pub struct VirtualMachinePrompt<'c> {
    pub vm: &'c VirtualMachine<'c>,
}
impl<'c> VirtualMachinePrompt<'c> {
    pub fn new(vm: &VirtualMachine<'c>) -> VirtualMachinePrompt<'c> {
        VirtualMachinePrompt {
            vm: unsafe {
                std::mem::transmute::<&VirtualMachine, &'c VirtualMachine>(vm)
            },
        }
    }
}
impl<'c> Highlighter for VirtualMachinePrompt<'c> {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        Cow::from(
            highlight(line.to_string(), "lisp")
                .unwrap_or_else(|_| line.to_string()),
        )
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        // on new line
        // dbg!(prompt, default);
        Cow::from(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        // dbg!(hint);
        // Cow::from(hint)
        Cow::from(
            highlight(hint.to_string(), "list")
                .unwrap_or_else(|_| hint.to_string()),
        )
    }

    fn highlight_candidate<'h>(
        &self,
        candidate: &'h str,
        completion: CompletionType,
    ) -> Cow<'h, str> {
        dbg!(candidate, completion);
        Cow::from(candidate)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        // on key change
        // dbg!(line, pos, &kind);
        false
    }
}

pub struct DummyHint {
    display: String,
    completion: String,
}
impl DummyHint {
    pub fn new(display: String, completion: String) -> DummyHint {
        DummyHint {
            display,
            completion,
        }
    }
}

impl Hint for DummyHint {
    fn display(&self) -> &str {
        self.display.clone().leak()
    }

    fn completion(&self) -> Option<&str> {
        Some(self.completion.clone().leak())
    }
}

impl<'c> Hinter for VirtualMachinePrompt<'c> {
    type Hint = DummyHint;

    fn hint(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Option<Self::Hint> {
        None
        // dbg!(line, pos);
        // Some(DummyHint::new(
        //     "dummy_hint_display".to_string(),
        //     "dummy_hint_completion".to_string(),
        // ))
    }
}

pub struct DummyCandidate {
    display: String,
    replacement: String,
}
impl DummyCandidate {
    pub fn new(display: String, replacement: String) -> DummyCandidate {
        DummyCandidate {
            display,
            replacement,
        }
    }
}

impl Candidate for DummyCandidate {
    fn display(&self) -> &str {
        self.display.clone().leak()
    }

    fn replacement(&self) -> &str {
        self.replacement.clone().leak()
    }
}

impl<'c> Completer for VirtualMachinePrompt<'c> {
    type Candidate = DummyCandidate;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // on tab
        // dbg!(line, pos);
        // if line.contains("dummy") {
        //     Ok((
        //         1,
        //         vec![DummyCandidate::new(
        //             "dummy_candidate_display".to_string(),
        //             "dummy_candidate_replacement".to_string(),
        //         )],
        //     ))
        // } else {
        //     Ok((0, Vec::new()))
        // }
        Ok((0, Vec::new()))
    }

    fn update(
        &self,
        line: &mut LineBuffer,
        start: usize,
        elected: &str,
        cl: &mut Changeset,
    ) {
        dbg!(line, start, elected);
    }
}

impl<'c> Validator for VirtualMachinePrompt<'c> {
    fn validate(
        &self,
        ctx: &mut ValidationContext<'_>,
    ) -> rustyline::Result<ValidationResult> {
        // ValidationResult::Incomplete,
        // ValidationResult::Invalid(Option<String>),
        // ValidationResult::Valid(Option<String>),
        // dbg!(ctx);
        match parse_source(ctx.input()) {
            Ok(value) => Ok(ValidationResult::Valid(None)),
            // Ok(value) => Ok(ValidationResult::Valid(Some(format!(" ;; ok.\n")))),
            Err(e) => Ok(ValidationResult::Invalid(Some(e.to_string()))),
        }
    }

    fn validate_while_typing(&self) -> bool {
        true
    }
}

impl<'c> Helper for VirtualMachinePrompt<'c> {}
