mod index;
mod about;
pub mod calendar;
mod document;
mod hymnal;
mod hymn;
pub mod meditation;
mod canticle_table;
mod daily_readings;
pub use index::Index;

use leptos2::*;

use self::{about::About, calendar::CalendarView, daily_readings::ReadingsView, document::DocumentPage, hymnal::HymnalView, hymn::HymnView, meditation::MeditationView, canticle_table::CanticleTableView};

pub fn router() -> Router<Index> {
	Router::new(
		Route::<Index>::new("")
			.child(Route::<About>::new("about"))
			.child(Route::<CalendarView>::new("calendar"))
			.child(Route::<CalendarView>::new("calendar/lff2018"))
			.child(Route::<CanticleTableView>::new("canticle-table"))
			.child(Route::<CanticleTableView>::new("canticle-table/:table"))
			.child(Route::<ReadingsView>::new("readings"))
			.child(Route::<DocumentPage>::new("document/**"))
			.child(Route::<MeditationView>::new("meditation"))
			.child(Route::<HymnView>::new("hymn/:hymnal/:number"))
			.child(Route::<HymnalView>::new("hymnal")
				.child(Route::<HymnalView>::new(":hymnal")
			)
		),
		&["en"]
	)
}