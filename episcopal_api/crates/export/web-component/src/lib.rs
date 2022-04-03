use custom_elements::inject_style;
use custom_elements::CustomElement;
use liturgy::Document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Node};
use website::components::DocumentController;

struct ComponentWrapper {
    locale: String,
    node: Option<Node>,
}

impl ComponentWrapper {
    fn new() -> Self {
        Self {
            locale: String::from("en"),
            node: None,
        }
    }
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(
            this,
            include_str!("../../../../../website/website/static/document.css"),
        );
        let node = leptos::create_comment_node(&leptos::document());
        this.append_child(&node);
        self.node = Some(node);
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["locale", "doc", "href"]
    }

    fn attribute_changed_callback(
        &mut self,
        this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name.as_str() == "locale" {
            if let Some(value) = &new_value {
                self.locale = value.to_owned();
            }
        }
        if name.as_str() == "doc" {
            if let Some(value) = &new_value {
                match serde_json::from_str::<Document>(value) {
                    Ok(doc) => {
                        let view = DocumentController::new(doc).view(&self.locale);
                        let node = view.client_side_render();
                        if let Some(old_node) = &self.node {
                            leptos::replace_with(old_node.unchecked_ref(), &node);
                            self.node = Some(node);
                        }
                    }
                    Err(e) => leptos::log(&format!("error\n\n{:#?}", e)),
                }
            }
        };
        if name.as_str() == "href" {
            if let Some(href) = &new_value {
                wasm_bindgen_futures::spawn_local({
                    let this = this.clone();
                    let href = href.clone();
                    async move {
                        if let Ok(resp) = reqwasm::http::Request::get(&href).send().await {
                            if let Ok(data) = resp.json::<Document>().await {
                                if let Ok(json) = serde_json::to_string(&data) {
                                    this.set_attribute("doc", &json);
                                }
                            }
                        }
                    }
                });
            }
        }
    }
}

impl Default for ComponentWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapper::define("commonprayer-doc");
}
