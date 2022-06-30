use std::collections::HashMap;

use http::{Response, StatusCode};
use itertools::Itertools;
use leptos2::*;
use liturgy::{
    ClientPreferences, LiturgyPreferences, PreferenceKey, PreferenceValue, Slug, SlugPath, Version,
};
use reqwest::redirect::Action;

use super::Settings;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct LiturgySettings(pub HashMap<(SlugPath, PreferenceKey), PreferenceValue>);

impl LiturgySettings {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct SettingsForLiturgy {
    pub liturgy: SlugPath,
    pub liturgy_prefs: LiturgyPreferences,
    pub client_prefs: HashMap<PreferenceKey, PreferenceValue>,
}

pub struct LiturgySettingsView {
    locale: String,
    settings: SettingsForLiturgy,
    version: Version,
    success: bool,
}

#[derive(Params)]
pub struct LiturgySettingsParams {
    remainder: Option<SlugPath>,
}

#[derive(Params)]
pub struct LiturgySettingsQuery {
    success: Option<String>,
    version: Option<Version>,
}

#[async_trait(?Send)]
impl Loader for LiturgySettingsView {
    type Params = LiturgySettingsParams;
    type Query = LiturgySettingsQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let path = params
            .remainder
            .unwrap_or_else(|| SlugPath::from([Slug::Office, Slug::MorningPrayer]));
        let settings = Settings::liturgy(&req, &path).await?;

        Some(Self {
            locale: locale.to_string(),
            settings,
            version: query.version.unwrap_or(Version::RiteII),
            success: query.success.is_some(),
        })
    }

    async fn action(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> ActionResponse {
        match req.body() {
            Some(raw_body) => {
                let path = params
                    .remainder
                    .unwrap_or_else(|| SlugPath::from([Slug::Office, Slug::MorningPrayer]));
                let form_data = form_urlencoded::parse(raw_body.as_bytes())
                    .map(|(k, v)| {
                        (
                            urlencoding::decode(&k)
                                .map(|k| k.to_string())
                                .unwrap_or_else(|| k.to_string()),
                            urlencoding::decode(&v)
                                .map(|v| v.to_string())
                                .unwrap_or_else(|| v.to_string()),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let mut client_prefs = HashMap::new();
                for (k, v) in form_data {
                    eprintln!("(\n\t{}\n\t{}\n)", k, v);
                    let k: PreferenceKey = match serde_json::from_str(&k) {
                        Ok(k) => k,
                        Err(e) => return ActionResponse::Error(Box::new(e)),
                    };
                    let v: PreferenceValue = match serde_json::from_str(&v) {
                        Ok(v) => v,
                        Err(e) => return ActionResponse::Error(Box::new(e)),
                    };
                    client_prefs.insert(k, v);
                }

                // build response
                let mut res = Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(
                        "Location",
                        format!("/{}/settings/liturgy/{}?success", locale, path),
                    )
                    .body(())
                    .unwrap();

                Settings::set_liturgy(&req, &mut res, path, client_prefs).await;
                ActionResponse::from_response(res)
            }
            None => ActionResponse::None,
        }
        /* let settings = req
            .body()
            .and_then(|body| body.as_form_data::<LiturgySettings>().ok())
            .unwrap_or_default();

        // build response
        let mut res = Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", format!("/{}/settings/display?success", locale))
            .body(())
            .unwrap();

        Settings::set_display(&req, &mut res, settings).await;
        ActionResponse::from_response(res) */
    }
}

impl View for LiturgySettingsView {
    fn title(&self) -> String {
        format!("{} – {}", t!("settings.liturgy"), t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![include_str!("../../styles/toggle-fieldset.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <h2>{t!("settings.liturgy")}</h2>

                <div class="toggle-links">
                    <a href="?version=RiteI" class:current={self.version == Version::RiteI}>{t!("settings.rite_i")}</a>
                    <a href="?version=RiteII" class:current={self.version == Version::RiteII}>{t!("settings.rite_ii")}</a>
                </div>

                <div class="toggle-links">
                    {self.liturgy_setting_link(SlugPath::from([Slug::Office, Slug::MorningPrayer, Slug::Version(self.version)]), t!("toc.morning_prayer"))}
                    {self.liturgy_setting_link(SlugPath::from([Slug::Office, Slug::NoondayPrayer]), t!("toc.noonday_prayer"))}
                    {self.liturgy_setting_link(SlugPath::from([Slug::Office, Slug::EveningPrayer, Slug::Version(self.version)]), t!("toc.evening_prayer"))}
                    {self.liturgy_setting_link(SlugPath::from([Slug::Office, Slug::Compline]), t!("toc.compline"))}
                    {self.liturgy_setting_link(SlugPath::from([Slug::Eucharist, Slug::Eucharist, Slug::Version(self.version)]), t!("toc.holy_eucharist"))}
                </div>

                <form method="post">
                    {self.liturgy_preferences_view()}
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

impl LiturgySettingsView {
    fn liturgy_setting_link(&self, path: SlugPath, label: String) -> Node {
        let href = format!("/{}/settings/liturgy/{}", self.locale, path);
        view! { <a href={href} class:current={path == self.settings.liturgy}>{label}</a> }
    }

    fn liturgy_preferences_view(&self) -> Vec<Node> {
        let categories = self
            .settings
            .liturgy_prefs
            .iter()
            .group_by(|pref| pref.category.as_ref())
            .into_iter()
            .map(|(label, group)| {
                let prefs = group
                    .filter_map(|pref| {
                        let client_pref_here =
                            self.settings.client_prefs.value(&pref.key).or_else(|| {
                                self.settings.liturgy_prefs.default_value_for_key(&pref.key)
                            });

                        if pref.only_one_choice() {
                            None
                        } else {
                            let choices = pref
                                .choices()
                                .enumerate()
                                .map(|(choice_idx, choice)| {
                                    view! {
                                        <option
                                            value={urlencoding::encode(&serde_json::to_string(&choice.value).unwrap())}
                                            selected={client_pref_here == Some(&choice.value)}
                                        >
                                            {&choice.label}
                                        </option>
                                    }
                                })
                                .collect::<Vec<_>>();

                            // TODO value

                            Some(view! {
                                <label class="preference">
                                    {&pref.label}
                                    <select
                                        name={urlencoding::encode(&serde_json::to_string(&pref.key).unwrap())}
                                    >
                                        {choices}
                                    </select>
                                </label>
                            })
                        }
                    })
                    .collect::<Vec<_>>();

                let label = label.map(|label| {
                    view! { <h4>{label}</h4> }
                });

                view! {
                    <div>
                        {label}
                        {prefs}
                    </div>
                }
            })
            .collect::<Vec<_>>();

        categories
    }
}
