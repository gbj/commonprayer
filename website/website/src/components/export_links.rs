use std::time::Duration;

use futures::StreamExt;
use leptos::*;

use crate::{components::Toggle, pages::DocumentPageProps, utils::share};

use super::DocumentController;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    Idle,
    Success,
    Error(String),
}

pub fn export_links(props: &DocumentPageProps, document_controller: &DocumentController) -> View {
    let slug = props.slug.clone().unwrap_or_else(|| String::from("export"));

    let status = Behavior::new(Status::Idle);

    // whether to export only selections, or whole document
    let (selections_toggle_view, use_whole_doc) =
        if let Some(selections) = &document_controller.selections {
            let hide_toggle = selections
                .stream()
                .map(|selections| selections.is_empty())
                .boxed_local();

            let toggle = Toggle::new(
                false,
                "use_selections",
                t!("export.selections"),
                t!("export.whole_doc"),
                None,
            );
            let view = view! {
                <dyn:div class="hidden" class:hidden={hide_toggle}>
                    {toggle.view()}
                </dyn:div>
            };
            (view, toggle.toggled)
        } else {
            (View::Empty, Behavior::new(true))
        };

    // content for different export options

    // for .docx form
    let serialized_doc_stream = use_whole_doc
        .stream()
        .flat_map({
            let document_controller = document_controller.clone();
            move |use_whole_doc| {
                if use_whole_doc {
                    document_controller
                        .stream()
                        .map(|doc| serde_json::to_string(&doc).unwrap())
                        .boxed_local()
                } else {
                    document_controller
                        .filtered_stream()
                        .map(|doc| serde_json::to_string(&doc).unwrap())
                        .boxed_local()
                }
            }
        })
        .boxed_local();

    // for JSON link
    let json_link_stream = use_whole_doc
        .stream()
        .flat_map({
            let document_controller = document_controller.clone();
            move |use_whole_doc| {
                if use_whole_doc {
                    document_controller
                        .stream()
                        .map(|doc| serde_json::to_string(&doc).unwrap())
                        .map(|json| format!("data:application/json,{}", js_sys::encode_uri(&json)))
                        .boxed_local()
                } else {
                    document_controller
                        .filtered_stream()
                        .map(|doc| serde_json::to_string(&doc).unwrap())
                        .map(|json| format!("data:application/json,{}", js_sys::encode_uri(&json)))
                        .boxed_local()
                }
            }
        })
        .boxed_local();

    // in case of error, text to manually copy to clipboard
    let text_to_copy = status
        .stream()
        .map(|status| {
            if let Status::Error(text_to_copy) = status {
                text_to_copy
            } else {
                String::default()
            }
        })
        .boxed_local();

    view! {
        <>
            <main class="export-alert">
                {selections_toggle_view}
                <ul class="export-links">
                    // Link
                    <dyn:button class="link" on:click={
                        let status = status.clone();
                        move |_ev: Event| share_link(&status)
                    }>
                        <img src="/static/icons/tabler-icon-link.svg"/>
                        {t!("export.link")}
                    </dyn:button>

                    // Embed code: TODO
                    /* <a class="embed" href="#">
                        <img src="/static/icons/tabler-icon-code.svg"/>
                        {t!("export.embed")}
                    </a> */

                    // Word: posts a hidden form to the server and opens the result in a new tab
                    <form class="word" target="_blank" method="post" action="/api/export/docx">
                        <input type="hidden" name="liturgy" value={&slug}/>
                        <input type="hidden" name="date" value={&props.date}/>
                        <dyn:input type="hidden" name="doc" value={serialized_doc_stream}/>
                        <button type="submit">
                            <img src="/static/icons/file-word-regular.svg"/>
                            {t!("export.word")}
                        </button>
                    </form>

                    // Venite: TODO
                    <dyn:button class="link venite" on:click={
                        let status = status.clone();
                        let ctrl = document_controller.clone();
                        move |_ev: Event| copy_ldf(&status, &ctrl, &use_whole_doc)
                    }>
                        <img src="/static/icons/venite.svg"/>
                        {t!("export.venite")}
                    </dyn:button>

                    // JSON: downloads a named JSON file
                    <dyn:a
                        class="json"
                        download={&format!("{}{}{}.json", slug, if props.date.is_empty() { "" } else { "-" }, props.date)}
                        href={json_link_stream}
                    >
                        <img src="/static/icons/tabler-icon-download.svg"/>
                        {t!("export.json")}
                    </dyn:a>
                </ul>
            </main>
            <footer class="export-status">
                <dyn:p class="success" class:hidden={status.stream().map(|status| status != Status::Success).boxed_local()}>
                    {t!("export.clipboard_success")}
                </dyn:p>
                <dyn:p class="error" class:hidden={status.stream().map(|status| !matches!(status, Status::Error(_))).boxed_local()}>
                    {t!("export.clipboard_error")}
                    <dyn:pre>{text_to_copy}</dyn:pre>
                </dyn:p>
            </footer>
        </>
    }
}

fn copy_ldf(status: &Behavior<Status>, ctrl: &DocumentController, use_whole_doc: &Behavior<bool>) {
    let doc = if use_whole_doc.get() {
        ctrl.get_state()
    } else {
        ctrl.get_state_filtered_by_selections()
    };

    let json = ldf::LdfJson::from(doc);
    copy(serde_json::to_string(&json.into_inner()), status);
}

fn share_link(status: &Behavior<Status>) {
    copy(location().href(), status)
}

fn copy<E>(text: Result<String, E>, status: &Behavior<Status>) {
    match text {
        Ok(value) => {
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match share::copy_text(&value).await {
                    Ok(_) => {
                        status.set(Status::Success);
                        set_timeout(
                            {
                                let status = status.clone();
                                move || status.set(Status::Idle)
                            },
                            Duration::from_secs(2),
                        );
                    }
                    Err(_) => {
                        status.set(Status::Error(String::default()));
                        set_timeout(
                            {
                                let status = status.clone();
                                move || status.set(Status::Idle)
                            },
                            Duration::from_secs(10),
                        );
                    }
                }
            });
        }
        Err(_) => {
            status.set(Status::Error(String::default()));
            set_timeout(
                {
                    let status = status.clone();
                    move || status.set(Status::Idle)
                },
                Duration::from_secs(10),
            );
        }
    }
}
