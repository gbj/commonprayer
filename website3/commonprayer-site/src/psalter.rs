use crate::i18n::use_i18n;
use crate::icon::Icon;
use crate::settings::use_display_settings;
use crate::{document::*, fetch::fetch};
use leptos::*;
use liturgy::{Psalm, Version};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PsalterData {
    pub number: Memo<u8>,
    pub version: Memo<Version>,
    pub psalm: Resource<(u8, Version), Result<Psalm, ()>>,
}

async fn fetch_psalm((number, version): (u8, Version)) -> Result<Psalm, ()> {
    // uncomment to use static psalm JSON files
    fetch(&format!(
        "/static/psalter/{}/psalm-{:03}.json",
        if version == Version::LibroDeOracionComun {
            "loc"
        } else {
            "bcp1979"
        },
        number
    ))
    .await
    .map_err(|_| ())

    // uncomment to fetch psalms from server
    /* fetch(&format!("/psalm?number={number}&version={version}"))
    .await
    .map_err(|_| ()) */

    // uncomment to include psalters in WASM bundle
    /* use psalter::{bcp1979::BCP1979_PSALTER, loc::LOC_PSALTER};
    if version == Version::LibroDeOracionComun {
        LOC_PSALTER.psalm_by_number(number).cloned().ok_or(())
    } else {
        BCP1979_PSALTER.psalm_by_number(number).cloned().ok_or(())
    } */
}

pub fn psalter_data(cx: Scope, _params: Memo<ParamsMap>, location: Location) -> PsalterData {
    let number = create_memo(cx, move |_| {
        location
            .query
            .with(|q| q.get("number").and_then(|n| n.parse::<u8>().ok()))
            .unwrap_or(1)
    });

    let version = create_memo(cx, move |_| {
        location
            .query
            .with(|q| q.get("version").and_then(|v| v.parse::<Version>().ok()))
            .unwrap_or_default()
    });

    // TODO move to server for bundle size
    let psalm = create_resource(cx, move || (number(), version()), fetch_psalm);

    PsalterData {
        number,
        version,
        psalm,
    }
}

#[component]
pub fn Psalter(cx: Scope) -> Element {
    let (t, _, _) = use_i18n(cx);
    let PsalterData {
        number,
        version,
        psalm,
    } = use_loader(cx);
    let (display_settings, _) = use_display_settings(cx);

    view! {
        <main class=move || display_settings.get().to_class()>
            <nav class="Psalter-nav">
                // Prev Psalm
                {move || (number() > 1).then(|| view! { <Link to=move || format!("?number={}&version={:?}", number() - 1, version())>
                    <img src=Icon::Left.to_string() alt=move || t("psalm-prev")/>
                </Link>})}

                // Psalm header (can be edited)
                <Form>
                    <input name="version" type="hidden" value=move || version.get().to_string()/>
                    <h2 class="Psalter-nav-form-title">
                        <label for="number">{t("lectionary-psalm")}</label>
                        <input class="Psalter-nav-form-input" name="number" id="number" type="text" value=number() prop:value=number />
                    </h2>
                </Form>

                // Next Psalm
                {move || (number() < 150).then(|| view ! { <Link to=move || format!("?number={}&version={:?}", number() + 1, version())>
                    <img src=Icon::Right.to_string() alt=move || t("psalm-next")/>
                </Link> })}
            </nav>
            {move || psalm.read().map(|psalm| match psalm {
                Err(_) => view! { <p class="error">{t("psalm-error")}</p> },
                Ok(psalm) => view! { <Psalm psalm/> }
            })}
        </main>
    }
}
