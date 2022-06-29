use hymnal::{Hymn, Hymnal};
use leptos2::*;

use super::{hymnary_hymnal_id, HymnViewParams};

pub struct HymnMusicView {
    hymn: Hymn,
}

#[async_trait(?Send)]
impl Loader for HymnMusicView {
    type Params = HymnViewParams;
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let hymnal: Hymnal = params.hymnal.into();
        let hymn = hymnal
            .hymns
            .iter()
            .find(|s_hymn| s_hymn.number == params.number)?
            .clone();
        Some(HymnMusicView { hymn })
    }
}

impl View for HymnMusicView {
    fn title(&self) -> String {
        String::new()
    }

    fn styles(&self) -> Styles {
        vec![include_str!("hymn_music_view.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        view! {
            <div class="music-view">
                {if self.hymn.copyright_restriction {
                    view! {
                        <p class="page-scan">{t!("hymnal.copyright_restriction")}</p>
                    }
                } else {
                    view! {
                        <a href={format!("../../../hymnal/{:?}/page?p={}", self.hymn.source, self.hymn.page_number)}>
                            <img
                                src={format!(
                                    "https://hymnary.org/page/fetch/{}/{}/high",
                                    hymnary_hymnal_id(self.hymn.source), self.hymn.page_number
                                )}
                                alt={t!("hymnal.alt_text")}
                                class="page-scan"
                            />
                        </a>
                    }
                }}
            </div>
        }
    }
}
