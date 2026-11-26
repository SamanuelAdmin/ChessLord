use dotenv;
use std::{env, ptr, error, fmt};

use crate::domain::{
    EResult,
    DotEnvNotLoaded,
    DotEnvVarNotFound,
    IncorrectValue,
    PrivateField,
    AccessLevel
};


fn _get_env_var<T>(var_name: &str, default_value: Option<T>) -> EResult<T> {
    return match env::var(var_name) {
        Ok(var) => {
            var.parse().ok_or(
                Err(IncorrectValue::new(var_name.to_string()))
            )
        },
        Err(env::VarError::NotPresent) => {
            default_value.unwrap_or(
                Err(DotEnvVarNotFound::new(var_name.to_string()))
            )
        },
        Err(env::VarError::NotUnicode(raw)) => {
            Err(IncorrectValue::new(var_name))
        }
    }
}




pub struct SettingsField<T, const A: AccessLevel> {
    value: T,
    access: AccessLevel
}

impl<T, const A: AccessLevel> SettingsField<T, A> {
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            access: A
        }
    }

    pub fn value(&self) -> EResult<T> {
        if self.access == AccessLevel::Public {
            return Ok(self.value)
        }

        Err(PrivateField::new())
    }
}

impl<T, const A: AccessLevel> fmt::Display for SettingsField<T, { A }> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.access == AccessLevel::Public {
            return write!(f, "{}", self.value);
        } 

        return write!(f, "<PRIVATE>");
    }
}


pub struct Settings {
    tbot_token: SettingsField::<String, { AccessLevel::Private }>,
    default_frequency: SettingsField::<u32, { AccessLevel::Public }>,
    default_puzzle_rating: SettingsField::<u32, { AccessLevel::Public }>,
}



impl Settings {
    pub fn new(env_file: Option<&str>) -> EResult<Self> {
        let env_path = env_file.unwrap_or(".env");

        println!("Loading env variables from {}", env_path);
        if let Err(_) = dotenv::from_filename(env_path) {
            return DotEnvNotLoaded::new(env_path.to_string());
        }

        let tbot_token = _get_env_var::<String>("BOT_TOKEN", None)?;
        let d_frequency = _get_env_var::<u32>("D_FREQUENCY", Some(4u32))?;
        let dpr = _get_env_var::<u32>("DPR", Some(100u32))?;

        Ok(Self {
            tbot_token: SettingsField::<String, { AccessLevel::Private }>::new(
                tbot_token),
            default_frequency: SettingsField::<u32, { AccessLevel::Public }>::new(
                d_frequency),
            default_puzzle_rating: SettingsField::<u32, { AccessLevel::Public }>::new(
                dpr)
        })
    }
}
