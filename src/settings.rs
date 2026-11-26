use dotenv;
use std::{env, ptr, error, fmt};

use domain::{
    EResult,
    DotEnvNotLoaded,
    DotEnvVarNotFound,
    IncorrectValue,
    PrivateField,
    AccessLevel
};



pub struct SettingsField<T, const A: AccessLevel> {
    value: T,
    access: AccessLevel
}

impl<T, const A: AccessLevel> SettingsField<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            access: A
        }
    }

    pub fn value(&self) -> UResult<T> {
        if (self.access == AccessLevel::Public) {
            return self.value
        }

        PrivateField::new()
    }
}

impl fmt::Display for SettingsField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.access == AccessLevel::Public {
            return write!(f, "{}", self.value);
        } 

        return write!(f, "<PRIVATE>");
    }
}


pub struct Settings {
    tbot_token: SettingsField::<String, { AccessLevel::Private }>,
    default_frequency: SettingsField::<u8, { AccessLevel::Public }>,
    default_puzzle_rating: SettingsField::<u16, { AccessLevel::Public }>,
}



impl Settings {
    fn _parse_var<T> for T where T: From<String> (var_name: &str, default_value: *T) -> EResult<T> {
        return match env::var(var_name) {
            Ok(var) => T {
                var.parse().ok_or(
                    IncorrectValue::new(var_name)
                )
            }
            Err(env::VarError::NotPresent) => {
                default_value.is_null()
                ? DotEnvVarNotFound::new(var_name.to_string())
                : default_value
            }
            Err(env::VarError::NotUnicode(raw)) => {
                IncorrectValue::new(var_name)
            }
        }
    }

    pub fn new(env_path: *const str) -> EResult<Self> {
        if env_path.is_null() {
            *env_path = ".env";
        }

        println!("Loading env variables from {}", env_path);
        if let Err(_) = dotenv::from_filename(*env_path) {
            return DotEnvNotLoaded::new(env_path);
        }

        let tbot_token = _parse_var::<String>("BOT_TOKEN", ptr::null())?;
        let d_frequency = _parse_var::<u8>("D_FREQUENCY", 4)?;
        let dpr = _parse_var::<u16>("DPR", 100)?;

        Self {
            tbot_token: SettingsField::<String, { AccessLevel::Private }>::new(
                tbot_token),
            default_frequency: SettingsField::<String, { AccessLevel::Private }>::new(
                d_frequency),
            default_puzzle_rating: SettingsField::<String, { AccessLevel::Private }>::new(
                dpr)
        }
    }
}
