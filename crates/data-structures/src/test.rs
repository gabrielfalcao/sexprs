#[macro_export]
macro_rules! assert_display_equal {
    ($left:expr, $right:literal) => {{
        k9::assert_equal!(format!("{}", $left), $right);
    }};
    ($left:expr, $right:expr) => {{
        k9::assert_equal!(format!("{}", $left), format!("{}", $right));
    }};
}

#[macro_export]
macro_rules! assert_debug_equal {
    ($left:expr, $right:literal) => {{
        k9::assert_equal!(format!("{:#?}", $left), $right);
    }};
    ($left:expr, $right:expr) => {{
        k9::assert_equal!(format!("{:#?}", $left), format!("{:#?}", $right));
    }};
}
