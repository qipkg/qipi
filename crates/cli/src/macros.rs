#[macro_export]
macro_rules! register_commands {
    ($($name:ident),*) => {
        $(
            pub mod $name;
            paste::paste! {
                pub use $name::[<$name:camel Command>];
            }
        )*
    }
}
