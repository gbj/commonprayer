// Based on dioxus: https://github.com/DioxusLabs/dioxus/blob/03973f692e89f230eae5b8d7ce0579d3b2b34958/packages/core-macro/src/inlineprops.rs

use leptos::*;

pub trait IntoLabel {
    fn into_label(self) -> Box<dyn Fn() -> String>;
}

impl IntoLabel for String {
    fn into_label(self) -> Box<dyn Fn() -> String> {
        Box::new(move || self.clone())
    }
}

impl<F> IntoLabel for F
where
    F: Fn() -> String + 'static,
{
    fn into_label(self) -> Box<dyn Fn() -> String> {
        Box::new(self)
    }
}

#[component]
pub fn Header<L>(cx: Scope, children: Option<Box<dyn Fn() -> Vec<Element>>>, label: L) -> Element
where
    L: IntoLabel,
{
    view! {
        <header class="Header">
            <h1>{label.into_label()}</h1>
            {children.map(|children| view! { <div class="Header-buttons">{children()}</div> })}
        </header>
    }
}
