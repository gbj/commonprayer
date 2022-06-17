mod about;
pub mod calendar;
mod canticle_table;
mod contents;
mod document;
mod home;
mod hymn;
mod hymnal;
mod index;
pub mod meditation;
mod readings;
mod settings;
pub use index::Index;

use leptos2::*;

use self::{
    about::About,
    calendar::CalendarView,
    canticle_table::CanticleTableView,
    contents::ContentsView,
    document::DocumentPage,
    home::HomePage,
    hymn::HymnView,
    hymnal::HymnalView,
    meditation::MeditationView,
    readings::office::OfficeView,
    readings::{eucharist::EucharistView, ReadingsView},
    settings::*,
};

pub fn router() -> Router<Index> {
    Router::new(
        Route::<Index>::new("")
            .child(Route::<HomePage>::new(""))
            .child(Route::<About>::new("about"))
            .child(Route::<CalendarView>::new("calendar"))
            .child(Route::<CalendarView>::new("calendar/lff2018"))
            .child(Route::<CanticleTableView>::new("canticle-table"))
            .child(Route::<CanticleTableView>::new("canticle-table/:table"))
            .child(Route::<ContentsView>::new("contents"))
            .child(
                Route::<ReadingsView>::new("readings")
                    .child(Route::<OfficeView>::new("office"))
                    .child(Route::<EucharistView>::new("eucharist")),
            )
            .child(Route::<DocumentPage>::new("document/**"))
            .child(Route::<MeditationView>::new("meditation"))
            .child(Route::<HymnView>::new("hymn/:hymnal/:number"))
            .child(Route::<HymnalView>::new("hymnal").child(Route::<HymnalView>::new(":hymnal")))
            .child(
                Route::<SettingsView>::new("settings").child(Route::<GeneralSettingsView>::new("")), //.child(Route::<DisplaySettingsView>::new("display"))
                                                                                                     //.child(Route::<LiturgySettingsView>::new(":liturgy")),
            ),
        &["en"],
    )
}
