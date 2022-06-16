use leptos2::*;
use wasm_bindgen::{prelude::*, JsCast, JsValue};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, WebComponent)]
pub struct Form {
    pub action: String,
    pub enctype: String,
    pub method: String,
    state: FormState,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum FormState {
    #[default]
    Idle,
    Loading,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormMsg {
    Submit(web_sys::Event),
    Loaded(Node),
}

impl State for Form {
    type Msg = FormMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match (&self.state, msg) {
            (FormState::Idle, FormMsg::Submit(ev)) => {
                self.state = FormState::Loading;
                ev.prevent_default();
                let form = ev
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlFormElement>();
                let form_data = web_sys::FormData::new_with_form(&form).unwrap();

                if form.method() == "get" {
                    let search_params =
                        web_sys::UrlSearchParams::new_with_str_sequence_sequence(&form_data)
                            .unwrap();
                    Some(self.get(
                        form.action(),
                        search_params.to_string().as_string().unwrap(),
                    ))
                } else {
                    let data = if self.enctype == "application/x-www-form-urlencoded" {
                        let search_params =
                            web_sys::UrlSearchParams::new_with_str_sequence_sequence(&form_data)
                                .unwrap();
                        search_params.to_string().into()
                    } else {
                        form_data.into()
                    };
                    Some(self.post(form.action(), data))
                }
            }
            (FormState::Loading, FormMsg::Loaded(new_vdom)) => {
                self.state = FormState::Idle;
                Some(self.patch_dom(new_vdom))
            }
            _ => None,
        }
    }
}

impl Component for Form {
    fn view(&self) -> Host {
        view! {
                    <Host
                        on:submit=|ev: web_sys::Event| FormMsg::Submit(ev)
                    >
        /*                 <form
                            action={if self.action.is_empty() { None } else { Some(self.action.clone()) }}
                            enctype={if self.enctype.is_empty() { None } else { Some(self.enctype.clone()) }}
                            method={if self.method.is_empty() { None } else { Some(self.method.clone()) }}
                        >

                        </form> */
                        <slot></slot>
                    </Host>
                }
    }
}

impl Form {
    fn get(&self, action: String, params: String) -> Cmd<Self> {
        let mut action = action.split('?');
        let action = action.next().unwrap();
        let action = action.to_string();

        Cmd::new(move |_, link| {
            let link = link.clone();
            spawn_local(async move {
                // load new DOM state from server
                // TODO real error handling here
                let new_node =
                    reqwasm::http::Request::get(&format!("{}get.json?{}", action, params))
                        .send()
                        .await
                        .unwrap()
                        .json::<Node>()
                        .await
                        .unwrap();

                // push the new URL; "does" nothing (doesn't reload from server)
                // TODO error handling
                window().history().unwrap().push_state_with_url(
                    &JsValue::NULL,
                    "",
                    Some(&format!("?{}", params)),
                );

                // this will patch the DOM rather than refreshing the page
                link.send(&FormMsg::Loaded(new_node));
            });
        })
    }

    fn post(&self, action: String, data: JsValue) -> Cmd<Self> {
        let mut action = action.split('?');
        let action = action.next().unwrap();
        let action = action.to_string();

        Cmd::new(move |_, link| {
            let link = link.clone();
            spawn_local(async move {
                // load new DOM state from server
                // TODO real error handling here
                let new_node = reqwasm::http::Request::get(&format!("{}post.json", action))
                    .body(data)
                    .send()
                    .await
                    .unwrap()
                    .json::<Node>()
                    .await
                    .unwrap();

                // TODO routing
                // this will patch the DOM rather than refreshing the page
                link.send(&FormMsg::Loaded(new_node));
            });
        })
    }

    fn patch_dom(&self, mut new_vdom: Node) -> Cmd<Self> {
        let node_to_patch = document().query_selector("html").unwrap().unwrap();

        Cmd::new(move |_, _| {
            patch_dom(&node_to_patch, &mut new_vdom);
            // set up listeners to hydrate any custom elements we've now added
            observe_custom_elements();
        })
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = observeCustomElements)]
    fn observe_custom_elements();
}
