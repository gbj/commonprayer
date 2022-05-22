use crate::views::Header;
use leptos2::*;
use serde::{Deserialize, Serialize};

pub fn not_found_404() -> Page<NotFoundPageProps, ()> {
    Page::new("404")
        .head_fn(head)
        .body_fn(body)
        .state(|_, path, _| {
            Some(NotFoundPageProps {
                path: path.to_string(),
            })
        })
        .static_page()
        .incremental_generation()
}

#[derive(Serialize, Deserialize)]
pub struct NotFoundPageProps {
    path: String,
}

fn head(_locale: &str, _props: &NotFoundPageProps) -> Vec<Node> {
    view! {
        <>
            <title>{t!("page_404.title")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/vars.css"/>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/404.css"/>
        </>
    }
}

fn body(locale: &str, props: &NotFoundPageProps) -> Vec<Node> {
    view! {
        <>
            {Header::new(locale, &t!("common_prayer")).to_node()}
            <main>
                <h1>{t!("page_404.uh_oh")}</h1>
                <p>{t!("page_404.explanation")}</p>
                <section class="links">
                    <a class="email-link" href=format!("mailto:greg@venite.app?subject=(Common Prayer) Broken link at {}", props.path)>
                        {t!("page_404.email_greg")}
                    </a>
                    <a class="toc-link" href="/">{t!("page_404.button_text")}</a>
                </section>
            </main>
        </>
    }
}
