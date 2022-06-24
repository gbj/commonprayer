use leptos2::*;
use wasm_bindgen::JsCast;

use crate::UserInfo;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct Auth {
    #[prop]
    pub user: Option<UserInfo>,
    pub logout_open: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]

pub enum AuthMsg {
    Logout,
    OpenModal,
    SetUser(Option<UserInfo>),
}

impl State for Auth {
    type Msg = AuthMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            Self::Msg::SetUser(user) => {
                // set a cookie
                // note that this is inherently untrustable; client can set this to whatever
                // however, we can do any verification we need on the server side
                // this is just a clue to the server to simply pre-render the photoURL
                let cookie =
                    cookie::Cookie::build("untrusted-user", serde_json::to_string(&user).unwrap())
                        .path("/")
                        .finish();
                document()
                    .unchecked_into::<web_sys::HtmlDocument>()
                    .set_cookie(&cookie.to_string());

                // set internal user
                self.user = user;
                None
            }
            Self::Msg::OpenModal => {
                if self.user.is_none() {
                    Some(Cmd::event(CustomEvent::<()>::new("openmodal").bubbles()))
                } else {
                    self.logout_open = !self.logout_open;
                    None
                }
            }
            Self::Msg::Logout => {
                self.user = None;
                Some(Cmd::<Self>::event(
                    CustomEvent::<()>::new("logout").bubbles(),
                ))
            }
        }
    }
}

impl Component for Auth {
    fn view(&self) -> Host {
        view! {
            <Host
                window:user=|ev: Event| {
                    let ev: CustomEvent<Option<UserInfo>> = ev.into();
                    Self::Msg::SetUser(ev.detail.flatten())
                }
            >
                <style>{include_str!("auth.css")}</style>
                <div>
                    <button
                        class="auth-button"
                        aria-label={t!("auth.title")}
                        on:click=|_| Self::Msg::OpenModal
                    >
                        <img
                            class:hidden={self.user.is_none()}
                            src={self.user.as_ref().and_then(|user| user.photo_url.clone()).unwrap_or_default()}
                            alt={self.user.as_ref().and_then(|user| user.display_name.clone()).unwrap_or_default()}
                        />
                        <div class:hidden={self.user.is_some()}>
                            <svg viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
                                <path fill="#c5dbff" d="M0 0h512v512H0z"/>
                                <path d="M 256 322.591 C 312.094 322.591 357.99 276.696 357.99 220.601 C 357.99 164.507 312.094 118.612 256 118.612 C 199.906 118.612 154.011 164.507 154.011 220.601 C 154.011 276.696 199.906 322.591 256 322.591 Z M 256 359.016 C 188.432 359.016 52.021 393.437 52.021 461.005 L 52.021 512 L 459.979 512 L 459.979 461.005 C 459.979 393.437 323.568 359.016 256 359.016 Z" fill="#82aeff" style=""/>
                                <path style="fill: rgb(130, 174, 255);" d="M 194.702 162.983 C 186.244 164.843 196.655 179.422 178.946 183.639 C 172.31 185.219 160.402 174.987 160.402 174.987 L 122.115 104.933 C 122.115 104.933 114.466 84.41 121.713 82.275 C 182.71 64.298 244.835 64.853 244.835 64.853 C 244.835 64.853 246.389 49.501 256.018 49.754 C 265.613 49.539 267.166 64.853 267.166 64.853 C 267.166 64.853 329.288 64.299 390.288 82.275 C 397.533 84.411 389.884 104.934 389.884 104.934 L 351.598 174.988 C 351.598 174.988 339.69 185.219 333.054 183.639 C 315.345 179.423 325.754 164.844 317.296 162.984 C 258.543 150.059 253.456 150.058 194.702 162.983 Z"/>
                            </svg>
                        </div>
                    </button>
                </div>
                {if self.logout_open {
                    self.user.as_ref().map(|user| view! {
                        <div class="user-info">
                            <p>{user.display_name.clone().unwrap_or_default()}</p>
                            <button on:click=|_| Self::Msg::Logout>{t!("auth.logout")}</button>
                        </div>
                    })
                } else {
                    None
                }}
            </Host>
        }
    }
}
