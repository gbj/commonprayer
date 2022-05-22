use std::collections::HashMap;

use super::breadcrumbs::breadcrumbs;
use crate::views::Header;
use leptos2::*;
use liturgy::{parallel_table::ParallelDocument, Document, Slug, SlugPath, Version};

pub fn parallels_body(
    locale: &str,
    base_slug: &SlugPath,
    label: &str,
    initial_text: &str,
    parallels: &[Vec<(ParallelDocument, usize)>],
) -> Vec<Node> {
    todo!()
    /* let selecting = Behavior::new(false);
    let parallel_selections: Behavior<HashMap<(usize, usize), Document>> =
        Behavior::new(HashMap::new());

    // convert table into view
    let parallels_view = parallels
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                let cols = View::Fragment(
                    row.iter()
                        .enumerate()
                        .map(|(col_idx, (child, width))| {
                            let this_doc_selected = Behavior::new(false);

                            let view = match child {
                                ParallelDocument::Source(reference) => source_link(reference),
                                ParallelDocument::Link {
                                    label,
                                    slug,
                                } => view! {
                                    <a href={format!("/{}/document/{}", locale, slug)}>{label}</a>
                                },
                                ParallelDocument::Explainer(Some(explainer)) => view! {
                                    <p class="explainer">{explainer}</p>
                                },
                                ParallelDocument::Explainer(None) => View::Empty,
                                ParallelDocument::Document(doc) => {
                                    let doc = *(*doc).clone();

                                    let checkbox = view! {
                                        <dyn:input
                                            type="checkbox"
                                            class="manual-select hidden"
                                            class:hidden={selecting.stream().map(|selecting| !selecting).boxed_local()}
                                            on:click={
                                                let doc = doc.clone();
                                                let selections = parallel_selections.clone();
                                                let this_doc_selected = this_doc_selected.clone();
                                                move |ev: Event| {
                                                    let checked = event_target_checked(ev);
                                                    let key = (row_idx, col_idx);
                                                    if checked {
                                                        this_doc_selected.set(true);
                                                        selections.update(|selections| { selections.insert(key, doc.clone()); });
                                                    } else {
                                                        this_doc_selected.set(false);
                                                        selections.update(|selections| { selections.remove(&key); });
                                                    }
                                                }
                                            }
                                        />
                                    };

                                    view! {
                                        <>
                                            {checkbox}
                                            {DocumentController::new(doc).view(locale)}
                                        </>
                                    }
                                }
                            };

                            view! {
                                <dyn:td colspan={width.to_string()} class:selected={this_doc_selected.stream().boxed_local()}>{view}</dyn:td>
                            }
                        })
                        .collect(),
                );
                view! {
                    <tr>{cols}</tr>
                }
            })
            .collect::<Vec<_>>();

    let display_settings_menu = DisplaySettingsSideMenu::new();
    let side_menu = side_menu(
        Icon::Settings,
        view! {
            <>
                <h2>{t!("settings.display_settings.title")}</h2>
                {display_settings_menu.component.view()}
            </>
        },
    );

    let alert = Alert::new(parallel_exports(parallels, &parallel_selections));

    let select_button = {
        view! {
            <>
                <dyn:button
                    on:click={
                        let selecting = selecting.clone();
                        let alert_is_open = alert.is_open.clone();
                        move |_ev: Event| {
                            // if currently selecting, show export alert
                            if selecting.get() {
                                alert_is_open.set(true);
                            }
                            // otherwise, show checkboxes
                            else {
                                selecting.set(true);
                            }
                        }
                    }
                >
                    <dyn:img src={Icon::Checkbox.to_string()} class:hidden={selecting.stream().boxed_local()} />
                    <dyn:span class="screen-reader-only" class:hidden={selecting.stream().boxed_local()}>{t!("export.select")}</dyn:span>
                    <dyn:img src={Icon::Download.to_string()} class="hidden" class:hidden={selecting.stream().map(|n| !n).boxed_local()} />
                    <dyn:span class="hidden screen-reader-only" class:hidden={selecting.stream().map(|n| !n).boxed_local()}>{t!("export.export")}</dyn:span>
                </dyn:button>
                {alert.view()}
            </>
        }
    };

    let initial_text = View::Fragment(
        initial_text
            .split("\n\n")
            .map(|para| view! { <p class="initial-text">{para}</p> })
            .collect(),
    );

    view! {
        <>
            {header_with_side_menu_and_buttons(locale, label, side_menu, [select_button])}
            <dyn:main
                class="parallels"
                class={display_settings_menu.current_settings().stream().map(|settings| format!("parallels {}", settings.to_class())).boxed_local()}
            >
                {breadcrumbs(locale, base_slug)}
                {initial_text}
                <table>
                    {parallels_view}
                </table>
            </dyn:main>
        </>
    } */
}
