use std::{ftm, error};

pub type EResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct DotEnvNotLoaded;
impl error::Error for DotEnvNotLoaded {}

impl fmt::Display for DotEnvNotLoaded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot load env variables from file.")
    }
}


pub struct IncorrectValue;
impl error::Error for IncorrectValue {}

impl fmt::Display for DotEnvNotLoaded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect value!")
    }
}


pub struct DotEnvVarNotFound {
    var_name: String
};

impl DotEnvVarNotFound {
    pub fn new(var_name: String) -> Self {
        Self {
            var_name: var_name
        }
    }
}

impl error::Error for DotEnvVarNotFound {}
impl fmt::Display for DotEnvNotLoaded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot find env variable {}", self.var_name)
    }
}
