use std::rc::Rc;

use leptos2::*;

pub struct SegmentButton<T, M>
where
    T: Clone + PartialEq,
{
    pub name: String,
    pub options: Vec<(T, String, Option<String>)>,
    pub legend: String,
    pub value: T,
    pub msg_fn: Rc<dyn Fn(&T) -> M>,
}

impl<T, M> StaticView for SegmentButton<T, M>
where
    T: Clone + PartialEq + 'static,
    M: 'static,
{
    fn to_node(&self) -> Node {
        let legend_view = if self.legend.is_empty() {
            None
        } else {
            Some(view! { <legend>{&self.legend}</legend> })
        };

        let options = self
            .options
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(idx, (value, label, desc))| {
                let id = format!("{}-{}", self.name, idx);
                let desc = match desc {
                    Some(desc) => view! { <p class="description">{desc}</p> },
                    None => view! { <p></p> },
                };
                let msg_fn = self.msg_fn.clone();

                view! {
                    <>
                        <input
                            type="radio"
                            id={&id}
                            name={&self.name}
                            value={idx}
                            on:change=move |_| (msg_fn)(&value)
                            checked={self.value == value}
                            prop:checked={self.value == value}
                        />
                        <label for={&id}>
                            {label}
                        </label>
                    </>
                }
            })
            .collect::<Vec<_>>();

        // Optional descriptions that show below the buttons
        let description_field = if self.options.iter().any(|(_, _, desc)| desc.is_some()) {
            let current_description = self
                .options
                .iter()
                .find(|(s_value, _, _)| s_value == &self.value)
                .and_then(|(_, _, desc)| desc.as_ref().cloned())
                .unwrap_or_default();
            Some(view! {
                <p class="description">{current_description}</p>
            })
        } else {
            None
        };

        view! {
            <div>
                <fieldset class="toggle segment-button">
                    {legend_view}
                    {options}
                </fieldset>
                {description_field}
            </div>
        }
    }
}
