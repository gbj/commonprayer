use super::breadcrumbs::breadcrumbs;
use super::{DocumentPage, DocumentPageParams, ExportLinks};
use crate::views::document::DocumentView;
use crate::{
    components::DatePicker,
    views::{side_menu, Header, Icon},
    WebView,
};
use leptos2::*;
use liturgy::{Document, SlugPath};

pub fn document_body(
    locale: &str,
    base_slug: &SlugPath,
    document: &Document,
    props: &DocumentPage,
    params: &DocumentPageParams,
) -> Vec<Node> {
    /* let date_picker = DatePicker::new(t!("date"), None);
    date_picker
        .date
        .stream()
        // skip the first value, because the initial value of the input will
        // always be emitted but has already been reflected in the page
        .skip(1)
        .create_effect({
            let base_slug = base_slug.clone();
            let params = params.clone();
            let language = document.language;
            let version = document.version;
            let locale = locale.to_string();
            move |date| {
                if let Some(date) = date {
                    let mut params = params.clone();
                    params.date = Some(date);
                    redirect_with_new_preferences(
                        &locale,
                        &Behavior::new(Status::Idle),
                        &base_slug,
                        language,
                        version,
                        &params,
                    )
                } else {
                    location().set_href(&base_slug.to_string()).unwrap_throw();
                }
            }
        });

    let date_menu = if document.has_date_condition() {
        view! {
            <section class="preview-menu">
                <view view={date_picker.view()}/>
            </section>
        }
    } else {
        View::Empty
    };
    let display_settings_menu = DisplaySettingsSideMenu::new();

    let liturgy_preference_menu = liturgy_preferences_view(
        &display_settings_menu.status,
        base_slug,
        document.language,
        document.version,
        &liturgy_to_preferences(document),
    );
    let liturgy_preference_menu = view! {
        <section class="liturgy-preferences">
            <h2>{t!("settings.liturgy")}</h2>
            {liturgy_preference_menu}
            <button
                on:click={
                    let params = params.clone();
                    let language = document.language;
                    let version = document.version;
                    let status = display_settings_menu.status.clone();
                    let locale = locale.to_string();
                    let base_slug = base_slug.clone();
                    move |_ev: Event| redirect_with_new_preferences(&locale, &status, &base_slug, language, version, &params)
                }
            >
                {t!("settings.use_preferences")}
            </button>
        </section>
    };

    let selections = Behavior::new(HashSet::new());
    let document_controller = DocumentController::new_with_selections(document.clone(), selections);
    document_controller.selection_mode.set(SelectionMode::Auto);

    let side_menu = side_menu(
        Icon::Settings,
        view! {
            <>
                {date_menu}

                <h2>{t!("settings.display_settings.title")}</h2>
                {display_settings_menu.component.view()}

                {liturgy_preference_menu}
            </>
        },
    );

    // export button and overlay
    let export_alert = Alert::new(export_links(props, &document_controller));
    let export_button = {
        let is_open = export_alert.is_open.clone();
        view! {
            <button on:click=move |_ev: Event| is_open.set(!is_open.get())>
                <img src={Icon::Link.to_string()}/>
                <span class="screen-reader-only">{t!("export.export")}</span>
            </button>
        }
    }; */

    let contains_parallels = document.contains_parallels();

    let date_menu = if document.has_date_condition() {
        Some(view! {
            <section class="preview-menu">
                <DatePicker label={t!("date")} todaylabel={t!("today")} />
            </section>
        })
    } else {
        None
    };

    let export_button = view! { <ExportLinks fake="Fake attr" prop:document={document.clone()}/> };
    let display_settings_menu = view! { <pre>"TODO display_settings_menu" </pre> };
    let liturgy_preference_menu = view! { <pre>"TODO liturgy_preference_menu" </pre> };

    let side_menu = side_menu(
        Icon::Settings,
        view! {
            <div>
                {date_menu}

                <h2>{t!("settings.display_settings.title")}</h2>
                {display_settings_menu}

                {liturgy_preference_menu}
            </div>
        },
    );

    let viewer = DocumentView {
        doc: document,
        path: vec![],
    };

    view! {
        <>
            {Header::new_with_side_menu_and_additional_buttons(locale, &document.label.clone().unwrap_or_default(), side_menu, vec![export_button]).to_node()}
            <main
                // TODO display settings class
            >
                {breadcrumbs(locale, base_slug)}
                {viewer.view(locale)}
            </main>
        </>
    }
}
