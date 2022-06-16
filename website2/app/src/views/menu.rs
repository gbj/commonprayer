use leptos2::*;

use super::Icon;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Button {
    Hamburger,
    Image(Icon),
}

pub fn menu(locale: &str, current_location: &str) -> Node {
    build_menu(
        "main-menu",
        "left",
        Button::Hamburger,
        view! {
            <ul>
                <li>
                    <h1>
                        {nav_link(current_location, locale, "", t!("common_prayer"))}
                    </h1>
                </li>
                <form action="search">
                    <input class="main-search" type="search" name="q" placeholder={t!("search")}/>
                    <noscript><input type="submit" value={t!("search")}/></noscript>
                </form>
                <li>
                    {nav_link(current_location, locale, "/calendar", t!("menu.calendar"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/canticle-table", t!("menu.canticle_table"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/daily-office", t!("toc.daily_office"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/readings/office", t!("toc.daily_readings"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/lectionary", t!("menu.lectionary"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/psalter", t!("menu.psalter"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/hymnal", t!("menu.hymnal"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/meditation", t!("meditation.title"))}
                </li>
                <li>
                    {nav_link(current_location, locale, "/settings", t!("settings.title"))}
                </li>
            </ul>
        },
    )
}

fn nav_link(current_url: &str, locale: &str, href: &str, label: String) -> Node {
    let active = href != "" && current_url.starts_with(href);
    let href = format!("/{}{}", locale, href);
    view! {
        <a href={href} class:current={active}>{label}</a>
    }
}

pub fn side_menu(icon: Icon, content: Node) -> Node {
    build_menu("side-menu", "right", Button::Image(icon), content)
}

fn build_menu(id: &str, side: &'static str, button: Button, content: Node) -> Node {
    let button = match button {
        Button::Hamburger => view! {
            // "hamburger" menu button created via CSS, and positioned over the toggle checkbox
            // (but under it by z-index) so it appears to be a button
            // this, too, can be ignored by a screen reader (see above)
            <div class="menu-toggle-button hamburger" aria-hidden="true">
                <span></span>
                <span></span>
                <span></span>
            </div>
        },
        Button::Image(icon) => view! {
            <img class="menu-toggle-button" src={icon.to_string()} alt={t!("menu.open_menu")} />
        },
    };

    let toggle_id = format!("{}-toggle-checkbox", id);
    let menu_class = format!("menu {}", side);

    view! {
        <nav id={id} role="navigation" class={menu_class}>
            // an invisible checkbox that toggles whether the menu appears or not via CSS
            <input id={&toggle_id} type="checkbox" class="menu-toggle-input"/>

            // label contains the overlay, so that when the overlay is clicked the menu disappears
            <label for={&toggle_id}>
                <span class="screen-reader-only">{t!("menu.open_menu")}</span>
                <div class="overlay"></div>
            </label>

            {button}

            // Here's the actual content of the navigation menu
            <div class="menu-content">
                {content}
            </div>
        </nav>
    }
}
