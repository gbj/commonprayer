use std::collections::HashSet;

use super::HymnalSearch;
use hymnal::{HymnNumber, Hymnals};
use leptos2::*;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, WebComponent)]
pub struct HymnWrapper {
    pub hymnal: Hymnals,
    pub number: HymnNumber,
    hidden: bool,
    search_results: HashSet<(Hymnals, HymnNumber)>,
    chosen_hymnal: Option<Hymnals>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HymnWrapperMsg {
    SetResults(HashSet<(Hymnals, HymnNumber)>),
    SetChosenHymnal(Option<Hymnals>),
}

#[async_trait(?Send)]
impl Component for HymnWrapper {
    type Msg = HymnWrapperMsg;
    type Cmd = ();

    fn update(&mut self, msg: &Self::Msg) -> (bool, Option<Self::Cmd>) {
        match msg {
            HymnWrapperMsg::SetResults(res) => self.search_results = res.clone(),
            HymnWrapperMsg::SetChosenHymnal(hymnal) => self.chosen_hymnal = *hymnal,
        }
        (true, None)
    }

    async fn cmd(_cmd: Self::Cmd, _host: web_sys::HtmlElement) -> Option<Self::Msg> {
        None
    }

    fn view(&self) -> Host {
        let hymnal = self.hymnal;
        let number = self.number;
        let handle_search_results = move |ev: web_sys::Event| {
            let ev: CustomEvent<HashSet<(Hymnals, HymnNumber)>> = ev.into();
            Self::Msg::SetResults(ev.detail.unwrap_or_default())
        };
        let handle_hymnal_change = move |ev: web_sys::Event| {
            let ev: CustomEvent<Option<Hymnals>> = ev.into();
            leptos2::warn(&format!("caught hymnal change {:?}", ev.detail));
            Self::Msg::SetChosenHymnal(ev.detail.unwrap_or_default())
        };

        let in_search =
            self.search_results.is_empty() || self.search_results.contains(&(hymnal, number));
        let in_hymnal = self.chosen_hymnal.is_none() || self.chosen_hymnal == Some(hymnal);

        view! {
            <Host class:hidden={!(in_search && in_hymnal)}
                foreign:search=(HymnalSearch::tag(), handle_search_results)
                foreign:hymnal=(HymnalSearch::tag(), handle_hymnal_change)
            >
                <slot></slot>
            </Host>
        }
    }
}
