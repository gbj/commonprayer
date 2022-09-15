mod calendar;
mod header;
mod home;
mod i18n;
mod icon;
mod menu;
mod modal;
mod settings;
mod time;

use crate::calendar::*;
use crate::header::*;
use crate::home::*;
use crate::menu::*;
use i18n::*;
use leptos::*;

#[component]
pub fn App(cx: Scope) -> Element {
    view! {
        <div id="root">
            <Router mode=BrowserIntegration {}>
                <Routes>
                    <Route path=":lang?" element=|cx| view! { <Localizer/> }>
                        <Route path="" element=|cx| view! { <Layout/> }>
                            <Route path="calendar" element=|cx| view! { <Calendar/> } loader=calendar_data.into()/>
                            <Route path="" element=|cx| view! { <Home/> } loader=calendar_data.into()/>
                        </Route>
                    </Route>
                </Routes>
            </Router>
        </div>
    }
}

#[component]
fn Layout(cx: Scope) -> Vec<Element> {
    view! {
        <>
            <Menu/>
            <Header/>
            <main><Outlet/></main>
        </>
    }
}
