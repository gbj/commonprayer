use leptos2::*;
use web_sys::RequestInit;
use wasm_bindgen::{JsCast, JsValue};

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

#[derive(Clone, Debug, PartialEq)]
pub enum FormCmd {
    Get {
        action: String,
		params: String
    },
    Post {
        action: String,
        data: wasm_bindgen::JsValue
    },
    PatchDom(Node),
}

#[async_trait(?Send)]
impl State for Form {
    type Msg = FormMsg;
    type Cmd = FormCmd;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        match (&self.state, msg) {
            (FormState::Idle, FormMsg::Submit(ev)) => {
				self.state = FormState::Loading;
				ev.prevent_default();
				let form = ev.target().unwrap().unchecked_into::<web_sys::HtmlFormElement>();
                let form_data = web_sys::FormData::new_with_form(&form).unwrap();
                

				if form.method() == "get" {
                    let search_params = web_sys::UrlSearchParams::new_with_str_sequence_sequence(&form_data).unwrap();
					Some(Self::Cmd::Get {
						action: form.action(),
						params: search_params.to_string().as_string().unwrap()
					})
				} else {
                    let data = if self.enctype == "application/x-www-form-urlencoded" {
                        let search_params = web_sys::UrlSearchParams::new_with_str_sequence_sequence(&form_data).unwrap();
                        search_params.to_string().into()
                    } else {
                        form_data.into()
                    };
					Some(Self::Cmd::Post {
						action: form.action(),
						data
					})
				}
            }
            (FormState::Loading, FormMsg::Loaded(new_node)) => {
                self.state = FormState::Idle;
                Some(Self::Cmd::PatchDom(new_node))
            }
            _ => None,
        }
    }

    async fn cmd(
        cmd: Self::Cmd,
        host: web_sys::HtmlElement,
        link: StateLink<Self>,
    ) -> Option<Self::Msg> {
        match cmd {
            FormCmd::Get {
                action,
                params
            } => {
				let mut action = action.split('?');
				let action = action.next().unwrap();

                // load new DOM state from server
                // TODO real error handling here
                let new_node = reqwasm::http::Request::get(&format!("{}get.json?{}", action, params))
                    .send()
                    .await
                    .unwrap()
                    .json::<Node>()
                    .await
                    .unwrap();

                // push the new URL; "does" nothing (doesn't reload from server)
                // TODO error handling
                window().history().unwrap().push_state_with_url(&JsValue::NULL, "", Some(&format!("?{}", params)));

                // this will patch the DOM rather than refreshing the page
				Some(Self::Msg::Loaded(new_node))
            }
            FormCmd::Post { action, data } => {
                // TODO real error handling here
                let new_node = reqwasm::http::Request::get(&format!("{}post.json", action))
                    .body(data)
                    .send()
                    .await
                    .unwrap()
                    .json::<Node>()
                    .await
                    .unwrap();
				Some(Self::Msg::Loaded(new_node))
            },
            FormCmd::PatchDom(mut new_vdom) => {
                // TODO once we have nested views, obviously we won't patch the whole DOM
                let node_to_patch = document().query_selector("html").unwrap().unwrap();
                patch_dom(&node_to_patch, &mut new_vdom);
                None
            }
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
