use leptos::*;

pub mod biblical_reading;
pub mod psalm;
pub mod reference;

#[component]
pub fn Document(cx: Scope, doc: liturgy::Document) -> Element {
    view! { cx, 
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
                    Some(view! { cx,  <span>{piece.replace("LORD", "")}</span> }),
                    Some(view! { cx,  <span class="lord">"Lord"</span> }),
                ]
                .into_iter()
            } else if piece.ends_with("GOD") {
                [
                    Some(view! { cx,  <span>{piece.replace("GOD", "")}</span> }),
                    Some(view! { cx,  <span class="lord">"God"</span> }),
                ]
                .into_iter()
            } else if piece.ends_with("YAHWEH") {
                [
                    Some(view! { cx,  <span>{piece.replace("YAHWEH", "")}</span> }),
                    Some(view! { cx,  <span class="lord">"Yahweh"</span> }),
                ]
                .into_iter()
            } else {
                [None, Some(view! { cx,  <span>{piece.to_string()}</span> })].into_iter()
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}
