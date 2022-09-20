use crate::document::{SmallCaps, SmallCapsProps};
use leptos::*;

#[component]
pub fn Psalm(cx: Scope, psalm: liturgy::Psalm) -> Element {
    let psalm_number = psalm.number;
    let sections = psalm.filtered_sections();

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
                            <a id=format!("{}-{}", psalm_number, number)></a>
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
        <article class="document Psalm">
            <a id=psalm.citation></a>
            <header class="Psalm-header">
                <h3 class="Psalm-number">{psalm_number.to_string()}</h3>
                {section_1_header}
            </header>
            <main>{sections}</main>
        </article>
    }
}
