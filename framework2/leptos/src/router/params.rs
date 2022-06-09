use std::collections::HashMap;

use super::RouterError;

pub trait Params where Self: Sized {
	fn from_map(map: &HashMap<String, String>) -> Result<Self, RouterError>;
}

impl Params for () {
    fn from_map(_map: &HashMap<String, String>)-> Result<Self, RouterError> {
		Ok(())
	}
}