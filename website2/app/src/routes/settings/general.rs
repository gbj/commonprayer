use leptos2::*;
use liturgy::Version;

pub struct GeneralSettingsView {
    rite_i: bool,
    bcp_calendar: bool,
    thirty_day_psalms: bool,
    bible_version: Version,
}

#[async_trait]
impl Loader for GeneralSettingsView {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        Some(Self {
            rite_i: false,
            bcp_calendar: false,
            thirty_day_psalms: false,
            bible_version: Version::NRSV,
        })
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
                            checked={self.rite_i}
                        />
                        <label for="rite_i">{t!("settings.rite_i")}</label>
                        <input
                            type="radio"
                            id="rite_ii"
                            checked={!self.rite_i}
                        />
                        <label for="rite_ii">{t!("settings.rite_ii")}</label>
                    </fieldset>

                    // BCP 1979/LFF 2018 Calendar
                    <fieldset class="toggle">
                        <legend>{t!("settings.calendar")}</legend>
                        <input
                            type="radio"
                            id="bcp_1979"
                            checked={self.bcp_calendar}
                        />
                        <label for="bcp_1979">{t!("bcp_1979")}</label>
                        <input
                            type="radio"
                            id="lff_2018"
                            checked={!self.bcp_calendar}
                        />
                        <label for="lff_2018">{t!("lff_2018")}</label>
                    </fieldset>

                    // Psalm Cycle
                    <fieldset class="toggle">
                        <legend>{t!("settings.psalm_cycle")}</legend>
                        <input
                            type="radio"
                            id="daily_office_psalms"
                            checked={!self.thirty_day_psalms}
                        />
                        <label for="daily_office_psalms">{t!("daily_readings.daily_office_psalms")}</label>
                        <input
                            type="radio"
                            id="thirty_day_psalms"
                            checked={self.thirty_day_psalms}
                        />
                        <label for="thirty_day_psalms">{t!("daily_readings.thirty_day_psalms")}</label>
                    </fieldset>

                    // Bible Version
                    <fieldset class="toggle">
                        <legend>{t!("settings.bible_version")}</legend>
                        <input
                            type="radio"
                            id="NRSV"
                            checked={self.bible_version == Version::NRSV}
                        />
                        <label for="NRSV" title={t!("bible_version.NRSV_full")}>
                            {t!("bible_version.NRSV")}
                        </label>
                        <input
                            type="radio"
                            id="CEB"
                            checked={self.bible_version == Version::CEB}
                        />
                        <label for="CEB" title={t!("bible_version.CEB_full")}>
                            {t!("bible_version.CEB")}
                        </label>
                        <input
                            type="radio"
                            id="ESV"
                            checked={self.bible_version == Version::ESV}
                        />
                        <label for="ESV" title={t!("bible_version.ESV_full")}>
                            {t!("bible_version.ESV")}
                        </label>
                        <input
                            type="radio"
                            id="KJV"
                            checked={self.bible_version == Version::KJV}
                        />
                        <label for="KJV" title={t!("bible_version.KJV_full")}>
                            {t!("bible_version.KJV")}
                        </label>
                        <input
                            type="radio"
                            id="RV"
                            checked={self.bible_version == Version::RV09}
                        />
                        <label for="RV" title={t!("bible_version.RV_full")}>
                            {t!("bible_version.RV")}
                        </label>
                    </fieldset>
                    <input type="submit" value={t!("settings.submit")}/>
                </form>
            </>
        }
    }
}
