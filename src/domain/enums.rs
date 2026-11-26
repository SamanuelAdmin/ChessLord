use std::convert::From;
use std::marker::ConstParamTy;


#[derive(ConstParamTy, PartialEq, Eq)]
pub enum AccessLevel {
    Public,
    Private
}

impl From<bool> for AccessLevel {
    fn from(item: bool) -> AccessLevel {
        if item {
            return AccessLevel::Public;
        }

        AccessLevel::Private
    }
}
