use leptos::*;

pub mod biblical_reading;
pub mod psalm;

#[component]
pub fn Document(cx: Scope, doc: liturgy::Document) -> Element {
    view! {
        <p>"TODO generic <Document/> implementation"</p>
    }
}

#[component]
pub fn SmallCaps(cx: Scope, line: String) -> Vec<Element> {
    line.split_inclusive("LORD")
        .flat_map(|s| s.split_inclusive("GOD"))
        .flat_map(|s| s.split_inclusive("YAHWEH"))
        .flat_map(|piece| {
            if piece.ends_with("LORD") {
                [
                    Some(view! { <span>{piece.replace("LORD", "")}</span> }),
                    Some(view! { <span class="lord">"Lord"</span> }),
                ]
                .into_iter()
            } else if piece.ends_with("GOD") {
                [
                    Some(view! { <span>{piece.replace("GOD", "")}</span> }),
                    Some(view! { <span class="lord">"God"</span> }),
                ]
                .into_iter()
            } else if piece.ends_with("YAHWEH") {
                [
                    Some(view! { <span>{piece.replace("YAHWEH", "")}</span> }),
                    Some(view! { <span class="lord">"Yahweh"</span> }),
                ]
                .into_iter()
            } else {
                [None, Some(view! { <span>{piece.to_string()}</span> })].into_iter()
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}
