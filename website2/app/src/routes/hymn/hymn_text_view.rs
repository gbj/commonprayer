use hymnal::{Hymn, Hymnal};
use leptos2::*;

use super::HymnViewParams;

pub struct HymnTextView {
    text: String,
}

#[async_trait(?Send)]
impl Loader for HymnTextView {
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
        Some(HymnTextView { text: hymn.text })
    }
}

impl View for HymnTextView {
    fn title(&self) -> String {
        String::new()
    }

    fn styles(&self) -> Styles {
        vec![include_str!("hymn_text_view.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        if self.text.is_empty() {
            view! {
                <div class="text empty">
                    <p class="copyright">{t!("hymnal.copyright_restriction")}</p>
                </div>
            }
        } else {
            view! {
                <div class="text">
                    {self.text
                        .split("\n\n")
                        .map(|stanza| view! { <p class="stanza">{stanza}</p> })
                        .collect::<Vec<_>>()
                    }
                </div>
            }
        }
    }
}
