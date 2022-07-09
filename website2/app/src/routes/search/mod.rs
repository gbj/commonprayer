mod search;
mod search_result;

use crate::WebView;
use hymnal::{EL_HIMNARIO, HYMNAL_1982, LEVAS, WLP};
use language::Language;
use leptos2::*;
use library::CommonPrayer;
use search::global_search;
use search_result::SearchResult;

#[derive(Params)]
pub struct SearchViewQuery {
    q: Option<String>,
}

pub struct SearchView {
    search: String,
    locale: String,
    state: SearchViewState,
}

pub enum SearchViewState {
    EmptyQuery,
    NoResults,
    Results(Vec<SearchResult>),
}

#[async_trait(?Send)]
impl Loader for SearchView {
    type Params = ();
    type Query = SearchViewQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let q = query.q.unwrap_or_default();
        let state = if q.is_empty() {
            SearchViewState::EmptyQuery
        } else {
            let hymnals = vec![&*HYMNAL_1982, &*LEVAS, &*WLP, &*EL_HIMNARIO];
            let mut results =
                global_search::<CommonPrayer>(hymnals, &q, Language::from_locale(locale));
            if results.is_empty() {
                SearchViewState::NoResults
            } else {
                results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                SearchViewState::Results(results)
            }
        };

        Some(Self {
            search: q,
            locale: locale.to_string(),
            state,
        })
    }
}

impl View for SearchView {
    fn title(&self) -> String {
        if self.search.is_empty() {
            format!("{} – {}", t!("search"), t!("common_prayer"))
        } else {
            format!(
                "“{}” – {} – {}",
                self.search,
                t!("search"),
                t!("common_prayer")
            )
        }
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("../hymnal/hymnal.css").into(),
            include_str!("search.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        let res = match self.state {
            SearchViewState::EmptyQuery => view! {
                <p>{t!("search_page.enter_query")}</p>
            },
            SearchViewState::NoResults => view! {
                <p>{t!("search_page.no_results")}</p>
            },
            SearchViewState::Results(res) => view! {
                <div>
                    <p class="results-number">
                        {if res.len() == 1 {
                            t!("search_page.results_singular")
                        } else {
                            t!("search_page.results_plural", number = &res.len().to_string())
                        }}
                    </p>
                    <ol class="results">{
                        res.into_iter()
                            .map(|result| result.view(&self.locale))
                            .collect::<Vec<_>>()
                        }
                    </ol>
                </div>
            },
        };

        view! {
            <div>
                <header>
                    <h1>
                        {if self.search.is_empty() {
                            t!("search")
                        } else {
                            format!("“{}”", self.search)
                        }}
                    </h1>
                </header>
                <main>
                    <form>
                        // Search bar
                        <label class="stacked">
                            {t!("search")}
                            <input
                                type="search"
                                name="q"
                                value={&self.search}
                            />
                        </label>
                    </form>
                    {res}
                </main>
            </div>
        }
    }
}
