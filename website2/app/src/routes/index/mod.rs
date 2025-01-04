use std::pin::Pin;
use std::sync::Arc;

use super::settings::{
    CommonLiturgySettings, DarkMode, GeneralSettings, Settings, SettingsForLiturgy,
};
use crate::components::{Auth, Modal};
use crate::utils::encode_uri;
use crate::utils::time::{today, TimezoneOffset};
use calendar::Date;
use futures::{join, Future};
use leptos2::{view::View, *};
use liturgy::{Slug, SlugPath, Version};

pub mod auth;
use crate::UserInfo;
use auth::auth_scripts;
pub use auth::UserId;

pub struct Index {
    tzoffset: TimezoneOffset,
    locale: String,
    path: String,
    dark_mode: DarkMode,
    general_settings: GeneralSettings,
    user: Option<UserInfo>,
    liturgy_settings: CommonLiturgySettings,
    verified: Pin<Box<dyn Future<Output = bool> + Send + Sync>>,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            tzoffset: TimezoneOffset::default(),
            locale: "en".to_string(),
            path: String::new(),
            dark_mode: DarkMode::Auto,
            general_settings: GeneralSettings::default(),
            user: None,
            liturgy_settings: CommonLiturgySettings::default(),
            verified: Box::pin(async { false }),
        }
    }
}

#[async_trait(?Send)]
impl Loader for Index {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        let settings = Settings::all(&req).await;
        let dark_mode = settings.display.dark_mode;
        let general_settings = settings.general;
        let user = UserInfo::get_untrusted(&req);
        let verified = if let Some(user) = user.clone() {
            Box::pin(async move { user.to_verified_id().await.is_some() })
                as Pin<Box<dyn Future<Output = bool> + Send + Sync>>
        } else {
            Box::pin(async { false })
        };
        let liturgy_settings =
            CommonLiturgySettings::from_req(&req, general_settings.liturgy_version).await;

        Some(Self {
            tzoffset: TimezoneOffset::from(&req),
            locale: locale.to_string(),
            path: req.path().to_string(),
            dark_mode,
            general_settings,
            user,
            liturgy_settings,
            verified,
        })
    }
}

impl View for Index {
    fn title(&self) -> String {
        t!("common_prayer").to_string()
    }

    fn meta(&self) -> MetaTags {
        vec![
            ("charset".to_string(), "UTF-8".to_string()),
            (
                "viewport".to_string(),
                "width=device-width, initial-scale=1.0".to_string(),
            ),
        ]
    }

    fn links(&self) -> Vec<Node> {
        view! {
            <>
                <link rel="preload" href="/static/fonts/Sabon_Roman.woff2" _as="font" type="font/woff2" crossorigin="anonymous" />
                <link rel="preload" href="/static/fonts/Sabon_Bold.woff2" _as="font" type="font/woff2" crossorigin="anonymous" />
                <link rel="preload" href="/static/fonts/Sabon_Italic.woff2" _as="font" type="font/woff2" crossorigin="anonymous" />
                <link rel="preload" href="/static/fonts/Sabon_BoldItalic.woff2" _as="font" type="font/woff2" crossorigin="anonymous" />
            </>
        }
    }

    fn styles(&self) -> view::Styles {
        vec![
            include_str!("vars.css").into(),
            include_str!("general.css").into(),
            include_str!("header.css").into(),
            include_str!("menu.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        let menu = Self::menu(
            &self.tzoffset,
            &self.path,
            &self.locale,
            self.user.as_ref(),
            &self.general_settings,
            self.liturgy_settings,
            self.verified,
        );

        view! {
            <div class={format!("app-shell dark-mode-{}", self.dark_mode).to_lowercase()}>
                {menu}
                {nested_view.unwrap_or_default()}
                {body_scripts()}

                // Store timezone offset
                <script>"document.cookie = `tzoffset=${new Date().getTimezoneOffset()}`"</script>

                // Firebase Auth
                {auth_scripts()}

                // Plausible Analytics
                <script defer data-domain="commonprayeronline.org" src="https://plausible.io/js/plausible.js"></script>
            </div>
        }
    }

    fn error_boundary(error: RouterError) -> Body {
        view! {
            <div class="error-boundary">
                <style>{include_str!("404.css")}</style>
                <main>
                    <h1>{t!("page_404.uh_oh")}</h1>
                    <p>{t!("page_404.explanation")}</p>
                    <section class="links">
                        <a class="email-link" href=format!("mailto:greg@venite.app?subject=(Common Prayer) Error&body={}", encode_uri(&format!("{:#?}", error)))>
                            {t!("page_404.email_greg")}
                        </a>
                        <a class="toc-link" href="/">{t!("page_404.button_text")}</a>
                    </section>
                </main>
            </div>
        }
    }
}

impl Index {
    fn menu(
        tzoffset: &TimezoneOffset,
        path: &str,
        locale: &str,
        user: Option<&UserInfo>,
        settings: &GeneralSettings,
        liturgy_settings: CommonLiturgySettings,
        verified: Pin<Box<dyn Future<Output = bool> + Send + Sync>>,
    ) -> Node {
        let user_logged_in = view! {
            <div class="verified">{(
                view! { <span>"?"</span> },
                async {
                    if verified.await {
                        view! { <span>"✓"</span> }
                    } else {
                        view! { <span>"✕"</span> }
                    }
                }
            )}</div>
        };

        view! {
            <nav id="main-menu" role="navigation" class="menu left">
                // an invisible checkbox that toggles whether the menu appears or not via CSS
                <input id="main-menu-toggle-checkbox" type="checkbox" class="menu-toggle-input"/>

                // label contains the overlay, so that when the overlay is clicked the menu disappears
                <label for="main-menu-toggle-checkbox">
                    <span class="screen-reader-only">{t!("menu.open_menu")}</span>
                    <div class="overlay"></div>
                </label>

                // "hamburger" menu button created via CSS, and positioned over the toggle checkbox
                // (but under it by z-index) so it appears to be a button
                // this can be ignored by a screen reader (see above)
                <div class="menu-toggle-button hamburger" aria-hidden="true">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>

                // Here's the actual content of the navigation menu
                <div class="menu-content">
                    <ul>
                        <li class="title horizontal">
                            <h1>
                                {nav_link(path, locale, "", t!("common_prayer"))}
                            </h1>
                            <Auth
                                prop:user={user.cloned()}
                                data-modal-id="login"
                                loginlabel={t!("auth.title")}
                                logoutlabel={t!("auth.logout")}
                            ></Auth>
                            {user_logged_in}
                            <Modal id="login">
                                <div id="firebase-auth" slot="content"></div>
                            </Modal>
                        </li>
                        <form action={format!("/{}/search", locale)}>
                            <input class="main-search" type="search" name="q" placeholder={t!("search")}/>
                            <noscript><input type="submit" value={t!("search")}/></noscript>
                        </form>
                        <li>
                            {nav_link(path, locale, "/contents", t!("toc.table_of_contents"))}
                        </li>
                        <li>
                            {nav_link(
                                path,
                                locale,
                                if settings.use_lff {
                                    "/calendar"
                                } else {
                                    "/calendar?calendar=bcp"
                                },
                                t!("menu.calendar")
                            )}
                        </li>
                        <li>
                            {nav_link(
                                path,
                                locale,
                                &format!("/readings/office?version={}", settings.bible_version),
                                t!("menu.readings")
                            )}
                        </li>
                        <li>
                            {nav_link(path, locale, "/document/office", t!("toc.daily_office"))}
                            <ul>
                                <li>
                                    {office_link(tzoffset, path, locale, settings, liturgy_settings.mp, Slug::MorningPrayer, t!("toc.morning_prayer"))}
                                </li>
                                <li>
                                    {office_link(tzoffset, path, locale, settings, liturgy_settings.np, Slug::NoondayPrayer, t!("toc.noonday_prayer"))}
                                </li>
                                <li>
                                    {office_link(tzoffset, path, locale, settings, liturgy_settings.ep, Slug::EveningPrayer, t!("toc.evening_prayer"))}
                                </li>
                                <li>
                                    {office_link(tzoffset, path, locale, settings, liturgy_settings.cp, Slug::Compline, t!("toc.compline"))}
                                </li>
                                <li>
                                    {nav_link(path, locale, "/canticle-table", t!("menu.canticle_table"))}
                                </li>
                            </ul>
                        </li>
                        <li>
                            {nav_link(path, locale, "/psalm", t!("menu.psalter"))}
                        </li>
                        <li>
                            {nav_link(path, locale, "/document/prayers-and-thanksgivings", t!("toc.prayers_and_thanksgivings"))}
                        </li>
                        <li>
                            {nav_link(path, locale, "/hymnal", t!("menu.hymnal"))}
                        </li>
                        <li>
                            {nav_link(path, locale, "/meditation", t!("meditation.title"))}
                        </li>
                        <li>
                            {nav_link(path, locale, "/settings", t!("settings.title"))}
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}

fn nav_link(current_url: &str, locale: &str, href: &str, label: impl ToString) -> Node {
    let localized_href = format!("/{}{}", locale, href);
    let active = !href.is_empty() && current_url.starts_with(&localized_href);
    view! {
        <a href={localized_href} class:current={active}>{label.to_string()}</a>
    }
}

fn office_link(
    tzoffset: &TimezoneOffset,
    path: &str,
    locale: &str,
    settings: &GeneralSettings,
    prefs: SettingsForLiturgy,
    slug: Slug,
    label: impl ToString,
) -> Node {
    let version = settings.liturgy_version;
    let slug = slug.slugify();
    let today = today(tzoffset);

    let href = office_link_href(&slug, version, today, prefs);
    nav_link(path, locale, &href, label)
}

fn office_link_href(slug: &str, version: Version, date: Date, prefs: SettingsForLiturgy) -> String {
    format!(
        "/document/office/{}/{:?}?date={}&prefs={}",
        slug,
        version,
        date,
        urlencoding::encode(&prefs.serialize_non_default_prefs())
    )
}
