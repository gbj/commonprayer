mod algorithm;
mod result;

use std::collections::HashMap;

use algorithm::*;
use language::Language;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use result::*;

use crate::header::*;
use crate::i18n::{use_i18n, use_language};

pub async fn search_data(cx: Scope, _params: ParamsMap, url: Url) -> Option<Vec<SearchResult>> {
    use hymnal::{EL_HIMNARIO, HYMNAL_1982, LEVAS, WLP};
    use library::CommonPrayer;

    let locale = use_language(cx).get();

    let q = url.search_params().remove("q").unwrap_or_default();
    if q.is_empty() {
        None
    } else {
        let hymnals = vec![&*HYMNAL_1982, &*LEVAS, &*WLP, &*EL_HIMNARIO];
        let mut results = cx.untrack(move || {
            global_search::<CommonPrayer>(cx, hymnals, &q, Language::from_locale(&locale))
        });
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        Some(results)
    }
}

#[component]
pub fn Search(cx: Scope) -> Element {
    let (t, t_with_args, _) = use_i18n(cx);
    let search = use_query_map(cx);
    let search = move || search.with(|search| search.get("q").cloned());
    let title = move || match search() {
        Some(search) => format!("“{search}” — {}", t("search")),
        None => t("search"),
    };
    let results = use_loader::<Option<Vec<SearchResult>>>(cx);
    let status = move || match results.read() {
        // Loading
        None => t("loading"),
        // Loaded, but no search provided
        Some(None) => t("search-enter_query"),
        // No results found
        Some(Some(res)) if res.is_empty() => t("search-no_results"),
        // Search results
        Some(Some(res)) => {
            if res.len() == 1 {
                t("search-results_singular")
            } else {
                t_with_args(
                    "search-results_plural",
                    HashMap::from([("number", res.len().to_string())]),
                )
            }
        }
    };

    let loading = move || results.loading();

    let results = move || match results.read() {
        Some(Some(res)) => res,
        _ => Vec::new(),
    };

    view! { cx, 
        <div>
            <Header label=t("search")/>
            <main class="Search">
                <Title text=title.into()/>
                <Stylesheet href="/styles/search.css".into() />
                <Form>
                    // Search bar
                    <label class="stacked">
                        {t("search")}
                        <input
                            type="search"
                            name="q"
                            value=search
                            prop:value=search
                        />
                    </label>
                </Form>
                <p class="Search-status" class:pending=loading>{status}</p>
                <ol class:pending=loading>
                    <For each=results key=|res: &SearchResult| (res.link.clone(), res.score)>
                        {|cx: Scope, res: &SearchResult| view! { cx,  <ShowSearchResult content=res.clone() /> }}
                    </For>
                </ol>
            </main>
        </div>
    }
}
