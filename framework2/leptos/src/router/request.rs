use std::collections::HashMap;

use thiserror::Error;

use crate::{Params, RouterError};
pub trait Request {
    fn path(&self) -> &str;

    fn query_string(&self) -> &str;

    fn method(&self) -> http::Method;

    fn headers(&self) -> http::HeaderMap;

    fn body(&self) -> Option<RequestBody>;
}

pub struct RequestBody<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a [u8]> for RequestBody<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl<'a> RequestBody<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes
    }

    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.bytes)
    }

    pub fn as_form_data<T: Params>(&self) -> Result<T, RouterError> {
        let form_data = form_urlencoded::parse(self.bytes)
            .into_owned()
            .collect::<HashMap<_, _>>();
        T::from_map(&form_data)
    }
}
pub trait Cookies {
    fn cookies(&self) -> Box<dyn Iterator<Item = Result<cookie::Cookie, CookieError>> + '_>;
}

impl Cookies for http::HeaderMap {
    fn cookies(&self) -> Box<dyn Iterator<Item = Result<cookie::Cookie, CookieError>> + '_> {
        Box::new(self.get_all("Cookie").iter().map(|value| {
            value
                .to_str()
                .map_err(|_| CookieError::NonAsciiValue)
                .and_then(|value| cookie::Cookie::parse(value).map_err(CookieError::Parse))
        }))
    }
}

#[derive(Error, Debug, Clone)]
pub enum CookieError {
    #[error("Cookie header value contained non-ASCII characters")]
    NonAsciiValue,
    #[error("could not parse Cookie header as a cookie")]
    Parse(cookie::ParseError),
}
