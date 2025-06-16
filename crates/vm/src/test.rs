#[macro_export]
macro_rules! assert_eval {
    ($input:literal, $value:expr ) => {{
        use sexprs_vm::VirtualMachine;

        let mut vm = VirtualMachine::new();
        let value = match vm.eval_string($input) {
            Ok(value) => value,
            Err(error) => {
                eprintln!(
                    "{}",
                    [
                        format!("when evaluating:"),
                        $input.to_string(),
                        format!("error: {}", error)
                    ]
                    .join("\n")
                );
                std::process::exit(101);
            },
        };
        k9::assert_equal!(value, $value);
    }};
}

#[macro_export]
macro_rules! assert_eval_display {
    ($input:literal => $output:literal ) => {{
        use sexprs_vm::VirtualMachine;

        let mut vm = VirtualMachine::new();
        let value = match vm.eval_string($input) {
            Ok(value) => value,
            Err(error) => {
                eprintln!(
                    "{}",
                    [
                        format!("when evaluating:"),
                        $input.to_string(),
                        format!("error: {}", error)
                    ]
                    .join("\n")
                );
                std::process::exit(101);
            },
        };
        let code = value.to_string();
        k9::assert_equal!(code.trim(), $output.trim());
    }};
}
