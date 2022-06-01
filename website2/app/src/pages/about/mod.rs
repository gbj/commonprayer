use crate::views::Header;
use leptos2::*;

pub struct AboutPage {}

impl Page for AboutPage {
    type Params = ();

    fn name() -> &'static str {
        "about"
    }

    fn paths() -> Vec<String> {
        vec!["".into()]
    }

    fn build_state(_locale: &str, _path: &str, _params: Self::Params) -> Option<Self> {
        Some(Self {})
    }

    fn static_page() -> bool {
        true
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        view! {
            <>
                <title>{t!("menu.about")} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/about.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        let md = include_str!("./about.en.md");
        let markdown_html = markdown::to_html(md);
        let markdown_block = Element::new("article").inner_html(markdown_html);

        view! {
            <>
                {Header::new(locale, &t!("menu.about")).to_node()}
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
