#![feature(fn_traits)]
#![feature(unboxed_closures)]

mod calendar;
mod document;
mod fetch;
mod header;
mod home;
mod i18n;
mod icon;
mod menu;
mod modal;
mod psalter;
mod readings;
mod settings;
mod time;

use crate::calendar::*;
use crate::header::*;
use crate::home::*;
use crate::menu::*;
use crate::psalter::*;
use crate::readings::*;
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
                            <Route path=":lang?" element=|cx| view! { <Layout/> }>
                                <Route path="calendar" element=|cx| view! { <Calendar/> } loader=calendar_data.into()/>
                                <Route path="psalm" element=|cx| view! { <Psalter/> } loader=psalter_data.into()/>
                                <Route path="readings" element=|cx| view! { <Readings/> } loader=readings_data.into()>
                                    <Route path="office" element=|cx| view! { <OfficeReadings/> } loader=office_readings_data.into()/>
                                    <Route path="eucharist" element=|cx| view! { <EucharistReadings/> }/>
                                    <Route path="holy-day" element=|cx| view! { <HolyDayReadings/> }/>
                                    <Route path="" element=|cx| view! { <OfficeReadings/> }/>
                                </Route>
                                <Route path="" element=|cx| view! { <Home/> }/>
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
    let (_, _, set_locale) = use_i18n(cx);
    set_locale(&params.get("lang").cloned().unwrap_or_else(|| "en".into()));

    view! {
        <div class="Layout">
            <Menu/>
            <div class="Layout-outlet"><Outlet/></div>
        </div>
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
