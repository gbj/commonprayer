use crate::utils::encode_uri;
use crate::views::{menu, Header};
use leptos2::{view::View, *};

#[derive(Debug)]
pub struct Index {
    locale: String,
    path: String,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            path: String::new(),
        }
    }
}

#[async_trait]
impl Loader for Index {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        path: &str,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        Some(Self {
            locale: locale.to_string(),
            path: path.to_string(),
        })
    }
}

impl View for Index {
    fn title(&self) -> String {
        t!("common_prayer")
    }

    fn meta(&self) -> MetaTags {
        vec![
            ("charset".to_string(), "UTF-8".to_string()),
            (
                "viewport".to_string(),
                "width=device-width, initial-scale=1.0".to_string(),
            ),
        ]
    }

    fn links(&self) -> Vec<Node> {
        view! {
            <>
                <link rel="preload" href="/static/fonts/Sabon_Roman.woff2" _as="font" type="font/woff2" crossorigin />
                <link rel="preload" href="/static/fonts/Sabon_Bold.woff2" _as="font" type="font/woff2" crossorigin/>
                <link rel="preload" href="/static/fonts/Sabon_Italic.woff2" _as="font" type="font/woff2" crossorigin/>
                <link rel="preload" href="/static/fonts/Sabon_BoldItalic.woff2" _as="font" type="font/woff2" crossorigin/>
            </>
        }
    }

    fn styles(&self) -> view::Styles {
        vec![
            include_str!("vars.css").into(),
            include_str!("general.css").into(),
            include_str!("header.css").into(),
            include_str!("menu.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Vec<Node> {
        let menu = menu(&self.locale, &self.path);

        let mut index_content = match nested_view {
            Some(view) => vec![menu, view],
            None => {
                view! {
                    <>
                        {menu}
                        <div>
                            <header><h1>{t!("common_prayer")}</h1></header>
                            <main>

                            </main>
                        </div>
                    </>
                }
            }
        };

        index_content.extend(body_scripts());
        index_content.push(view! {
			<script defer data-domain="commonprayeronline.org" src="https://plausible.io/js/plausible.js"></script>
		});
        index_content
    }

    fn error_boundary(error: RouterError) -> Body {
        view! {
            <>
                <style>{include_str!("404.css")}</style>
                <header><h1>{t!("common_prayer")}</h1></header>
                <main>
                    <h1>{t!("page_404.uh_oh")}</h1>
                    <p>{t!("page_404.explanation")}</p>
                    <section class="links">
                        <a class="email-link" href=format!("mailto:greg@venite.app?subject=(Common Prayer) Error&body={}", encode_uri(&format!("{:#?}", error)))>
                            {t!("page_404.email_greg")}
                        </a>
                        <a class="toc-link" href="/">{t!("page_404.button_text")}</a>
                    </section>
                </main>
            </>
        }
    }
}
