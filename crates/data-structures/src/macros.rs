#[macro_export]
macro_rules! list {
    () => {{
        $crate::cons::list([])
    }};
    ($( $arg:expr ),* ) => {{
        $crate::cons::list([
            $(
                $arg
            ),*
        ])
    }};
}

#[macro_export]
macro_rules! append {
    () => {{
        $crate::cons::append([])
    }};
    ($( $arg:expr ),* ) => {{
        $crate::cons::append([
            $(
                $arg
            ),*
        ])
    }};
}
