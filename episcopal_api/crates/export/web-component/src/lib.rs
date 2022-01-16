use custom_elements::CustomElement;
use liturgy::Document;
use sauron::{prelude::*, web_sys::HtmlElement};
use web::{Msg, Viewer};

struct ComponentWrapper {
    program: Option<Program<Viewer, Msg>>,
}

impl ComponentWrapper {
    fn new() -> Self {
        Self { program: None }
    }
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        let program = Program::append_to_mount(Viewer::new(), this);
        self.program = Some(program);
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["doc"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name.as_str() == "doc" {
            if let Some(value) = new_value {
                if let Ok(doc) = serde_json::from_str::<Document>(&value) {
                    if let Some(program) = &self.program {
                        program.dispatch(Msg::SetDocument(doc));
                    }
                }
            }
        };
    }
}

impl Default for ComponentWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapper::define("eapi-doc");
}
