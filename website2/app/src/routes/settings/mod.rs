use std::{collections::HashMap, sync::Arc};

use ::liturgy::{Content, PreferenceKey, PreferenceValue, SlugPath};
use lazy_static::lazy_static;
use leptos2::{http::Response, *};
use library::{CommonPrayer, Contents, Library};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};

mod dark_mode;
mod display;
mod general;
mod liturgy;

use crate::UserInfo;

pub use self::liturgy::*;
pub use dark_mode::DarkMode;
pub use display::*;
pub use general::*;

pub struct SettingsView {
    locale: String,
    path: String,
}

#[async_trait(?Send)]
impl Loader for SettingsView {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        Some(Self {
            locale: locale.to_string(),
            path: req.path().to_string(),
        })
    }
}

impl View for SettingsView {
    fn title(&self) -> String {
        t!("settings.title")
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("settings.css").into(),
            include_str!("../../styles/toggle-links.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <header><h1>{t!("settings.title")}</h1></header>
                <main>
                    <div class="toggle-links">
                        <a
                            href={format!("/{}/settings", self.locale)}
                            class:current={self.path.ends_with("settings")}
                        >
                            {t!("settings.general")}
                        </a>
                         <a
                            href={format!("/{}/settings/display", self.locale)}
                            class:current={self.path.ends_with("display")}
                        >
                            {t!("settings.display_settings.title")}
                        </a>
                        <a
                            href={format!("/{}/settings/liturgy", self.locale)}
                            class:current={self.path.contains("/liturgy")}
                        >
                            {t!("settings.liturgy")}
                        </a>
                    </div>
                    {nested_view}
                </main>
            </div>
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Settings {
    pub general: GeneralSettings,
    pub display: DisplaySettings,
}

struct DBSettings {
    user_id: String,
    general: Value,
    display: Value,
    liturgy: Value,
}

const GENERAL_COOKIE_NAME: &str = "general";
const DISPLAY_COOKIE_NAME: &str = "display";
const LITURGY_COOKIE_NAME: &str = "liturgy";

lazy_static! {
    pub static ref ALL_SETTINGS_CACHE: moka::sync::Cache<String, Settings> =
        moka::sync::Cache::builder()
            .time_to_live(std::time::Duration::from_millis(500))
            .build();
    pub static ref GENERAL_SETTINGS_CACHE: moka::sync::Cache<String, GeneralSettings> =
        moka::sync::Cache::builder()
            .time_to_live(std::time::Duration::from_millis(500))
            .build();
    pub static ref DISPLAY_SETTINGS_CACHE: moka::sync::Cache<String, DisplaySettings> =
        moka::sync::Cache::builder()
            .time_to_live(std::time::Duration::from_millis(500))
            .build();
    pub static ref LITURGY_SETTINGS_CACHE: moka::sync::Cache<(String, SlugPath), Vec<(PreferenceKey, PreferenceValue)>> =
        moka::sync::Cache::builder()
            .time_to_live(std::time::Duration::from_millis(500))
            .build();
}

impl Settings {
    pub async fn all(req: &Arc<dyn Request>) -> Settings {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            if let Some(cached) = ALL_SETTINGS_CACHE.get(&uid) {
                return cached;
            }

            match sqlx::query_as!(
                DBSettings,
                "SELECT * from user_settings where user_id = $1",
                uid
            )
            .fetch_one(req.db())
            .await
            {
                Ok(DBSettings {
                    general,
                    display,
                    liturgy,
                    ..
                }) => {
                    let general = from_value::<GeneralSettings>(general).unwrap_or_default();
                    let display = from_value::<DisplaySettings>(display).unwrap_or_default();

                    let from_db = Settings { general, display };
                    ALL_SETTINGS_CACHE.insert(uid, from_db.clone());
                    Some(from_db)
                }
                Err(e) => {
                    eprintln!("[Settings::all] {}", e);
                    None
                }
            }
        } else {
            let general =
                Settings::get_prefs_from_cookie(req, GENERAL_COOKIE_NAME).unwrap_or_default();
            let display =
                Settings::get_prefs_from_cookie(req, DISPLAY_COOKIE_NAME).unwrap_or_default();
            Some(Settings { general, display })
        }
        .unwrap_or_default()
    }

    pub async fn general(req: &Arc<dyn Request>) -> GeneralSettings {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            if let Some(cached) = GENERAL_SETTINGS_CACHE.get(&uid) {
                return cached;
            }

            match sqlx::query!("SELECT general from user_settings where user_id = $1", uid)
                .fetch_one(req.db())
                .await
            {
                Ok(value) => {
                    let from_db = from_value::<GeneralSettings>(value.general).ok();
                    if let Some(from_db) = &from_db {
                        GENERAL_SETTINGS_CACHE.insert(uid, from_db.clone());
                    };
                    from_db
                }
                Err(e) => {
                    eprintln!("[Settings::general] {}", e);
                    None
                }
            }
        } else {
            Self::get_prefs_from_cookie(req, GENERAL_COOKIE_NAME)
        }
        .unwrap_or_default()
    }

    pub async fn display(req: &Arc<dyn Request>) -> DisplaySettings {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            if let Some(cached) = DISPLAY_SETTINGS_CACHE.get(&uid) {
                return cached;
            }

            match sqlx::query!("SELECT display from user_settings where user_id = $1", uid)
                .fetch_one(req.db())
                .await
            {
                Ok(value) => {
                    let from_db = from_value::<DisplaySettings>(value.display).ok();
                    if let Some(from_db) = &from_db {
                        DISPLAY_SETTINGS_CACHE.insert(uid, from_db.clone());
                    };
                    from_db
                }
                Err(e) => {
                    eprintln!("[Settings::display] {}", e);
                    None
                }
            }
        } else {
            Self::get_prefs_from_cookie(req, DISPLAY_COOKIE_NAME)
        }
        .unwrap_or_default()
    }

    pub async fn liturgy(req: &Arc<dyn Request>, path: SlugPath) -> Option<SettingsForLiturgy> {
        let contents = CommonPrayer::contents().contents_at_path(&path)?;
        let liturgy_prefs = if let Contents::Document(doc) = contents {
            if let Content::Liturgy(liturgy) = &doc.content {
                Some(liturgy.preferences.clone())
            } else {
                None
            }
        } else {
            None
        }?;

        let client_prefs = if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            if let Some(cached) = LITURGY_SETTINGS_CACHE.get(&(uid.clone(), path.clone())) {
                Some(cached)
            } else {
                match sqlx::query!(
                    "SELECT prefs from liturgy_settings where user_id_and_liturgy = $1",
                    format!("{}-{}", uid, path.to_string())
                )
                .fetch_one(req.db())
                .await
                {
                    Ok(value) => {
                        let from_db =
                            from_value::<Vec<(PreferenceKey, PreferenceValue)>>(value.prefs).ok();
                        if let Some(from_db) = &from_db {
                            LITURGY_SETTINGS_CACHE.insert((uid, path.clone()), from_db.clone());
                        };
                        from_db
                    }
                    Err(e) => {
                        eprintln!("[Settings::liturgy] {}", e);
                        None
                    }
                }
            }
        } else {
            Self::get_prefs_from_cookie(req, &format!("{}-{}", LITURGY_COOKIE_NAME, path))
        }
        .unwrap_or_default()
        .into_iter()
        .collect::<HashMap<_, _>>();

        Some(SettingsForLiturgy {
            liturgy: path.clone(),
            liturgy_prefs,
            client_prefs,
        })
    }

    async fn set_display(
        req: &Arc<dyn Request>,
        res: &mut Response<()>,
        settings: DisplaySettings,
    ) {
        // store in database for users who are logged in
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query!("INSERT INTO user_settings VALUES ($1, $2, $3, $4) ON CONFLICT (user_id) DO UPDATE SET display = $3;",
                uid,
                serde_json::to_value(&GeneralSettings::default()).unwrap(),
                serde_json::to_value(&settings).unwrap(),
                serde_json::to_value(&LiturgySettings::default()).unwrap()
            )
            .execute(req.db())
            .await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[Settings::set_display] {}", e)
                }
            }
        }
        // store in cookies for users not logged in
        else {
            Self::store_prefs_in_cookie(
                req,
                res,
                DISPLAY_COOKIE_NAME,
                serde_json::to_string(&settings).unwrap(),
            )
        }
    }

    async fn set_general(
        req: &Arc<dyn Request>,
        res: &mut Response<()>,
        settings: GeneralSettings,
    ) {
        // store in database for users who are logged in
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query!("INSERT INTO user_settings VALUES ($1, $2, $3, $4) ON CONFLICT (user_id) DO UPDATE SET general = $2;",
                uid,
                serde_json::to_value(&settings).unwrap(),
                serde_json::to_value(&DisplaySettings::default()).unwrap(),
                serde_json::to_value(&LiturgySettings::default()).unwrap()
            )
            .execute(req.db())
            .await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[Settings::set_display] {}", e)
                }
            }

            // update cached values
            GENERAL_SETTINGS_CACHE.invalidate(&uid);
        }
        // store in cookies for users not logged in
        else {
            Self::store_prefs_in_cookie(
                req,
                res,
                GENERAL_COOKIE_NAME,
                serde_json::to_string(&settings).unwrap(),
            )
        }
    }

    async fn set_liturgy(
        req: &Arc<dyn Request>,
        res: &mut Response<()>,
        liturgy: SlugPath,
        settings: HashMap<PreferenceKey, PreferenceValue>,
    ) {
        // because Serde JSON Map keys must be strings
        let settings = settings.into_iter().collect::<Vec<_>>();

        // store in database for users who are logged in
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query!("INSERT INTO liturgy_settings VALUES ($1, $2) ON CONFLICT (user_id_and_liturgy) DO UPDATE SET prefs = $2;",
                format!("{}-{}", uid, liturgy),
                serde_json::to_value(&settings).unwrap(),
            )
            .execute(req.db())
            .await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[Settings::set_liturgy] {}", e)
                }
            }

            // update cached values
            LITURGY_SETTINGS_CACHE.invalidate(&(uid, liturgy));
        }
        // store in cookies for users not logged in
        else {
            Self::store_prefs_in_cookie(
                req,
                res,
                &format!("{}-{}", LITURGY_COOKIE_NAME, liturgy),
                serde_json::to_string(&settings).unwrap(),
            )
        }
    }

    fn get_prefs_from_cookie<T: Default + DeserializeOwned>(
        req: &Arc<dyn Request>,
        cookie_name: &str,
    ) -> Option<T> {
        req.headers()
            .cookies()
            .filter_map(|cookie| match cookie {
                Ok(cookie) => Some(cookie),
                Err(e) => {
                    eprintln!("invalid cookie: {:#?}", e);
                    None
                }
            })
            .find(|cookie| cookie.name() == cookie_name)
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
    }

    fn store_prefs_in_cookie(
        req: &Arc<dyn Request>,
        res: &mut Response<()>,
        cookie_name: &str,
        json: String,
    ) {
        let settings_cookie = cookie::Cookie::build(cookie_name, json)
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(cookie::time::Duration::days(365_000))
            .finish();
        res.headers_mut().insert(
            "Set-Cookie",
            http::HeaderValue::from_str(&settings_cookie.to_string()).unwrap(),
        );
    }
}
