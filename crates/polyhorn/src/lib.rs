pub use polyhorn_macros::*;

#[cfg(target_os = "ios")]
pub use polyhorn_ios::*;

#[macro_export]
macro_rules! with {
    (($($name:ident),*), $($tt:tt)*) => {{
        $(
            let $name = $name.clone();
        )*
        move $($tt)*
    }};
}
