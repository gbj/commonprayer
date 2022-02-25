use leptos::*;

use super::Icon;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Button {
    Hamburger,
    Image(Icon),
}

fn build_menu(id: &str, side: &'static str, button: Button, content: View) -> View {
    let button = match button {
        Button::Hamburger => view! {
            // "hambuger" menu button created via CSS, and positioned over the toggle checkbox
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
            // note that it can be hidden from a screen reader, because the nav menu will simply appear
            // for a screen reader (it's off-page visually until toggled)
            <label for={&toggle_id} aria-hidden="true">
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

pub fn menu(locale: &str) -> View {
    build_menu(
        "main-menu",
        "left",
        Button::Hamburger,
        view! {
            <ul>
                <li>
                    <h1>
                        <a href={format!("/{}", locale)}>{t!("common_prayer")}</a>
                    </h1>
                </li>
                <li>
                    <a href={format!("/{}/calendar", locale)}>{t!("menu.calendar")}</a>
                </li>
                <li>
                    <a href={format!("/{}/daily-office", locale)}>{t!("toc.daily_office")}</a>
                </li>
                <li>
                    <a href={format!("/{}/readings", locale)}>{t!("toc.daily_readings")}</a>
                </li>
                <li>
                    <a href={format!("/{}/lectionary", locale)}>{t!("menu.lectionary")}</a>
                </li>
                <li>
                    <a href={format!("/{}/psalter", locale)}>{t!("menu.psalter")}</a>
                </li>
                <li>
                    <a href={format!("/{}/hymnal", locale)}>{t!("menu.hymnal")}</a>
                </li>
                <li>
                    <a href={format!("/{}/settings", locale)}>{t!("settings.title")}</a>
                </li>
                <li>
                    <a href={format!("/{}/about", locale)}>{t!("menu.about")}</a>
                </li>
            </ul>
        },
    )
}

pub fn side_menu(icon: Icon, content: View) -> View {
    build_menu("side-menu", "right", Button::Image(icon), content)
}
