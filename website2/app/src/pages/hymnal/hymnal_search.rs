use std::collections::HashSet;
use std::rc::Rc;

use crate::utils::fetch::fetch;
use crate::{fragments::SegmentButton, utils::fetch::FetchError};
use hymnal::{HymnNumber, Hymnal, Hymnals};
use leptos2::*;
use wasm_bindgen::JsCast;
use web_sys::AbortController;

#[derive(Clone, Debug, Default, WebComponent)]
pub struct HymnalSearch {
    pub locale: String,
    pub hymnal_count: usize,
    #[prop]
    pub hymnal: Option<Hymnals>,
    pub search: String,
    #[prop]
    pub latest_search_results: Option<HashSet<(Hymnals, HymnNumber)>>,
    #[prop]
    pub hymns: Vec<(Hymnals, HymnNumber)>,
    #[prop]
    pub metadata: Vec<Hymnal>,
    loading: bool,
    search_error: Option<FetchError>,
    search_abort_controller: Option<AbortController>,
    hymns_with_nodes: Vec<(Hymnals, HymnNumber, web_sys::HtmlElement)>,
}

impl PartialEq for HymnalSearch {
    fn eq(&self, other: &Self) -> bool {
        self.locale == other.locale
            && self.hymnal_count == other.hymnal_count
            && self.hymnal == other.hymnal
            && self.search == other.search
            && self.latest_search_results == other.latest_search_results
            && self.hymns == other.hymns
            && self.metadata == other.metadata
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
    SetHymnsWithNodes(Vec<(Hymnals, HymnNumber, web_sys::HtmlElement)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HymnalSearchCmd {
    FetchSearch(String, Option<AbortController>),
    CancelSearch(AbortController, String),
    GrabNodesForHymns(Vec<(Hymnals, HymnNumber)>),
    FilterHymns(
        HashSet<(Hymnals, HymnNumber)>,
        Vec<(Hymnals, HymnNumber, web_sys::HtmlElement)>,
    ),
}

#[async_trait(?Send)]
impl Component for HymnalSearch {
    type Msg = HymnalSearchMsg;
    type Cmd = HymnalSearchCmd;

    fn init(&self) -> Option<Self::Cmd> {
        Some(HymnalSearchCmd::GrabNodesForHymns(self.hymns.clone()))
    }

    fn update(&mut self, msg: &Self::Msg) -> Option<Self::Cmd> {
        match msg {
            HymnalSearchMsg::SetHymnal(hymnal) => {
                self.hymnal = *hymnal;
                None
            }
            HymnalSearchMsg::SearchChanged(search) => {
                self.search = search.to_string();
                if self.loading {
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
                        self.latest_search_results = None;
                        Some(HymnalSearchCmd::FilterHymns(
                            HashSet::new(),
                            self.hymns_with_nodes.clone(),
                        ))
                    } else {
                        self.loading = true;
                        Some(HymnalSearchCmd::FetchSearch(
                            self.search.clone(),
                            abort_controller,
                        ))
                    }
                }
            }
            HymnalSearchMsg::SearchSuccess(res) => {
                self.loading = false;
                self.latest_search_results = Some(res.clone());
                Some(HymnalSearchCmd::FilterHymns(
                    res.clone(),
                    self.hymns_with_nodes.clone(),
                ))
            }
            HymnalSearchMsg::SearchError(e) => {
                self.search_error = Some(*e);
                None
            }
            HymnalSearchMsg::SetHymnsWithNodes(toc) => {
                self.hymns_with_nodes = toc.clone();
                None
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
            // this kind of DOM manipulation within a command is not ideal,
            // but it is the only way to manipulate these pre-rendered hymns
            HymnalSearchCmd::GrabNodesForHymns(hymns) => {
                let slotted_hymns = host.query_selector_all(".hymn-listing").unwrap();
                let hymns_with_nodes = hymns
                    .iter()
                    .enumerate()
                    .map(|(idx, (hymnal, hymn_number))| {
                        (
                            *hymnal,
                            *hymn_number,
                            slotted_hymns
                                .item(idx as u32)
                                .unwrap()
                                .unchecked_into::<web_sys::HtmlElement>(),
                        )
                    })
                    .collect::<Vec<_>>();
                Some(HymnalSearchMsg::SetHymnsWithNodes(hymns_with_nodes))
            }
            HymnalSearchCmd::FilterHymns(ids, hymns_with_nodes) => {
                for (hymnal, hymn_number, element) in hymns_with_nodes {
                    let class_list = element.class_list();
                    if ids.is_empty() || ids.contains(&(*hymnal, *hymn_number)) {
                        class_list.remove_1("hidden");
                    } else {
                        class_list.add_1("hidden");
                    }
                }
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
                {self.search_bar()}
                <p class="loading search-state" class:hidden={!self.loading}>{t!("loading")}</p>

                // Results filtered by search
                <slot name="hymns"></slot>
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

    fn search_bar(&self) -> Node {
        view! {
            <label class="stacked">
                {t!("search")}
                <input
                    type="search"
                    value={&self.search}
                    prop:value={self.search.clone()}
                    on:input=|ev| HymnalSearchMsg::SearchChanged(event_target_value(&ev))
                />
            </label>
        }
    }
}
