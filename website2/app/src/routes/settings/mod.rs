use std::sync::Arc;

use leptos2::*;
use serde::de::DeserializeOwned;

mod dark_mode;
mod display;
mod general;
mod liturgy;

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

pub trait Settings
where
    Self: Serialize + DeserializeOwned + Default,
{
    fn cookie_name() -> &'static str;

    fn get_all(req: &Arc<dyn Request>) -> Self {
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
            .find(|cookie| cookie.name() == Self::cookie_name())
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok());
        settings.unwrap_or_default()
    }

    fn get<T>(req: &Arc<dyn Request>, cb: fn(Self) -> T) -> T {
        let settings = Self::get_all(req);
        (cb)(settings)
    }

    fn set<T>(req: &Arc<dyn Request>, res: &mut http::Response<T>, settings: Self) {
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
