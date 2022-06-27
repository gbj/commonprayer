use std::{collections::HashMap, sync::Arc};

use ::liturgy::{PreferenceKey, PreferenceValue, Slug, Version};
use leptos2::{http::Response, *};
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
                            href={format!("/{}/liturgy", self.locale)}
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

#[derive(Default)]
pub struct Settings {
    pub general: GeneralSettings,
    pub display: DisplaySettings,
}

#[derive(Default, Serialize, Deserialize)]
struct LiturgySettings(HashMap<(Slug, Version), HashMap<PreferenceKey, PreferenceValue>>);

impl LiturgySettings {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

struct DBSettings {
    user_id: String,
    general: Value,
    display: Value,
    liturgy: Value,
}

const GENERAL_COOKIE_NAME: &'static str = "general";
const DISPLAY_COOKIE_NAME: &'static str = "display";

impl Settings {
    pub async fn all(req: &Arc<dyn Request>) -> Self {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query_as!(
                DBSettings,
                "SELECT * from user_settings where user_id = $1",
                uid
            )
            .fetch_one(req.db())
            .await
            {
                Ok(DBSettings {
                    general, display, ..
                }) => {
                    match (
                        from_value::<GeneralSettings>(general),
                        from_value::<DisplaySettings>(display),
                    ) {
                        (Ok(general), Ok(display)) => Some(Self { general, display }),
                        (Err(e), Ok(display)) => {
                            eprintln!("[Settings::all — settings.general JSON error] {}", e);
                            Some(Self {
                                general: GeneralSettings::default(),
                                display,
                            })
                        }
                        (Ok(general), Err(e)) => {
                            eprintln!("[Settings::all — settings.display JSON error] {}", e);
                            Some(Self {
                                general,
                                display: DisplaySettings::default(),
                            })
                        }
                        (Err(e_general), Err(e_display)) => {
                            eprintln!(
                                "[Settings::all — settings.general JSON error] {}",
                                e_general
                            );
                            eprintln!(
                                "[Settings::all — settings.display JSON error] {}",
                                e_display
                            );
                            None
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Settings::all] {}", e);
                    None
                }
            }
        } else {
            let general = Self::get_prefs_from_cookie(req, GENERAL_COOKIE_NAME).unwrap_or_default();
            let display = Self::get_prefs_from_cookie(req, DISPLAY_COOKIE_NAME).unwrap_or_default();
            Some(Self { general, display })
        }
        .unwrap_or_default()
    }

    pub async fn general(req: &Arc<dyn Request>) -> GeneralSettings {
        if let Some(uid) = UserInfo::verified_id(req.clone()).await {
            match sqlx::query!("SELECT general from user_settings where user_id = $1", uid)
                .fetch_one(req.db())
                .await
            {
                Ok(value) => from_value::<GeneralSettings>(value.general).ok(),
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
            match sqlx::query!("SELECT display from user_settings where user_id = $1", uid)
                .fetch_one(req.db())
                .await
            {
                Ok(value) => from_value::<DisplaySettings>(value.display).ok(),
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

/*  pub async fn general(req: &Arc<dyn Request>) -> Option<GeneralSettings> {
    if let Some(user) = UserInfo::get_untrusted(req) {
        if let Some(uid) = user.verified_uid().await {
            sqlx::query_as!(
                Settings,
                "SELECT * from user_settings where user_id = $1",
                uid
            )
            .fetch_one(req.db())
            .await
            .ok()
        } else {
            None
        }
    } else {
        None
    }
}

pub async fn display(req: &Arc<dyn Request>) -> Option<DisplaySettings> {
    if let Some(user) = UserInfo::get_untrusted(req) {
        if let Some(uid) = user.verified_uid().await {
            sqlx::query_as!(
                Settings,
                "SELECT * from user_settings where user_id = $1",
                uid
            )
            .fetch_one(req.db())
            .await
            .ok()
        } else {
            None
        }
    } else {
        None
    }
} */

/*
#[async_trait(?Send)]
pub trait Settings
where
    Self: Serialize + DeserializeOwned + Default,
{
    fn cookie_name() -> &'static str;

    async fn get_all(req: &Arc<dyn Request>) -> Self {
        /* let headers = req.headers();
        let settings = headers
            .cookies()
            .filter_map(|cookie| match cookie {
                Ok(cookie) => Some(cookie),
                Err(e) => {
                    eprintln!("invalid cookie: {:#?}", e);
                    None
                }
            })
            .find(|cookie| cookie.name() == Self::cookie_name())
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok());
        settings.unwrap_or_default() */

        let settings = sqxl::query_as!(Self, )

    }

    async fn get<T>(req: &Arc<dyn Request>, cb: fn(Self) -> T) -> T {
        let settings = Self::get_all(req).await;
        (cb)(settings)
    }

    async fn set<T>(req: &Arc<dyn Request>, res: &mut http::Response<T>, settings: Self) {
        // set in DB
        let existing_info =

        // set as cookie
        let settings_cookie = cookie::Cookie::build(
            Self::cookie_name(),
            serde_json::to_string(&settings).unwrap(),
        )
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
 */
