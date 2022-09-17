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
            <Localizer>
                <div>
                    <Router mode=BrowserIntegration {}>
                        <Routes>
                            <Route path=":lang" element=|cx| view! { <Layout/> }>
                                <Route path="calendar" element=|cx| view! { <Calendar/> } loader=calendar_data.into()/>
                                <Route path="" element=|cx| view! { <Home/> } loader=calendar_data.into()/>
                            </Route>
                        </Routes>
                    </Router>
                </div>
            </Localizer>
        </div>
    }
}

#[component]
fn Layout(cx: Scope) -> impl IntoChild {
    let params = use_params_map(cx);
    let (_, set_locale) = use_i18n(cx);
    set_locale(&params.get("lang").cloned().unwrap_or_else(|| "en".into()));

    view! {
        <>
            <Menu/>
            <Header/>
            <main>
                <Outlet/>
            </main>
        </>
    }
}

#[macro_export]
macro_rules! debug_warn {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                log::warn!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            { }
        }
    }
}
