use std::{fmt, error};

pub type EResult<T> = std::result::Result<T, Box<dyn error::Error>>;


pub trait CustomError {}
pub trait ErrorWithMessage {
    fn get_message() -> String;
}

#[derive(error::Error, CustomError)]
pub struct DotEnvNotLoaded;
#[derive(error::Error, CustomError)]
pub struct IncorrectValue;
#[derive(error::Error, CustomError)]
pub struct DotEnvVarNotFound;


impl<const M: 'const &str, U: CustomError> ErrorWithMessage for U {
    #[inline(always)]
    fn get_message() -> String {
        M
    }
}

impl<U: CustomError> error::Error for U {}
impl<U: CustomError> fmt::Display for U {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, &M)
    }
}


// USAGE:
// IncorrectValue::<"This is incorrect">
