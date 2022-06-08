use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::{StateLink, State, EventEmitter, CustomEvent};

pub struct Cmd<T>(Box<dyn FnOnce(
	&web_sys::HtmlElement,
    &StateLink<T>
)>) where T: State;

impl<T> Cmd<T> where T: State {
	pub fn new(cmd: impl FnOnce(&web_sys::HtmlElement, &StateLink<T>) + 'static) -> Self {
		Self(Box::new(cmd))
	}

	pub fn event<E>(event: CustomEvent<E>) -> Self where E: Serialize + DeserializeOwned + 'static {
		Cmd::new(move |el: &web_sys::HtmlElement, _link: &StateLink<T>| {
			let emitter = EventEmitter::new(el);
			emitter.emit(event)
		})
	}
}

impl<T> Cmd<T> where T: State {
	pub fn call(self, host: &web_sys::HtmlElement, link: &StateLink<T>) {
		(self.0)(host, link)
	}
}