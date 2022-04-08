use crate::components::header;
use leptos::*;

pub fn about() -> Page<(), (), ()> {
    Page::new("about")
        .head_fn(head)
        .body_fn(body)
        .hydration_state(|_, _, _| Some(()))
        .static_page()
}

pub fn head(_locale: &str, _props: &(), _render_state: &()) -> View {
    view! {
        <>
            <title>{t!("menu.about")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/vars.css"/>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/about.css"/>
        </>
    }
}

pub fn body(locale: &str, _props: &(), _render_state: &()) -> View {
    let md = include_str!("./about.en.md");
    let markdown_html = markdown::to_html(md);
    let markdown_block =
        View::StaticElement(StaticElement::new("article").inner_html(markdown_html));

    view! {
        <>
            {header(locale, &t!("menu.about"))}
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
