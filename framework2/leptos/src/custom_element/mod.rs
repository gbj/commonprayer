use std::{cell::RefCell, rc::Rc};

use futures::{channel::mpsc::unbounded, StreamExt};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue, UnwrapThrowExt,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use crate::{
    self as leptos2, debug_warn,
    link::{Link, StateLink},
    patch_host, request_animation_frame, view, AsAny, Attribute, Component, Host, Node, State,
    StateSender,
};

pub trait WebComponent
where
    Self: State + Component + Default,
    Self::Msg: Clone,
{
    fn tag() -> &'static str;

    fn attributes() -> &'static [&'static str];

    fn properties() -> &'static [&'static str];

    fn set_attribute(&mut self, attr_name: String, new_value: Option<String>);

    fn set_property(&mut self, attr_name: String, new_value: JsValue);

    fn state_to_attributes(&self) -> Vec<Attribute>;

    fn define() {
        // constructor function will be called for each new instance of the component
        let constructor = Closure::wrap(Box::new(move |this: HtmlElement| {
            // start with the default
            let mut initial_state = Self::default();

            // load initial state from attributes and props
            for attr in Self::attributes() {
                if let Some(value) = this.get_attribute(attr) {
                    initial_state.set_attribute(attr.to_string(), Some(value));
                }
            }

            // it's entirely possible that the web component is upgrading after the prop deserializer
            // has already set some of the properties on the element, so we'll want to check the element
            // for each property
            for prop in Self::properties() {
                if let Ok(value) = js_sys::Reflect::get(&this, &JsValue::from_str(prop)) {
                    if !value.is_undefined() {
                        initial_state.set_property(prop.to_string(), value);
                    }
                }
            }

            // set up event cycle
            let (tx, mut rx) = unbounded::<Option<Self::Msg>>();

            let link: StateLink<Self> = StateLink::from(tx.clone());
            let state = Rc::new(RefCell::new(initial_state));

            let inject_children = Closure::once({
                let state = state.clone();
                let state_link = link.clone();
                move |host: web_sys::HtmlElement, hydrate: bool| {
                    let mut current_view = state.borrow().view();
                    let link = Link::from(state_link.clone());
                    if hydrate {
                        current_view.hydrate(&host, &host.shadow_root().unwrap(), &link);
                    } else {
                        current_view.mount(&host, &host.shadow_root().unwrap(), &link);
                    }

                    // start nested state machines
                    for nested_state in state.borrow_mut().nested_states() {
                        nested_state.set_host_and_parent(
                            Rc::new(StateSender::<Self>::new(tx.clone())),
                            &host,
                        );
                        nested_state.run();
                    }

                    // check for an initial command
                    let init = state.borrow().init();
                    if let Some(cmd) = init {
                        cmd.call(&host, &state_link);
                    }

                    // listen for messages
                    spawn_local({
                        async move {
                            while let Some(msg) = rx.next().await {
                                // Some(T::Msg) means a message was sent; we need to update the state and then update the view
                                // None means that an attribute was changed, so state has already been changed; we only update the view
                                let should_render = if let Some(msg) = msg {
                                    let should_render = state.borrow().should_render(&msg);

                                    // [UPDATE] apply the change to
                                    // 1) mutate the state and
                                    // 2) get any Cmd output from the update fn
                                    let cmd = state.borrow_mut().update(msg);

                                    // handle any async commands here, so the Cmd type doesn't leak
                                    // out of the component up to the ComponentInstance
                                    if let Some(cmd) = cmd {
                                        cmd.call(&host, &state_link);
                                    }

                                    should_render
                                }
                                // if an attribute or prop change, always render
                                else {
                                    true
                                };

                                if should_render {
                                    // apply updated state to view

                                    // generate new view tree
                                    let mut new_view = state.borrow().view();

                                    // reconcile the DOM to the new tree
                                    patch_host(
                                        &host,
                                        &host.shadow_root().unwrap(),
                                        &current_view,
                                        &mut new_view,
                                        &link,
                                    );

                                    // store the new tree for future diffing
                                    current_view = new_view;
                                }
                            }
                        }
                    })
                }
            });
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_injectChildren"),
                &inject_children.into_js_value(),
            )
            .unwrap_throw();

            // attributeChangedCallback
            let attribute_changed = Closure::wrap(Box::new({
                let state = state.clone();
                let link = link.clone();
                move |_el, name: String, _old_value, new_value: Option<String>| {
                    let state = state.clone();
                    let link = link.clone();
                    request_animation_frame(move || {
                        state
                            .borrow_mut()
                            .set_attribute(name.clone(), new_value.clone());
                        if let Err(e) = link.attributes_changed() {
                            debug_warn(&format!("[WebComponent::attribute_changed] {}", e))
                        }
                    })
                }
            })
                as Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_attributeChangedCallback"),
                &attribute_changed.into_js_value(),
            )
            .unwrap_throw();

            // callback when properties are changed
            let set_property = Closure::wrap(Box::new(move |name: String, new_value: JsValue| {
                let state = state.clone();
                let link = link.clone();
                state.borrow_mut().set_property(name, new_value);
                if let Err(e) = link.attributes_changed() {
                    debug_warn(&format!("[WebComponent::set_property] {}", e))
                }
            }) as Box<dyn FnMut(String, JsValue)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_setProperty"),
                &set_property.into_js_value(),
            )
            .unwrap_throw();
        }) as Box<dyn FnMut(HtmlElement)>);

        // observedAttributes is static and needs to be known when the class is defined
        let attributes = Self::attributes();
        let observed_attributes = JsValue::from(
            attributes
                .iter()
                .map(|attr| JsValue::from_str(attr))
                .collect::<js_sys::Array>(),
        );

        let properties = Self::properties();
        let observed_properties = JsValue::from(
            properties
                .iter()
                .map(|attr| JsValue::from_str(attr))
                .collect::<js_sys::Array>(),
        );

        // call out to JS to define the Custom Element
        let (super_tag, super_constructor) = Self::superclass();
        make_custom_element(
            super_constructor,
            Self::tag(),
            Self::shadow(),
            constructor.into_js_value(),
            observed_attributes,
            observed_properties,
            super_tag,
        );
    }

    fn superclass() -> (Option<&'static str>, &'static js_sys::Function) {
        (None, &HTML_ELEMENT_CONSTRUCTOR)
    }

    fn shadow() -> bool {
        true
    }
}

pub trait DeclarativeWebApi
where
    Self: Default + 'static,
{
    fn tag() -> &'static str;

    fn attributes() -> &'static [&'static str];

    fn properties() -> &'static [&'static str];

    fn set_attribute(&mut self, attr_name: String, new_value: Option<String>);

    fn set_property(&mut self, attr_name: String, new_value: JsValue);

    fn state_to_attributes(&self) -> Vec<Attribute>;

    fn superclass() -> (Option<&'static str>, &'static js_sys::Function) {
        (None, &HTML_ELEMENT_CONSTRUCTOR)
    }

    fn shadow() -> bool {
        true
    }

    fn view(&self) -> Host {
        view! { <Host></Host> }
    }

    fn init(&mut self, host: HtmlElement) {}

    fn define() {
        // constructor function will be called for each new instance of the component
        let constructor = Closure::wrap(Box::new(move |this: HtmlElement| {
            // start with the default
            let mut initial_state = Self::default();
            initial_state.init(this.clone());

            // load initial state from attributes and props
            for attr in Self::attributes() {
                if let Some(value) = this.get_attribute(attr) {
                    initial_state.set_attribute(attr.to_string(), Some(value));
                }
            }

            for prop in Self::properties() {
                if let Ok(value) = js_sys::Reflect::get(&this, &JsValue::from_str(prop)) {
                    initial_state.set_property(prop.to_string(), value);
                }
            }

            let state = Rc::new(RefCell::new(initial_state));

            // attributeChangedCallback
            let attribute_changed = Closure::wrap(Box::new({
                let state = state.clone();
                move |_el, name: String, _old_value, new_value: Option<String>| {
                    state.borrow_mut().set_attribute(name, new_value);
                }
            })
                as Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_attributeChangedCallback"),
                &attribute_changed.into_js_value(),
            )
            .unwrap_throw();

            // callback when properties are changed
            let set_property = Closure::wrap(Box::new({
                let state = state.clone();
                move |name: String, new_value: JsValue| {
                    let state = state.clone();
                    state.borrow_mut().set_property(name, new_value);
                }
            }) as Box<dyn FnMut(String, JsValue)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_setProperty"),
                &set_property.into_js_value(),
            )
            .unwrap_throw();
        }) as Box<dyn Fn(HtmlElement)>);

        // observedAttributes is static and needs to be known when the class is defined
        let attributes = Self::attributes();
        let observed_attributes = JsValue::from(
            attributes
                .iter()
                .map(|attr| JsValue::from_str(attr))
                .collect::<js_sys::Array>(),
        );

        let properties = Self::properties();
        let observed_properties = JsValue::from(
            properties
                .iter()
                .map(|attr| JsValue::from_str(attr))
                .collect::<js_sys::Array>(),
        );

        // call out to JS to define the Custom Element
        let (super_tag, super_constructor) = Self::superclass();
        make_custom_element(
            super_constructor,
            Self::tag(),
            Self::shadow(),
            constructor.into_js_value(),
            observed_attributes,
            observed_properties,
            super_tag,
        );
    }
}

// JavaScript shim
#[wasm_bindgen(module = "/src/custom_element/make_custom_element.js")]
extern "C" {
    fn make_custom_element(
        superclass: &js_sys::Function,
        tag_name: &str,
        shadow: bool,
        constructor: JsValue,
        observed_attributes: JsValue,
        observed_properties: JsValue,
        superclass_tag: Option<&str>,
    );
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = HTMLElement, js_namespace = window)]
    pub static HTML_ELEMENT_CONSTRUCTOR: js_sys::Function;
}
