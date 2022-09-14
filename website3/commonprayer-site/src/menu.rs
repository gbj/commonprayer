use leptos::*;
use liturgy::Slug;
use typed_builder::TypedBuilder;

use crate::{
    i18n::use_i18n,
    settings::{use_settings, use_settings_for_liturgy},
    time::{get_timezone_offset, today},
};

#[component]
pub fn Menu(cx: Scope) -> Element {
    let (t, _) = use_i18n(cx);
    let (settings, _) = use_settings(cx);

    view! {
        <nav id="main-menu" role="navigation" class="menu left">
            // an invisible checkbox that toggles whether the menu appears or not via CSS
            <input id="main-menu-toggle-checkbox" type="checkbox" class="menu-toggle-input"/>

            // label contains the overlay, so that when the overlay is clicked the menu disappears
            <label for="main-menu-toggle-checkbox">
                <span class="screen-reader-only">{t("menu.open_menu")}</span>
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
                            <LocalizedNavLink to="/">{t("common_prayer")}</LocalizedNavLink>
                        </h1>
                        /* <Auth
                            prop:user={user.cloned()}
                            data-modal-id="login"
                            loginlabel={t("auth.title")}
                            logoutlabel={t("auth.logout")}
                        ></Auth>
                        {user_logged_in}
                        <Modal id="login">
                            <div id="firebase-auth" slot="content"></div>
                        </Modal> */
                    </li>
                    <form action="search">
                        <input class="main-search" type="search" name="q" placeholder={t("search")}/>
                        <noscript><input type="submit" value={t("search")}/></noscript>
                    </form>
                   <li>
                        <LocalizedNavLink to="contents">{t("toc.table_of_contents")}</LocalizedNavLink>
                    </li>
                    <li>
                        <LocalizedNavLink to={move || if settings.with(|settings| settings.use_lff) {
                            "calendar".to_string()
                        } else {
                            "calendar?calendar=bcp".to_string()
                        }}>
                            {t("menu.calendar")}
                        </LocalizedNavLink>

                    </li>
                    <li>
                        <LocalizedNavLink to={move || format!("readings/office?version={}", settings.with(|settings| settings.bible_version))}>
                            {t("menu.readings")}
                        </LocalizedNavLink>
                    </li>
                    <li>
                        // TODO render fix (wrapper unnecessary)
                        <span>
                            <LocalizedNavLink to="document/office">{t("toc.daily_office")}</LocalizedNavLink>
                        </span>
                        <ul>
                            <li>
                                <OfficeLink slug=Slug::MorningPrayer label=t("toc.morning_prayer")/>
                            </li>
                            <li>
                                <OfficeLink slug=Slug::NoondayPrayer label=t("toc.noonday_prayer")/>
                            </li>
                            <li>
                                <OfficeLink slug=Slug::EveningPrayer label=t("toc.evening_prayer")/>
                            </li>
                            <li>
                                <OfficeLink slug=Slug::Compline label=t("toc.compline")/>
                            </li>
                            <li>
                                <LocalizedNavLink to="canticle-table">{t("menu.canticle_table")}</LocalizedNavLink>
                            </li>
                        </ul>
                    </li>
                    <li>
                        <LocalizedNavLink to="psalm">{t("menu.psalter")}</LocalizedNavLink>
                    </li>
                    <li>
                        <LocalizedNavLink to="document/prayers-and-thanksgivings">{t("toc.prayers_and_thanksgivings")}</LocalizedNavLink>
                    </li>
                    <li>
                        <LocalizedNavLink to="hymnal">{t("menu.hymnal")}</LocalizedNavLink>
                    </li>
                    <li>
                        <LocalizedNavLink to="meditation">{t("meditation.title")}</LocalizedNavLink>
                    </li>
                    <li>
                        <LocalizedNavLink to="settings">{t("settings.title")}</LocalizedNavLink>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn OfficeLink(cx: Scope, slug: Slug, label: String) -> Element {
    let tz = get_timezone_offset(cx);
    let (settings, _) = use_settings(cx);
    let prefs = use_settings_for_liturgy(cx, slug);
    let slug = slug.slugify();
    let today = today(&tz);
    let href = move || {
        format!(
            "document/office/{}/{:?}?date={}&prefs={}",
            slug,
            settings.with(|s| s.liturgy_version),
            today,
            prefs.serialize_non_default_prefs() // TODO URL encode //urlencoding::encode(&prefs.serialize_non_default_prefs())
        )
    };

    view! {
        <LocalizedNavLink to={href}>{label}</LocalizedNavLink>
    }
}

#[derive(TypedBuilder)]
pub struct LocalizedNavLinkProps<C, H>
where
    C: IntoChild,
    H: ToHref + 'static,
{
    to: H,
    children: Vec<C>,
}

#[allow(non_snake_case)]
pub fn LocalizedNavLink<C, H>(cx: Scope, mut props: LocalizedNavLinkProps<C, H>) -> Element
where
    C: IntoChild,
    H: ToHref + 'static,
{
    let params = use_params_map(cx);
    let locale_in_path = params.get("lang").cloned();
    let localized_href = move || {
        if locale_in_path.is_some() {
            props.to.to_href()()
        } else {
            format!("/en/{}", props.to.to_href()())
        }
    };

    if props.children.len() != 1 {
        log::warn!("[LocalizedNavLink] Pass exactly one child to <Link/>. If you want to pass more than one child, next them within an element.");
    }
    let child = props.children.remove(0);

    view! {
        <NavLink to={localized_href}>{child}</NavLink>
    }
}
