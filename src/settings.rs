use dotenv;
use std::{env, ptr, error};

use domain::{
    EResult,
    DotEnvNotLoaded,
    DotEnvVarNotFound,
    IncorrectValue,
    AccessLevel
};



pub struct SettingsField<T> {
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
    }
}

pub struct Settings {
    tbot_token: String,
    default_frequency: u8,
    default_puzzle_rating: u16
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (self.access == AccessLevel::Public) {
            return write!(f, "{}", this.value);
        } 
    }
}


impl Setting {
    pub fn new(env_path: *str) -> EResult<Self> {
        if env_path.is_null() {
            *env_path = ".env";
        }

        println!("Loading env variables from {}", env_path);
        dotenv::from_filename(*env_path).ok_or();

        Self {
            tbot_token: Self._parse_var::<String>("BOT_TOKEN", ptr::null()),
            default_frequency: Self._parse_var::<u8>("D_FREQUENCY", 4),
            default_puzzle_rating: Self._parse_var::<u16>("DPR", 100)
        }
    }

    fn _parse_var<T> for T where T: From<String> (var_name: &str, default_value: *T) -> EResult<T> {
        return match env::var(var_name) {
            Ok(var) => T {
                var.parse().ok_or(
                    IncorrectValue::new()
                )
            }
            Err(env::VarError::NotPresent) => {
                default_value.is_null()
                ? DotEnvVarNotFound::new(var_name.to_string())
                : default_value
            }
            Err(env::VarError::NotUnicode(raw)) => {
                IncorrectValue::new()
            }
        }
    }
}
