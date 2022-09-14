mod calendar;
//mod form;
mod i18n;
mod icon;
mod menu;
mod modal;
mod settings;
mod time;

use crate::calendar::*;
use crate::menu::*;
use i18n::*;
use leptos::*;

#[component]
pub fn App(cx: Scope) -> Element {
    view! {
        <div>
            <Router mode=BrowserIntegration {}>
                <Routes>
                    <Route path=":lang?" element=|cx| view! { <Localizer/> }>
                        <Route path="" element=|cx| view! { <Index/> }>
                            <Route path="calendar" element=|cx| view! { <Calendar/> } loader=calendar_data.into()/>
                        </Route>
                    </Route>
                </Routes>
            </Router>
        </div>
    }
}

#[component]
fn Index(cx: Scope) -> Element {
    view! {
        <div>
            <Menu/>
            <Outlet/>
        </div>
    }
}
