use leptos2::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[derive(Clone, Debug, PartialEq, Default, WebComponent)]
pub struct Tabs {
    #[prop]
    pub labels: Vec<String>,
    pub align: String,
    pub selected: usize,
    focused: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum TabMsg {
    FocusFirst,
    FocusPrev(usize),
    FocusNext(usize),
    FocusLast,
    Select(usize),
    Noop,
}

#[derive(Copy, Clone, Debug)]
pub enum TabCmd {
    FocusTab(usize),
    SelectTab(usize),
}

impl State for Tabs {
    type Msg = TabMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            TabMsg::FocusFirst => self.focused = 0,
            TabMsg::FocusNext(current_focus) => {
                self.focused = if current_focus + 1 == self.labels.len() {
                    0
                } else {
                    current_focus + 1
                }
            }

            TabMsg::FocusPrev(current_focus) => {
                self.focused = if current_focus == 0 {
                    self.labels.len() - 1
                } else {
                    current_focus - 1
                }
            }
            TabMsg::FocusLast => self.focused = self.labels.len() - 1,
            TabMsg::Select(current_focus) => {
                self.selected = current_focus;
                return Some(self.select_tab());
            }
            TabMsg::Noop => return None,
        };
        Some(self.focus_tab())
    }
}

impl Component for Tabs {
    fn view(&self) -> Host {
        view! {
            <Host>
                <style>{include_str!("tabs.css")}</style>
                <div class="tabs">
                    <div role="tablist">
                        {
                            self.labels.iter()
                                .enumerate()
                                .map(|(idx, label)| view! {
                                    <button id={format!("tab-{}", idx)}
                                        type="button"
                                        role="tab"
                                        aria-selected={if idx == self.selected {
                                            "true"
                                        } else {
                                            "false"
                                        }}
                                        aria-controls={format!("tabpanel-{}", idx)}
                                        on:keydown=move |ev: web_sys::Event| {
                                            let ev = ev.unchecked_into::<web_sys::KeyboardEvent>();
                                            match ev.code().as_str() {
                                                "ArrowLeft" => Self::Msg::FocusPrev(idx),
                                                "ArrowRight" => Self::Msg::FocusNext(idx),
                                                "Space" => Self::Msg::Select(idx),
                                                "Enter" => Self::Msg::Select(idx),
                                                "Home" => Self::Msg::FocusFirst,
                                                "End" => Self::Msg::FocusLast,
                                                _ => Self::Msg::Noop
                                            }
                                        }
                                        on:click=move |_| Self::Msg::Select(idx)
                                    >
                                        <span class="focus">{label}</span>
                                    </button>
                                })
                                .collect::<Vec<_>>()
                        }
                    </div>

                    {
                        self.labels.iter()
                            .enumerate()
                            .map(|(idx, _)| view! {
                                <slot name={format!("content-{}", idx)}
                                    class:hidden={idx != self.selected}
                                >
                                </slot>
                            })
                            .collect::<Vec<_>>()
                    }
                </div>
            </Host>
        }
    }
}

impl Tabs {
    pub fn content(tabs: impl IntoIterator<Item = Node>) -> Vec<Node> {
        tabs.into_iter()
            .enumerate()
            .map(|(idx, content)| {
                view! {
                    <div slot={format!("content-{}", idx)}
                        id=format!("tabpanel-{}", idx)
                        role="tabpanel"
                        aria-labelledby=format!("tab-{}", idx)
                    >
                        {content}
                    </div>
                }
            })
            .collect()
    }

    pub fn focus_tab(&self) -> Cmd<Self> {
        let focused = self.focused;
        Cmd::new(move |host, _| {
            let buttons = host
                .shadow_root()
                .unwrap()
                .query_selector("[role='tablist']")
                .unwrap()
                .unwrap();
            if let Some(tab_btn) = buttons.children().item(focused as u32) {
                tab_btn.unchecked_ref::<HtmlElement>().focus();
            }
        })
    }

    pub fn select_tab(&self) -> Cmd<Self> {
        Cmd::event(CustomEvent::new("change").detail(self.selected).bubbles())
    }
}
