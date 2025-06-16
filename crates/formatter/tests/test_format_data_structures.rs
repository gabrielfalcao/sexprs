#![allow(unused)]
use k9::assert_equal;
use sexprs_formatter::{format_code_naive, Result};

#[test]
fn test_format_enum_unnamed_fields() -> Result<()> {
    #[derive(Debug)]
    struct Info<'a> {
        id: u8,
        name: &'a str,
    }
    #[derive(Debug)]
    enum Value<'a> {
        Info(Info<'a>),
        String(String),
        Integer(i8),
    }
    impl Value<'_> {
        pub fn to_string(&self) -> String {
            match self {
                Value::Info(val) => format!("{:#?}", val),
                Value::String(val) => format!("{:#?}", val),
                Value::Integer(val) => format!("{:#?}", val),
            }
        }
    }

    let value = Value::Info(Info { id: 1, name: "G" });

    assert_equal!(
        format_code_naive(format!("{:#?}", &value)),
        r#"
Info(
    Info {
        id: 1,
        name: "G",
    },
)
"#
        .trim()
    );
    Ok(())
}

#[test]
fn test_format_struct() {
    assert_equal!(
        format_code_naive(
            r#"
TypeName {
    field_name: None,
    segments: [
        "NodeInfo"
    ],
    lifetimes: [],
    generic_type_names: [],
}
"#
            .trim()
        ),
        r#"
TypeName {
    field_name: None,
    segments: [
        "NodeInfo"
    ],
    lifetimes: [],
    generic_type_names: [],
}
"#
        .trim()
    );
}

#[test]
fn test_format_reference_type() -> Result<()> {
    assert_equal!(format_code_naive("& str"), "&str");
    assert_equal!(format_code_naive("& 'a str"), "&'a str");
    Ok(())
}
