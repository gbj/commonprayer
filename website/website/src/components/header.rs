use super::menu;
use leptos::*;
use web_sys::{ScrollBehavior, ScrollToOptions};

pub fn header(locale: &str, page_title: &str) -> View {
    build_header(locale, page_title, None, [])
}

pub fn header_with_side_menu(locale: &str, page_title: &str, side_menu: View) -> View {
    build_header(locale, page_title, Some(side_menu), [])
}

pub fn header_with_side_menu_and_buttons(
    locale: &str,
    page_title: &str,
    side_menu: View,
    additional_buttons: impl IntoIterator<Item = View>,
) -> View {
    build_header(locale, page_title, Some(side_menu), additional_buttons)
}

fn build_header(
    locale: &str,
    page_title: &str,
    side_menu: Option<View>,
    additional_buttons: impl IntoIterator<Item = View>,
) -> View {
    let side_menu = side_menu.unwrap_or(View::Empty);
    let additional_buttons = additional_buttons.into_iter().collect::<Vec<_>>();
    let additional_buttons = if additional_buttons.is_empty() {
        View::Empty
    } else {
        view! {
            <section class="additional-buttons">
                {View::Fragment(additional_buttons)}
            </section>
        }
    };

    view! {
        <header>
            {menu(locale)}
            <dyn:h1
                class="page-title"
                on:click=|_ev: Event| window().scroll_to_with_scroll_to_options(ScrollToOptions::new().top(0.0).behavior(ScrollBehavior::Smooth))
            >
                {page_title}
            </dyn:h1>
            {side_menu}
            {additional_buttons}
        </header>
    }
}
