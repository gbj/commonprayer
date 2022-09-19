use leptos::*;
use liturgy::Psalm;

/*pub enum Prop<T> {
    Static(T),
    Dynamic(Box<dyn FnOnce() -> T>),
}

pub trait IntoProp<T> {
    fn into_prop(self) -> Prop<T>;
}

impl<T> FnOnce<()> for Prop<T> {
    type Output = T;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        match self {
            Prop::Static(val) => val,
            Prop::Dynamic(f) => f(),
        }
    }
}

macro_rules! prop_type {
    ($t:ty) => {
        impl IntoProp<$t> for $t {
            fn into_prop(self) -> Prop<$t> {
                Prop::Static(self)
            }
        }

        impl<F> IntoProp<$t> for F
        where
            F: FnOnce() -> $t + 'static,
        {
            fn into_prop(self) -> Prop<$t> {
                Prop::Dynamic(Box::new(self))
            }
        }
    };
}

// Definitions for this one
prop_type!(Psalm); */

#[component]
pub fn Psalm(cx: Scope, psalm: Psalm) -> Element {
    let psalm_number = psalm.number;

    // TODO if you want to filter client-side, will add 500kb of WASM at present
    // let sections = psalm.filtered_sections();
    let sections = psalm.sections;

    let section_1_header = sections.get(0).map(|section| {
        view! {
            <div>
                {section.local_name.is_empty().then(|| view! {
                    <h3 class="Psalm-local-name">{&section.local_name}</h3>
                })}
                <em class="Psalm-latin-name">{&section.latin_name}</em>
            </div>
        }
    });

    let sections = sections
        .into_iter()
        .enumerate()
        .map(|(idx, section)| {
            let local = section.local_name;
            let latin = section.latin_name;

            let verses = section
                .verses
                .into_iter()
                .map(|verse| {
                    let number = verse.number;

                    view! {
                        <p class="Psalm-verse">
                            <a id={format!("{}-{}", psalm_number, number)}></a>
                            <sup class="Psalm-verse-number">{number.to_string()}</sup>
                            <span class="Psalm-verse-a"><SmallCaps line=verse.a/></span>
                            <span class="Psalm-verse-b"><SmallCaps line=verse.b/></span>
                        </p>
                    }
                })
                .collect::<Vec<_>>();

            let header = if idx > 0 {
                Some(view! {
                    <header class="Psalm-section-header">
                        {if local.is_empty() {
                            None
                        } else {
                            Some(view! {
                                <h3 class="Psalm-local-name">{local}</h3>
                            })
                        }}
                        <em class="Psalm-latin-name">{latin}</em>
                    </header>
                })
            } else {
                None
            };

            view! {
                <section>
                    {header}
                    <main>{verses}</main>
                </section>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <article class="document psalm">
            <header class="Psalm-header">
                <h3 class="Psalm-number">{psalm_number.to_string()}</h3>
                {section_1_header}
            </header>
            <main>{sections}</main>
        </article>
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
