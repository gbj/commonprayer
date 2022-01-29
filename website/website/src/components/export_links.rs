use futures::StreamExt;
use leptos::*;

use super::DocumentController;

pub fn export_links(slug: &str, date: &str, document_controller: &DocumentController) -> View {
    let serialized_doc_stream = document_controller
        .stream()
        .map(|doc| serde_json::to_string(&doc).unwrap())
        .boxed_local();

    let json_link_stream = document_controller
        .stream()
        .map(|doc| serde_json::to_string(&doc).unwrap())
        .map(|json| format!("data:application/json,{}", js_sys::encode_uri(&json)))
        .boxed_local();

    view! {
        <ul class="export-links">
            // Link: TODO
            <a class="link" href="#">
                <img src="/static/icons/tabler-icon-link.svg"/>
                {t!("export.link")}
            </a>

            // Embed code: TODO
            <a class="embed" href="#">
                <img src="/static/icons/tabler-icon-code.svg"/>
                {t!("export.embed")}
            </a>

            // Word: posts a hidden form to the server and opens the result in a new tab
            <form class="word" target="_blank" method="post" action="/api/export/docx">
                <input type="hidden" name="liturgy" value={slug}/>
                <input type="hidden" name="date" value={date}/>
                <dyn:input type="hidden" name="doc" value={serialized_doc_stream}/>
                <button type="submit">
                    <img src="/static/icons/file-word-regular.svg"/>
                    {t!("export.word")}
                </button>
            </form>

            // Venite: TODO
            <a class="venite" href="#">
                <img src="/static/icons/venite.svg"/>
                {t!("export.venite")}
            </a>

            // JSON: downloads a named JSON file
            <dyn:a
                class="json"
                download={&format!("{}{}{}.json", slug, if date.is_empty() { "" } else { "-" }, date)}
                href={json_link_stream}
            >
                <img src="/static/icons/tabler-icon-download.svg"/>
                {t!("export.json")}
            </dyn:a>
        </ul>
    }
}
