use std::{collections::HashMap, sync::Arc};

use cookie::Cookie;
use http::{Response, StatusCode};
use leptos2::*;
use liturgy::{Lectionaries, Version};

#[derive(Serialize, Deserialize, Params)]
pub struct GeneralSettings {
    liturgy_version: Version,
    use_lff: bool,
    psalm_cycle: Lectionaries,
    bible_version: Version,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            liturgy_version: Version::RiteII,
            use_lff: true,
            psalm_cycle: Lectionaries::BCP1979DailyOfficePsalms,
            bible_version: Version::NRSV,
        }
    }
}

pub struct GeneralSettingsView {
    settings: GeneralSettings,
    success: bool,
}

#[derive(Params)]
pub struct GeneralSettingsParams {
    success: Option<String>,
}

#[async_trait(?Send)]
impl Loader for GeneralSettingsView {
    type Params = ();
    type Query = GeneralSettingsParams;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let headers = req.headers();
        let settings = headers
            .cookies()
            .filter_map(|cookie| match cookie {
                Ok(cookie) => Some(cookie),
                Err(e) => {
                    eprintln!("invalid cookie: {:#?}", e);
                    None
                }
            })
            .find(|cookie| cookie.name() == "GeneralSettings")
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
            .unwrap_or_default();

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
        // TODO test to make sure Content-Type is actually application/x-www-form-urlencoded
        // read form data
        let general_settings = req
            .body()
            .and_then(|body| body.as_form_data::<GeneralSettings>().ok())
            .unwrap_or_default();
        let settings_cookie = Cookie::build(
            "GeneralSettings",
            serde_json::to_string(&general_settings).unwrap(),
        )
        .path("/")
        .finish();

        // build response
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", format!("/{}/settings?success", locale))
            .header("Set-Cookie", settings_cookie.to_string())
            .body(())
            .map(ActionResponse::from_response)
            .unwrap_or_else(ActionResponse::from_error)
    }
}

impl View for GeneralSettingsView {
    fn title(&self) -> String {
        t!("settings.general")
    }

    fn styles(&self) -> Styles {
        vec![include_str!("../../styles/toggle-fieldset.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        view! {
            <>
                <h2>{t!("settings.general")}</h2>
                <form method="post">
                    // Rite I/Rite II
                    <fieldset class="toggle">
                        <legend>{t!("settings.version")}</legend>
                        <input
                            type="radio"
                            id="rite_i"
                            checked={self.settings.liturgy_version == Version::RiteI}
                            name="liturgy_version"
                            value={Version::RiteI}
                        />
                        <label for="rite_i">{t!("settings.rite_i")}</label>
                        <input
                            type="radio"
                            id="rite_ii"
                            checked={self.settings.liturgy_version != Version::RiteI}
                            name="liturgy_version"
                            value={Version::RiteII}
                        />
                        <label for="rite_ii">{t!("settings.rite_ii")}</label>
                    </fieldset>

                    // BCP 1979/LFF 2018 Calendar
                    <fieldset class="toggle">
                        <legend>{t!("settings.calendar")}</legend>
                        <input
                            type="radio"
                            id="bcp_1979"
                            checked={!self.settings.use_lff}
                            name="use_lff"
                            value="false"
                        />
                        <label for="bcp_1979">{t!("bcp_1979")}</label>
                        <input
                            type="radio"
                            id="lff_2018"
                            checked={self.settings.use_lff}
                            name="use_lff"
                            value="true"
                        />
                        <label for="lff_2018">{t!("lff_2018")}</label>
                    </fieldset>

                    // Psalm Cycle
                    <fieldset class="toggle">
                        <legend>{t!("settings.psalm_cycle")}</legend>
                        <input
                            type="radio"
                            id="daily_office_psalms"
                            checked={self.settings.psalm_cycle == Lectionaries::BCP1979DailyOfficePsalms}
                            name="psalm_cycle"
                            value={Lectionaries::BCP1979DailyOfficePsalms}
                        />
                        <label for="daily_office_psalms">{t!("daily_readings.daily_office_psalms")}</label>
                        <input
                            type="radio"
                            id="thirty_day_psalms"
                            checked={self.settings.psalm_cycle == Lectionaries::BCP1979ThirtyDayPsalms}
                            name="psalm_cycle"
                            value={Lectionaries::BCP1979ThirtyDayPsalms}
                        />
                        <label for="thirty_day_psalms">{t!("daily_readings.thirty_day_psalms")}</label>
                    </fieldset>

                    // Bible Version
                    <fieldset class="toggle">
                        <legend>{t!("settings.bible_version")}</legend>
                        <input
                            type="radio"
                            id="NRSV"
                            checked={self.settings.bible_version == Version::NRSV}
                            name="bible_version"
                            value={Version::NRSV}
                        />
                        <label for="NRSV" title={t!("bible_version.NRSV_full")}>
                            {t!("bible_version.NRSV")}
                        </label>
                        <input
                            type="radio"
                            id="CEB"
                            checked={self.settings.bible_version == Version::CEB}
                            name="bible_version"
                            value={Version::CEB}
                        />
                        <label for="CEB" title={t!("bible_version.CEB_full")}>
                            {t!("bible_version.CEB")}
                        </label>
                        <input
                            type="radio"
                            id="ESV"
                            checked={self.settings.bible_version == Version::ESV}
                            name="bible_version"
                            value={Version::ESV}
                        />
                        <label for="ESV" title={t!("bible_version.ESV_full")}>
                            {t!("bible_version.ESV")}
                        </label>
                        <input
                            type="radio"
                            id="KJV"
                            checked={self.settings.bible_version == Version::KJV}
                            name="bible_version"
                            value={Version::KJV}
                        />
                        <label for="KJV" title={t!("bible_version.KJV_full")}>
                            {t!("bible_version.KJV")}
                        </label>
                        <input
                            type="radio"
                            id="RV"
                            checked={self.settings.bible_version == Version::RV09}
                            name="bible_version"
                            value={Version::RV09}
                        />
                        <label for="RV" title={t!("bible_version.RV_full")}>
                            {t!("bible_version.RV")}
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
            </>
        }
    }
}
