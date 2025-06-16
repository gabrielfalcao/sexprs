use iocore_test::folder_path;
use sexprs_formatter::format_code;
use sexprs_macros_core::{
    assert_streams_equal, try_result, EnumBuilder, Error, Result,
};
use quote::quote;

#[test]
fn test_enum_builder_impl_and_lifetimes() -> Result<()> {
    let builder = try_result!(EnumBuilder::from_token_stream(&quote! {
        pub enum Value {
            CompileError,
            RuntimeError,
        }
    }));
    let stream = try_result!(builder.build());
    let expected = quote! {
        #[derive(PartialEq, Clone, Eq, PartialOrd, Ord)]
        pub enum Value {
            CompileError,
            RuntimeError,
        }
        impl Value {
            fn variants() -> [&'static str; 2] {
                ["CompileError", "RuntimeError"]
            }
        }
        impl std::fmt::Display for Value {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        Value::CompileError => "CompileError",
                        Value::RuntimeError => "RuntimeError",
                    }
                )
            }
        }
        impl std::fmt::Debug for Value {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        Value::CompileError => "Value::CompileError",
                        Value::RuntimeError => "Value::RuntimeError",
                    }
                )
            }
        }
        impl std::cmp::PartialEq<str> for Value {
            fn eq(&self, rhs: &str) -> bool {
                if &self.to_string() == rhs {
                    return true;
                };
                if &self.to_string() == rhs {
                    return true;
                }
                false
            }
        }
        impl std::cmp::PartialEq<String> for Value {
            fn eq(&self, rhs: &String) -> bool {
                if &self.to_string() == rhs {
                    return true;
                };
                if &self.to_string() == rhs {
                    return true;
                }
                false
            }
        }

    };

    folder_path!()
        .join("value.actual.rs")
        .write_unchecked(format_code(&stream).as_bytes());
    folder_path!()
        .join("value.expected.rs")
        .write_unchecked(format_code(&expected).as_bytes());

    assert_streams_equal!(stream, expected);

    Ok(())
}
