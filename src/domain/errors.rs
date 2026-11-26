use std::{fmt, error};

pub type EResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[macro_export]
macro_rules! define_error {
    ( $name:ident, $message:expr $(, $var_name:ident : $var_type:ty)* $(,)? ) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                $var_name: $var_type,
            )*
        }

        impl $name {
            pub fn new( $( $var_name: $var_type ),* ) -> Self {
                Self {
                    $( $var_name: $var_name ),*
                }
            }
        }

        impl error::Error for $name {}
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f, $message $(, self.$var_name )* 
                )
            }
        }
    };
}


define_error!(IncorrectValue, "Incorrect value of variable {}", var_name: String);
define_error!(DotEnvVarNotFound, "Cannot find environment variable {}", var_name: String);
define_error!(DotEnvNotLoaded, "Cannot load env file with path: {}", path: String);
define_error!(PrivateField, "Cannot access private field");
