#![feature(fn_traits)]
#![feature(iter_intersperse)]
#![feature(unboxed_closures)]

mod calendar;
mod canticle_table;
mod document;
mod fetch;
mod header;
mod home;
mod i18n;
mod icon;
mod locales;
mod menu;
mod modal;
mod psalter;
mod readings;
mod search;
mod settings;
mod time;

use crate::calendar::*;
use crate::canticle_table::*;
use crate::home::*;
use crate::menu::*;
use crate::psalter::*;
use crate::readings::*;
use crate::search::*;
use i18n::*;
pub use i18n::{Localizer, LocalizerProps};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use settings::GeneralSettings;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::new());
    provide_context(cx, RouterContext::new(cx, None, None));

    // i18n
    provide_localization(cx);
    let params = use_params_map(cx);
    let (t, _, set_locale) = use_i18n(cx);
    set_locale(&params().get("lang").cloned().unwrap_or_else(|| "en".into()));

    // Settings
    let settings = create_signal(cx, GeneralSettings::default());
    provide_context(cx, settings::GeneralSettingsContext(settings.0, settings.1));

    view! { cx, 
        <div id="root">
            <Title formatter=(move |title| format!("{} - {}", title, t("common_prayer"))).into() text="Home".into()/>
            <Stylesheet href="/styles/main.css".into() />
            <Menu/>
            <Routes>
                <Route path=":lang?" element=|cx| view! { cx,  <Outlet/> }>
                    <Route path="calendar" element=|cx| view! { cx,  <Calendar/> }/>
                    <Route path="psalm" element=|cx| view! { cx,  <Psalter/> } loader=psalter_data.into()/>
                    <Route path="readings" element=|cx| view! { cx,  <Readings/> }>
                        <Route path="office" element=|cx| view! { cx,  <OfficeReadings/> } loader=office_readings_data.into()/>
                        <Route path="eucharist" element=|cx| view! { cx,  <EucharistReadings/> }/>
                        //<Route path="holy-day" element=|cx| view! { cx,  <HolyDayReadings/> }/>
                        <Route path="" element=|cx| view! { cx,  <OfficeReadings/> } loader=office_readings_data.into()/>
                    </Route>
                    <Route path="canticle-table" element=|cx| view! { cx,  <CanticleTable/> }>
                        <Route path="eow" element=|cx| view! { cx,  <EOWTable/> }/>
                        <Route path="bcp" element=|cx| view! { cx,  <BCPTable/> }/>
                        <Route path="" element=|cx| view! { cx,  <BCPTable/> }/>
                    </Route>
                    <Route path="search" element=|cx| view! { cx,  <Search/> } loader=search_data.into() />
                    <Route path="" element=|cx| view! { cx,  <Home/> }/>
                </Route>
            </Routes>
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
