use leptos::*;

#[component]
pub fn Reference(cx: Scope, reference: liturgy::Reference) -> Element {
    let href = reference.as_url();
    let text = reference.to_string();
    view! {
        <a class="reference" href=href>{text}</a>
    }
}
