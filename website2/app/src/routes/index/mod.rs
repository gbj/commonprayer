use std::sync::Arc;

use super::settings::{DarkMode, DisplaySettings, GeneralSettings, Settings};
use crate::components::{Auth, Modal};
use crate::utils::encode_uri;
use leptos2::{view::View, *};

pub mod auth;
use auth::{auth_scripts, UserInfo};

pub struct Index {
    locale: String,
    path: String,
    dark_mode: DarkMode,
    general_settings: GeneralSettings,
    user: Option<UserInfo>,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            path: String::new(),
            dark_mode: DarkMode::Auto,
            general_settings: GeneralSettings::default(),
            user: None,
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
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let dark_mode = DisplaySettings::get(&req, |settings| settings.dark_mode);

        let general_settings = GeneralSettings::get_all(&req);
        let user = UserInfo::get_untrusted(&req);
        
        if let Some(user) = &user {
                    eprintln!("validating token");
        let token = auth::validate_token(&user.token).await;
        match token {
            Ok(token) =>         eprintln!("token result = {{\n\theaders: {:?},\n\tclaims: {:?}}}", token.headers, token.claims),
            Err(e) => eprintln!("error {:#?}", e)
        }
        }


        Some(Self {
            locale: locale.to_string(),
            path: req.path().to_string(),
            dark_mode,
            general_settings,
            user,
        })
    }
}

impl View for Index {
    fn title(&self) -> String {
        t!("common_prayer")
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
                <link rel="preload" href="/static/fonts/Sabon_Roman.woff2" _as="font" type="font/woff2"/>
                <link rel="preload" href="/static/fonts/Sabon_Bold.woff2" _as="font" type="font/woff2"/>
                <link rel="preload" href="/static/fonts/Sabon_Italic.woff2" _as="font" type="font/woff2"/>
                <link rel="preload" href="/static/fonts/Sabon_BoldItalic.woff2" _as="font" type="font/woff2"/>
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
        let menu = self.menu();

        view! {
            <div class={format!("app-shell dark-mode-{}", self.dark_mode).to_lowercase()}>
                {menu}
                {nested_view.unwrap_or_default()}
                {body_scripts()}

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
    fn menu(&self) -> Node {
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
                                {nav_link(&self.path, &self.locale, "", t!("common_prayer"))}
                            </h1>
                            <Auth prop:user={self.user.clone()} data-modal-id="login"></Auth>
                            <Modal id="login">
                                <div id="firebase-auth" slot="content"></div>
                            </Modal>
                        </li>
                        <form action="search">
                            <input class="main-search" type="search" name="q" placeholder={t!("search")}/>
                            <noscript><input type="submit" value={t!("search")}/></noscript>
                        </form>
                        <li>
                            {nav_link(&self.path, &self.locale, "/contents", t!("toc.table_of_contents"))}
                        </li>
                        <li>
                            {nav_link(&self.path, &self.locale, "/calendar", t!("menu.calendar"))}
                        </li>
                        <li>
                            {nav_link(&self.path, &self.locale, "/readings", t!("menu.readings"))}
                        </li>
                        <li>
                            {nav_link(&self.path, &self.locale, "/daily-office", t!("toc.daily_office"))}
                            <ul>
                                <li>
                                    {nav_link(&self.path, &self.locale, "/", t!("toc.morning_prayer"))}
                                </li>
                                <li>
                                    {nav_link(&self.path, &self.locale, "/canticle-table", t!("menu.canticle_table"))}
                                </li>
                            </ul>
                        </li>
                        <li>
                            {nav_link(&self.path, &self.locale, "/psalter", t!("menu.psalter"))}
                        </li>
                        <li>
                            {nav_link(&self.path, &self.locale, "/hymnal", t!("menu.hymnal"))}
                        </li>
                        <li>
                            {nav_link(&self.path, &self.locale, "/meditation", t!("meditation.title"))}
                        </li>
                        <li>
                            {nav_link(&self.path, &self.locale, "/settings", t!("settings.title"))}
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}

fn nav_link(current_url: &str, locale: &str, href: &str, label: String) -> Node {
    let localized_href = format!("/{}{}", locale, href);
    let active = !href.is_empty() && current_url.starts_with(&localized_href);
    view! {
        <a href={localized_href} class:current={active}>{label}</a>
    }
}
