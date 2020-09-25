#[macro_export]
macro_rules! style {
    ($($name:ident : $value:expr ;)*) => {
        $crate::Style {
            $(
                $name: ($value).into(),
            )*
            .. Default::default()
        }
    };
}

#[macro_export]
macro_rules! text_style {
    ($($name:ident : $value:expr ;)*) => {
        $crate::TextStyle {
            $(
                $name: ($value).into(),
            )*
            .. Default::default()
        }
    };
}
