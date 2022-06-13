use std::{collections::HashMap, str::FromStr};

use super::RouterError;

pub trait Params
where
    Self: Sized,
{
    fn from_map(map: &HashMap<String, String>) -> Result<Self, RouterError>;
}

impl Params for () {
    fn from_map(_map: &HashMap<String, String>) -> Result<Self, RouterError> {
        Ok(())
    }
}

pub trait IntoParam
where
    Self: Sized,
{
    fn into_param(value: Option<&str>, name: &str) -> Result<Self, RouterError>;
}

impl<T> IntoParam for Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    fn into_param(value: Option<&str>, _name: &str) -> Result<Self, RouterError> {
        match value {
            None => Ok(None),
            Some(value) => match T::from_str(value) {
                Ok(value) => Ok(Some(value)),
                Err(e) => Err(RouterError::Params(Box::new(e))),
            },
        }
    }
}

auto trait NotOption {}
impl<T> !NotOption for Option<T> {}

impl<T> IntoParam for T
where
    T: FromStr + NotOption,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    fn into_param(value: Option<&str>, name: &str) -> Result<Self, RouterError> {
        let value = value.ok_or_else(|| RouterError::MissingParam(name.to_string()))?;
        Self::from_str(value).map_err(|e| RouterError::Params(Box::new(e)))
    }
}
