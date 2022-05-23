use std::collections::HashSet;
use std::rc::Rc;

use crate::utils::fetch::fetch;
use crate::{fragments::SegmentButton, utils::fetch::FetchError};
use hymnal::{HymnNumber, Hymnals};
use leptos2::*;
use web_sys::AbortController;

#[derive(Clone, Debug, Default, WebComponent)]
pub struct HymnalSearch {
    pub locale: String,
    pub hymnal_count: usize,
    #[prop]
    pub hymnal: Option<Hymnals>,
    pub search: String,
    loading: bool,
    search_error: Option<FetchError>,
    search_abort_controller: Option<AbortController>,
}

impl PartialEq for HymnalSearch {
    fn eq(&self, other: &Self) -> bool {
        self.locale == other.locale
            && self.hymnal_count == other.hymnal_count
            && self.hymnal == other.hymnal
            && self.search == other.search
            && self.loading == other.loading
            && self.search_error == other.search_error
            && self.search_abort_controller == other.search_abort_controller
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HymnalSearchMsg {
    SetHymnal(Option<Hymnals>),
    SearchChanged(String),
    SearchSuccess(HashSet<(Hymnals, HymnNumber)>),
    SearchError(FetchError),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HymnalSearchCmd {
    FetchSearch(String, Option<AbortController>),
    CancelSearch(AbortController, String),
    EmitResults(HashSet<(Hymnals, HymnNumber)>),
    EmitHymnal(Option<Hymnals>),
}

#[async_trait(?Send)]
impl Component for HymnalSearch {
    type Msg = HymnalSearchMsg;
    type Cmd = HymnalSearchCmd;

    fn update(&mut self, msg: &Self::Msg) -> (bool, Option<Self::Cmd>) {
        match msg {
            HymnalSearchMsg::SetHymnal(hymnal) => {
                leptos2::warn("setting hymnal...");
                self.hymnal = *hymnal;
                (true, Some(HymnalSearchCmd::EmitHymnal(self.hymnal)))
            }
            HymnalSearchMsg::SearchChanged(search) => {
                self.search = search.to_string();
                let cmd = if self.loading {
                    self.loading = false;
                    self.search_abort_controller
                        .as_ref()
                        .map(|ac| HymnalSearchCmd::CancelSearch(ac.clone(), self.search.clone()))
                } else {
                    let abort_controller = AbortController::new().ok();
                    self.search_abort_controller = abort_controller.clone();
                    self.search_error = None;

                    if self.search.is_empty() {
                        self.loading = false;
                        Some(HymnalSearchCmd::EmitResults(HashSet::new()))
                    } else {
                        self.loading = true;
                        Some(HymnalSearchCmd::FetchSearch(
                            self.search.clone(),
                            abort_controller,
                        ))
                    }
                };
                (false, cmd)
            }
            HymnalSearchMsg::SearchSuccess(res) => {
                self.loading = false;
                (true, Some(HymnalSearchCmd::EmitResults(res.clone())))
            }
            HymnalSearchMsg::SearchError(e) => {
                self.search_error = Some(*e);
                leptos2::warn(&format!("[HymnalSearch] {}", e));
                (true, None)
            }
        }
    }

    async fn cmd(cmd: Self::Cmd, host: web_sys::HtmlElement) -> Option<Self::Msg> {
        match &cmd {
            HymnalSearchCmd::FetchSearch(search, controller) => {
                match fetch::<HashSet<(Hymnals, HymnNumber)>>(
                    &format!("/api/hymnal/search?q={}", search),
                    controller.as_ref().map(|ac| ac.signal()).as_ref(),
                )
                .await
                {
                    Ok(res) => Some(HymnalSearchMsg::SearchSuccess(res)),
                    Err(e) => Some(HymnalSearchMsg::SearchError(e)),
                }
            }
            HymnalSearchCmd::CancelSearch(controller, new_search) => {
                controller.abort();
                Some(HymnalSearchMsg::SearchChanged(new_search.to_string()))
            }
            HymnalSearchCmd::EmitResults(res) => {
                let event_emitter = EventEmitter::new(&host);
                event_emitter.emit(CustomEvent::new("search").bubbles().detail(res.clone()));
                None
            }
            HymnalSearchCmd::EmitHymnal(hymnal) => {
                let event_emitter = EventEmitter::new(&host);
                event_emitter.emit(CustomEvent::new("hymnal").bubbles().detail(*hymnal));
                None
            }
        }
    }

    fn view(&self) -> Host {
        view! {
            <Host>
                <style>
                {include_str!("../../components/toggle.css")}
                {include_str!("./hymnal_search.css")}
                </style>
                // Choose a hymnal
                {if self.hymnal_count == 1 {
                    None
                } else {
                    {Some(self.choose_hymnal().to_node())}
                }}

                // Search bar
                <label class="stacked">
                    {t!("search")}
                    <input
                        type="search"
                        value={&self.search}
                        prop:value={self.search.clone()}
                        on:input=|ev| HymnalSearchMsg::SearchChanged(event_target_value(&ev))
                    />
                </label>

                <p class="loading search-state" class:hidden={!self.loading}>{t!("loading")}</p>
            </Host>
        }
    }
}

impl HymnalSearch {
    fn choose_hymnal(&self) -> SegmentButton<Option<Hymnals>, HymnalSearchMsg> {
        SegmentButton {
            name: "search-hymnal".to_string(),
            legend: t!("menu.hymnal"),
            options: vec![
                (None, t!("hymnal.any"), None),
                (Some(Hymnals::Hymnal1982), t!("hymnal.h82_abbrev"), None),
                (Some(Hymnals::LEVAS), t!("hymnal.levas_abbrev"), None),
                (Some(Hymnals::WLP), t!("hymnal.wlp_abbrev"), None),
                (Some(Hymnals::ElHimnario), t!("hymnal.el_himnario"), None),
            ],
            value: None,
            msg_fn: Rc::new(|val| HymnalSearchMsg::SetHymnal(*val)),
        }
    }
}
