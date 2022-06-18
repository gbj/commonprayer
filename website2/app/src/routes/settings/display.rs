use http::{Response, StatusCode};
use leptos2::*;

use super::{dark_mode::DarkMode, Settings};

#[derive(Serialize, Deserialize, Params, Default, Debug)]
pub struct DisplaySettings {
    pub dark_mode: DarkMode,
    pub psalm_verse_numbers: bool,
    pub bible_verse_numbers: bool,
}

impl Settings for DisplaySettings {
    fn cookie_name() -> &'static str {
        "DisplaySettings"
    }
}

impl DisplaySettings {
    pub fn to_class(&self) -> String {
        format!(
            "{} {}",
            if self.psalm_verse_numbers {
                ""
            } else {
                "psalm-verses-hidden"
            },
            if self.bible_verse_numbers {
                ""
            } else {
                "bible-verses-hidden"
            }
        )
    }
}

pub struct DisplaySettingsView {
    settings: DisplaySettings,
    success: bool,
}

#[derive(Params)]
pub struct DisplaySettingsQuery {
    success: Option<String>,
}

#[async_trait(?Send)]
impl Loader for DisplaySettingsView {
    type Params = ();
    type Query = DisplaySettingsQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let settings = DisplaySettings::get_all(&req);

        Some(Self {
            settings,
            success: query.success.is_some(),
        })
    }

    async fn action(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> ActionResponse {
        let settings = req
            .body()
            .and_then(|body| body.as_form_data::<DisplaySettings>().ok())
            .unwrap_or_default();

        // build response
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", format!("/{}/settings/display?success", locale))
            .body(())
            .map(|mut res| {
                DisplaySettings::set(&req, &mut res, settings);
                res
            })
            .map(ActionResponse::from_response)
            .unwrap_or_else(ActionResponse::from_error)
    }
}

impl View for DisplaySettingsView {
    fn title(&self) -> String {
        t!("settings.display_settings.title")
    }

    fn styles(&self) -> Styles {
        vec![include_str!("../../styles/toggle-fieldset.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <h2>{t!("settings.display_settings.title")}</h2>
                <form method="post">
                    // Dark Mode
                    <fieldset class="toggle">
                        <legend>{t!("settings.dark_mode.label")}</legend>
                        <input
                            type="radio"
                            id="dm_auto"
                            checked={self.settings.dark_mode == DarkMode::Auto}
                            name="dark_mode"
                            value={DarkMode::Auto}
                        />
                        <label for="dm_auto" title={t!("settings.dark_mode.auto_explanation")}>
                            {t!("settings.dark_mode.auto")}
                        </label>
                        <input
                            type="radio"
                            id="dm_light"
                            checked={self.settings.dark_mode == DarkMode::Light}
                            name="dark_mode"
                            value={DarkMode::Light}
                        />
                        <label for="dm_light" title={t!("settings.dark_mode.light_explanation")}>
                            {t!("settings.dark_mode.light")}
                        </label>
                        <input
                            type="radio"
                            id="dm_dark"
                            checked={self.settings.dark_mode == DarkMode::Dark}
                            name="dark_mode"
                            value={DarkMode::Dark}
                        />
                        <label for="dm_dark" title={t!("settings.dark_mode.dark_explanation")}>
                            {t!("settings.dark_mode.dark")}
                        </label>
                    </fieldset>

                    // Verse Numbers
                    <fieldset class="vertical">
                        <label class="horizontal">
                            {t!("settings.display_settings.bible_verses")}
                            <input type="hidden" name="bible_verse_numbers" value="false"/>
                            <input type="checkbox"
                                checked={self.settings.bible_verse_numbers}
                                value="true"
                                name="bible_verse_numbers"
                            />
                        </label>
                        <label class="horizontal">
                            {t!("settings.display_settings.psalm_verses")}
                            <input type="hidden" name="psalm_verse_numbers" value="false"/>
                            <input type="checkbox"
                                checked={self.settings.psalm_verse_numbers}
                                name="psalm_verse_numbers"
                                value="true"
                            />
                        </label>
                    </fieldset>
                    <input type="submit" value={t!("settings.submit")}/>
                </form>
                {if self.success {
                    view! {
                        <footer class="success">{t!("settings.success")}</footer>
                    }
                } else {
                    Node::default()
                }}
            </div>
        }
    }
}
