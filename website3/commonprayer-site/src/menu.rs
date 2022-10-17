use leptos::*;
use leptos_router::*;
use liturgy::Slug;
use typed_builder::TypedBuilder;

use crate::{
    i18n::use_i18n,
    settings::{use_settings, use_settings_for_liturgy},
    time::{get_timezone_offset, today},
};

#[component]
pub fn Menu(cx: Scope) -> Element {
    let (t, _, _) = use_i18n(cx);
    let (settings, _) = use_settings(cx);

    view! { cx,
        <details class="Menu-root">
            <summary><span class="Menu-label">{t("menu-open")}</span></summary>
            <dialog open class="Menu-dialog">
                <nav id="main-menu" role="navigation" class="menu left">
                    <ul class="Menu-content">
                        <li class="title horizontal">
                            <h1>
                                <LocalizedNavLink href="/">{t("common_prayer")}</LocalizedNavLink>
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
                        <div>
                            <Form action="search".into()>
                                <input class="Menu-search" type="search" name="q" placeholder=t("search")/>
                                <noscript><input type="submit" value=t("search")/></noscript>
                            </Form>
                        </div>
                        <li>
                            <LocalizedNavLink href="contents">{t("toc-table_of_contents")}</LocalizedNavLink>
                        </li>
                        <li>
                            <LocalizedNavLink href={move || if settings.with(|settings| settings.use_lff) {
                                "calendar".to_string()
                            } else {
                                "calendar?calendar=bcp".to_string()
                            }}>
                                {t("menu-calendar")}
                            </LocalizedNavLink>

                        </li>
                        <li>
                            <LocalizedNavLink href=move || format!("readings/office?version={}", settings.with(|settings| settings.bible_version))>
                                {t("menu-readings")}
                            </LocalizedNavLink>
                        </li>
                        <li>
                            // TODO render fix (wrapper unnecessary)
                            <span>
                                <LocalizedNavLink href="document/office">{t("toc-daily_office")}</LocalizedNavLink>
                            </span>
                            <ul>
                                <li>
                                    <OfficeLink slug=Slug::MorningPrayer label=t("toc-morning_prayer")/>
                                </li>
                                <li>
                                    <OfficeLink slug=Slug::NoondayPrayer label=t("toc-noonday_prayer")/>
                                </li>
                                <li>
                                    <OfficeLink slug=Slug::EveningPrayer label=t("toc-evening_prayer")/>
                                </li>
                                <li>
                                    <OfficeLink slug=Slug::Compline label=t("toc-compline")/>
                                </li>
                                <li>
                                    <LocalizedNavLink href="canticle-table">{t("menu-canticle-table")}</LocalizedNavLink>
                                </li>
                            </ul>
                        </li>
                        <li>
                            <LocalizedNavLink href="psalm">{t("menu-psalter")}</LocalizedNavLink>
                        </li>
                        <li>
                            <LocalizedNavLink href="document/prayers-and-thanksgivings">{t("toc-prayers_and_thanksgivings")}</LocalizedNavLink>
                        </li>
                        <li>
                            <LocalizedNavLink href="hymnal">{t("menu-hymnal")}</LocalizedNavLink>
                        </li>
                        <li>
                            <LocalizedNavLink href="meditation">{t("meditation-title")}</LocalizedNavLink>
                        </li>
                        <li>
                            <LocalizedNavLink href="settings">{t("settings-title")}</LocalizedNavLink>
                        </li>
                    </ul>
                </nav>
            </dialog>
        </details>
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

    view! { cx,
        <LocalizedNavLink href=href>{label.clone()}</LocalizedNavLink>
    }
}

#[derive(TypedBuilder)]
pub struct LocalizedNavLinkProps<C, H>
where
    C: IntoChild,
    H: ToHref + 'static,
{
    href: H,
    children: Box<dyn Fn() -> Vec<C>>,
}

#[allow(non_snake_case)]
pub fn LocalizedNavLink<C, H>(cx: Scope, props: LocalizedNavLinkProps<C, H>) -> Element
where
    C: IntoChild + 'static,
    H: ToHref + 'static,
{
    let params = use_params_map(cx);
    let locale_in_path = move || params.with(|p| p.get("lang").cloned());
    let localized_href = move || {
        if locale_in_path().is_some() {
            props.href.to_href()()
        } else {
            format!("/en/{}", props.href.to_href()())
        }
    };

    let mut children = (props.children)();
    if children.len() != 1 {
        log::warn!("Pass exactly one child to <LocalizedNavLink/>. If you want to pass more than one child, next them within an element.");
    }
    let child = children.remove(0).into_child(cx);

    view! { cx,
        <A href=localized_href>{child.clone()}</A>
    }
}
