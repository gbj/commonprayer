mod about;
pub mod calendar;
pub mod canticle_choice;
mod canticle_table;
mod contents;
mod document;
mod home;
mod hymn;
mod hymnal;
pub mod index;
pub mod meditation;
pub mod psalm;
pub mod readings;
pub mod search;
pub mod settings;
pub use index::{Index, UserId};

use leptos2::*;

use self::{
    about::About,
    calendar::CalendarView,
    canticle_choice::CanticleChoice,
    canticle_table::CanticleTableView,
    contents::ContentsView,
    document::DocumentPage,
    home::HomePage,
    hymn::{
        hymn_music_view::HymnMusicView, hymn_text_view::HymnTextView,
        hymn_video_player_view::HymnVideoPlayerView, hymn_video_view::HymnVideoView, HymnView,
    },
    hymnal::{hymnal_page::HymnalPageView, HymnalView},
    meditation::MeditationView,
    psalm::PsalmView,
    readings::{eucharist::EucharistView, ReadingsView},
    readings::{holy_day::HolyDayView, office::OfficeView},
    search::SearchView,
    settings::*,
};

pub fn router() -> Router<Index> {
    Router::new(
        Route::<Index>::new("")
            .child(Route::<HomePage>::new(""))
            .child(Route::<About>::new("about"))
            .child(Route::<CalendarView>::new("calendar"))
            .child(Route::<CanticleChoice>::new("canticle-choice"))
            .child(Route::<CanticleChoice>::new("canticle-choice/:version"))
            .child(Route::<CanticleTableView>::new("canticle-table"))
            .child(Route::<CanticleTableView>::new("canticle-table/:table"))
            .child(Route::<ContentsView>::new("contents"))
            .child(
                Route::<ReadingsView>::new("readings")
                    .child(Route::<OfficeView>::new("office"))
                    .child(Route::<EucharistView>::new("eucharist"))
                    .child(Route::<HolyDayView>::new("holy-day")),
            )
            .child(Route::<DocumentPage>::new("document/**"))
            .child(Route::<MeditationView>::new("meditation"))
            .child(
                Route::<HymnView>::new("hymn/:hymnal/:number")
                    .child(Route::<HymnTextView>::new("text"))
                    .child(Route::<HymnMusicView>::new("music"))
                    .child(
                        Route::<HymnVideoView>::new("video")
                            .child(Route::<HymnVideoPlayerView>::new("play")),
                    )
                    .child(Route::<HymnTextView>::new("")),
            )
            .child(Route::<HymnalView>::new("hymnal").child(Route::<HymnalView>::new(":hymnal")))
            // note: this route is not nested under HymnalView because, while the route is "nested," the views are not nested
            // the fact it's a child route is just for show, basically
            // to nest the route with .child() would mean injecting the page viewer into the hymnal search page -- not what we want
            .child(Route::<HymnalPageView>::new("hymnal/:hymnal/page"))
            .child(Route::<PsalmView>::new("psalm"))
            .child(Route::<SearchView>::new("search"))
            .child(
                Route::<SettingsView>::new("settings")
                    .child(Route::<GeneralSettingsView>::new(""))
                    .child(Route::<DisplaySettingsView>::new("display"))
                    .child(Route::<LiturgySettingsView>::new("liturgy"))
                    .child(Route::<LiturgySettingsView>::new("liturgy/**")),
            ),
        &["en"],
    )
}
