use leptos::*;

use crate::i18n::use_i18n;

pub fn set_title(cx: Scope, title: impl std::fmt::Display) {
    match use_context::<HeaderContext>(cx) {
        None => {
            log::warn!("use_title() called without a <Header/>");
        }
        Some(ctx) => {
            let HeaderContext { set_title, .. } = ctx;
            set_title.update(|n| *n = title.to_string());
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HeaderContext {
    pub title: ReadSignal<String>,
    pub set_title: WriteSignal<String>,
}

#[component]
pub fn Header(cx: Scope) -> Element {
    let (t, _, _) = use_i18n(cx);
    let (title, set_title) = create_signal(cx, t("common_prayer"));
    provide_context(cx, HeaderContext { title, set_title });

    view! {
        <header><h1>{title}</h1></header>
    }
}
