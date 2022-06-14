use std::fmt::Display;

use super::menu;
use leptos2::*;
use web_sys::{ScrollBehavior, ScrollToOptions};

pub struct Header {
    locale: String,
    page_title: String,
    side_menu: Option<Node>,
    additional_buttons: Vec<Node>,
}

impl Header {
    pub fn new(locale: impl Display, page_title: impl Display) -> Self {
        Self {
            locale: locale.to_string(),
            page_title: page_title.to_string(),
            side_menu: None,
            additional_buttons: Vec::new(),
        }
    }

    pub fn new_with_side_menu(
        locale: impl Display,
        page_title: impl Display,
        side_menu: Node,
    ) -> Self {
        Self {
            locale: locale.to_string(),
            page_title: page_title.to_string(),
            side_menu: Some(side_menu),
            additional_buttons: Vec::new(),
        }
    }

    pub fn new_with_side_menu_and_additional_buttons(
        locale: impl Display,
        page_title: impl Display,
        side_menu: Node,
        additional_buttons: Vec<Node>,
    ) -> Self {
        Self {
            locale: locale.to_string(),
            page_title: page_title.to_string(),
            side_menu: Some(side_menu),
            additional_buttons,
        }
    }

    pub fn new_with_additional_buttons(
        locale: impl Display,
        page_title: impl Display,
        additional_buttons: Vec<Node>,
    ) -> Self {
        Self {
            locale: locale.to_string(),
            page_title: page_title.to_string(),
            side_menu: None,
            additional_buttons,
        }
    }
}

impl StaticView for Header {
    fn to_node(&self) -> Node {
        view! {
            <header>
                //{menu(&self.locale)}
                <h1
                    class="page-title"
                    on:click=|_ev: Event| window().scroll_to_with_scroll_to_options(ScrollToOptions::new().top(0.0).behavior(ScrollBehavior::Smooth))
                >
                    {&self.page_title}
                </h1>
                {self.side_menu.clone()}
                {&self.additional_buttons}
            </header>
        }
    }
}
