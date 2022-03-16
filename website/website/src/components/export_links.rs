use futures::StreamExt;
use leptos::*;
use wasm_bindgen_futures::JsFuture;

use crate::{pages::DocumentPageProps, utils::share};

use super::DocumentController;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    Idle,
    Success,
    Error(String),
}

pub fn export_links(props: &DocumentPageProps, document_controller: &DocumentController) -> View {
    let path = props.path.clone();
    let slug = props.slug.clone().unwrap_or_else(|| String::from("export"));

    let status = Behavior::new(Status::Idle);

    let serialized_doc_stream = document_controller
        .stream()
        .map(|doc| serde_json::to_string(&doc).unwrap())
        .boxed_local();

    let json_link_stream = document_controller
        .stream()
        .map(|doc| serde_json::to_string(&doc).unwrap())
        .map(|json| format!("data:application/json,{}", js_sys::encode_uri(&json)))
        .boxed_local();

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
            <main>
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
                    /* <a class="venite" href="#">
                        <img src="/static/icons/venite.svg"/>
                        {t!("export.venite")}
                    </a> */

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

fn share_link(status: &Behavior<Status>) {
    match location().href() {
        Ok(path) => {
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match share::copy_text(&path).await {
                    Ok(_) => status.set(Status::Success),
                    Err(_) => status.set(Status::Error(path)),
                }
            });
        }
        Err(_) => status.set(Status::Error(String::default())),
    }
}
