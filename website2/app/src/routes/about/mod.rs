use crate::views::Header;
use leptos2::{*, view::View};

#[derive(Debug)]
pub struct About {
    locale: String,
    markdown_html: String
}

#[async_trait]
impl Loader for About {
    type Params = ();
    type Query = ();

    async fn loader(
		locale: &str,
        _path: &str,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        // allowed because future i18n will be inserted here
        #[allow(clippy::match_single_binding)]
        let markdown = match locale {
            _ => include_str!("about.en.md")
        };
        Some(Self {
            locale: locale.to_string(),
            markdown_html: markdown::to_html(markdown)
        })
    }
}

impl View for About {
    fn title(&self) -> String {
        format!("{} – {}", t!("menu.about"), t!("common_prayer"))
    }

    fn styles(&self) -> view::Styles {
        vec![include_str!("about.css").into()]
    }

    fn body(&self, _nested_view: Option<Node>) -> view::Body {
        let markdown_block = Element::new("article").inner_html(self.markdown_html.clone());

        view! {
            <>
                <header><h1>{t!("menu.about")}</h1></header>
                <main>
                    <h2>{t!("about.title")}</h2>
                    <h3 class="subtitle">{t!("about.subtitle")}</h3>
                    {markdown_block}
                    <div class="buttons">
                        <a href="https://github.com/gbj/commonprayer" class="github" target="_blank"><span>"GitHub"</span></a>
                        <a href="mailto:greg@venite.app" class="email" target="_blank"><span>{t!("about.email")}</span></a>
                    </div>
                </main>
            </>
        }
    }
}