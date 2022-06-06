mod hymnal_search;

use crate::utils::{
    decode_uri,
    fetch::{Fetch, FetchStatus},
};
use hymnal::*;
use leptos2::*;

use crate::views::*;
use hymnal::{HymnMetadata, Hymnal, Hymnals};

pub use hymnal_search::HymnalSearch;

#[derive(Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
}

#[derive(Deserialize)]
pub struct HymnalPageQuery {
    q: Option<String>,
}

pub struct HymnalPage {
    search: String,
    search_results: Vec<HymnMetadata>,
    hymnals: Vec<Hymnal>,
}

impl Page for HymnalPage {
    type Params = HymnalPageParams;
    type Query = HymnalPageQuery;

    fn name() -> &'static str {
        "hymnal"
    }

    fn paths() -> Vec<String> {
        vec!["".into(), "{hymnal}".into()]
    }

    fn build_state(
        _locale: &str,
        path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let hymnals = match params.hymnal {
            None => {
                vec![
                    HYMNAL_1982.to_metadata(),
                    LEVAS.to_metadata(),
                    WLP.to_metadata(),
                    EL_HIMNARIO.to_metadata(),
                ]
            }
            Some(hymnal_id) => {
                let hymnal: Hymnal = hymnal_id.into();
                vec![hymnal.to_metadata()]
            }
        };

        let search = query.q.unwrap_or_default();

        let s = Some(match params.hymnal {
            None => HymnalPage {
                search_results: HYMNAL_1982
                    .search(&search)
                    .chain(LEVAS.search(&search))
                    .chain(WLP.search(&search))
                    .chain(EL_HIMNARIO.search(&search))
                    .collect(),
                hymnals,
                search,
            },
            Some(hymnal_id) => {
                let hymnal: Hymnal = hymnal_id.into();
                HymnalPage {
                    search_results: hymnal.search(&search).collect(),
                    hymnals,
                    search,
                }
            }
        });
        s
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        let title = if self.hymnals.len() == 1 {
            self.hymnals[0].title.clone()
        } else {
            t!("menu.hymnal")
        };

        view! {
            <>
                <title>{title} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        let results_query_status = if self.search_results.is_empty() {
            FetchStatus::Idle
        } else {
            FetchStatus::Success(Box::new(self.search_results.clone()))
        };

        let results_query_state = NestedState::new(Fetch::with_status(results_query_status));

        view! {
            <>
                {Header::new(locale, &t!("menu.hymnal")).to_node()}
                <main>
                    <HymnalSearch
                        locale={locale}
                        search={&self.search}
                        prop:state={results_query_state.clone()}
                        prop:hymnals={self.hymnals.clone()}
                    >
                    </HymnalSearch>
                </main>
            </>
        }
    }
}
