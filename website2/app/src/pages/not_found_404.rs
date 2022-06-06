use crate::views::Header;
use leptos2::*;

pub struct NotFound {
    pub path: String,
}

impl Page for NotFound {
    type Params = ();
    type Query = ();

    fn name() -> &'static str {
        "404"
    }

    fn paths() -> Vec<String> {
        vec![String::new()]
    }

    fn build_state(
        _locale: &str,
        path: &str,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        Some(Self {
            path: path.to_string(),
        })
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        view! {
            <>
                <title>{t!("page_404.title")} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/404.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        view! {
            <>
                {Header::new(locale, &t!("common_prayer")).to_node()}
                <main>
                    <h1>{t!("page_404.uh_oh")}</h1>
                    <p>{t!("page_404.explanation")}</p>
                    <section class="links">
                        <a class="email-link" href=format!("mailto:greg@venite.app?subject=(Common Prayer) Broken link at {}", &self.path)>
                            {t!("page_404.email_greg")}
                        </a>
                        <a class="toc-link" href="/">{t!("page_404.button_text")}</a>
                    </section>
                </main>
            </>
        }
    }

    fn static_page() -> bool {
        true
    }
}
