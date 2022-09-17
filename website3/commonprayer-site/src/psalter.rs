use crate::i18n::use_i18n;
use crate::icon::Icon;
use crate::{document::*, fetch::fetch};
use common_macros::hash_map;
use leptos::*;
use liturgy::{Psalm, Version};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PsalterData {
    pub number: Memo<u8>,
    pub version: Memo<Version>,
    pub psalm: Resource<(u8, Version), Result<Psalm, ()>>,
}

async fn fetch_psalm((number, version): (u8, Version)) -> Result<Psalm, ()> {
    // uncomment to fetch psalms from server
    /* fetch(&format!("/psalm?number={number}&version={version}"))
    .await
    .map_err(|_| ()) */

    // uncomment to include psalters in WASM bundle
    use psalter::{bcp1979::BCP1979_PSALTER, loc::LOC_PSALTER};
    if version == Version::LibroDeOracionComun {
        LOC_PSALTER.psalm_by_number(number).cloned().ok_or(())
    } else {
        BCP1979_PSALTER.psalm_by_number(number).cloned().ok_or(())
    }
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
    let (t, t_with_args, _) = use_i18n(cx);
    let PsalterData {
        number,
        version,
        psalm,
    } = use_loader(cx);

    view! {
        <main>
            <nav class="Psalter-nav">
                {move || (number() > 1).then(|| view! { <Link to={move || format!("?number={}&version={:?}", number() - 1, version())}>
                    <img src={Icon::Left.to_string()} alt={move || t("psalm-prev")}/>
                </Link>})}
                <h2>{move || t_with_args("daily-readings-psalm", &hash_map! { "number".to_string() => number.get().into() })}</h2>
                {move || (number() < 150).then(|| view ! { <Link to={move || format!("?number={}&version={:?}", number() + 1, version())}>
                    <img src={Icon::Right.to_string()} alt={move || t("psalm-next")}/>
                </Link> })}
            </nav>
            {move || psalm.read().map(|psalm| match psalm {
                Err(_) => view! { <p class="error">{t("psalm-error")}</p> },
                Ok(psalm) => view! { <Psalm psalm/> }
            })}
        </main>
    }
}
