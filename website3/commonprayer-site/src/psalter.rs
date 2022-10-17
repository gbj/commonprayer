use crate::document::psalm::*;
use crate::header::*;
use crate::i18n::use_i18n;
use crate::icon::Icon;
use crate::settings::use_display_settings;
use common_macros::hash_map;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use liturgy::{Psalm, Version};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PsalterData {
    pub number: Memo<u8>,
    pub version: Memo<Version>,
    pub psalm: Resource<(u8, Version), Result<Psalm, ()>>,
}

pub async fn psalter_data(_cx: Scope, _params: ParamsMap, url: Url) -> Option<Psalm> {
    log::debug!("<Psalter/> psalter_data()");
    let q = url.search_params();
    let number = q
        .get("number")
        .and_then(|n| n.parse::<u8>().ok())
        .unwrap_or(1);
    let version = q
        .get("version")
        .and_then(|v| v.parse::<Version>().ok())
        .unwrap_or_default();
    let psalter = if version == Version::LibroDeOracionComun {
        &*psalter::loc::LOC_PSALTER
    } else {
        &*psalter::bcp1979::BCP1979_PSALTER
    };
    psalter.psalm_by_number(number).cloned()
}

#[component]
pub fn Psalter(cx: Scope) -> Element {
    let (t, t_with_args, _) = use_i18n(cx);
    let query = use_query_map(cx);
    let number = create_memo(cx, move |_| {
        query
            .with(|q| q.get("number").and_then(|n| n.parse::<u8>().ok()))
            .unwrap_or(1)
    });

    let version = create_memo(cx, move |_| {
        query
            .with(|q| q.get("version").and_then(|v| v.parse::<Version>().ok()))
            .unwrap_or_default()
    });

    let psalm = use_loader::<Option<Psalm>>(cx);

    let (display_settings, _) = use_display_settings(cx);

    view! { cx,
        <div>
            <Header label=move || t_with_args("daily-readings-psalm", hash_map!("number" => number().to_string()))></Header>
            //<Header label={t}
            <main class=move || display_settings.get().to_class()>
                <Title text=(move || t_with_args("daily-readings-psalm", hash_map!("number" => number().to_string()))).into() />
                <Stylesheet href="/styles/document.css".into() />
                <Stylesheet href="/styles/psalter.css".into() />
                <nav class="Psalter-nav">
                    // Prev Psalm
                    {move || (number() > 1).then(|| view! { cx,  <A href=move || format!("?number={}&version={:?}", number() - 1, version())>
                        <img src=Icon::Left.to_string() alt=move || t("psalm-prev")/>
                    </A>})}

                    // Psalm header (can be edited)
                    <Form>
                        <input name="version" type="hidden" value=move || version.get().to_string()/>
                        <h2 class="Psalter-nav-form-title">
                            <label for="number">{t("lectionary-psalm")}</label>
                            <input class="Psalter-nav-form-input" name="number" id="number" type="text" value=number() prop:value=number />
                        </h2>
                    </Form>

                    // Next Psalm
                    {move || (number() < 150).then(|| view! { cx, <A href=move || format!("?number={}&version={:?}", number() + 1, version())>
                        <img src=Icon::Right.to_string() alt=move || t("psalm-next")/>
                    </A> })}
                </nav>
                <div class:pending={move || psalm.loading()}>
                    {move || psalm.read().map(|psalm| match psalm {
                        Some(psalm) => view! { cx,  <Psalm psalm/> },
                        _ => view! { cx,  <p class="error">{t("psalm-error")}</p> },
                    })}
                </div>
            </main>
        </div>
    }
}
