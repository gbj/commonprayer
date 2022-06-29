use std::sync::Arc;

use hymnal::*;
use itertools::Itertools;
use leptos2::*;

use super::hymnal_metadata;
use crate::routes::hymn::hymnary_page_link;
use crate::Icon;
use hymnal::{HymnMetadata, Hymnal, Hymnals};

#[derive(Params)]
pub struct HymnalPageViewParams {
    hymnal: Hymnals,
}

#[derive(Params, Debug)]
pub struct HymnalPageViewQuery {
    p: u16,
}

pub struct HymnalPageView {
    locale: String,
    hymnal: Hymnal,
    title: String,
    page: u16,
    image_url: String,
}

#[async_trait(?Send)]
impl Loader for HymnalPageView {
    type Params = HymnalPageViewParams;
    type Query = HymnalPageViewQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let hymnal: Hymnal = params.hymnal.into();
        let page = if query.p < 1 { 1 } else { query.p };

        Some(Self {
            locale: locale.to_string(),
            title: hymnal.title.clone(),
            hymnal,
            page,
            image_url: hymnary_page_link(params.hymnal, page),
        })
    }
}

impl View for HymnalPageView {
    fn title(&self) -> String {
        format!("{} – {}", self.title, t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("hymnal_page.css").into(),
            include_str!("../../styles/toggle-links.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <header>
                    <a href={format!("?p={}", self.page - 1)}>
                        <img src={Icon::Left} alt={t!("hymnal.page_back")}/>
                    </a>
                    <h1>{t!("menu.hymnal")}</h1>
                    <a href={format!("?p={}", self.page + 1)}>
                        <img src={Icon::Right} alt={t!("hymnal.page_forward")}/>
                    </a>
                </header>
                <main class="page-scan-viewer">
                    <img class="page-scan" src={self.image_url} alt={t!("hymnal.alt_text")} />
                </main>
                <footer><a href={format!("/{}/hymnal/{:?}", self.locale, self.hymnal.id)}>{hymnal_metadata(&self.hymnal)}</a></footer>
            </div>
        }
    }
}
