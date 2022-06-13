mod index;
mod about;
pub mod calendar;
mod document;
mod hymnal;
mod hymn;
pub use index::Index;

use leptos2::*;

use self::{about::About, calendar::CalendarView, document::DocumentPage, hymnal::HymnalView, hymn::HymnView};

pub fn router() -> Router<Index> {
	Router::new(
		Route::<Index>::new("")
			.child(Route::<About>::new("about"))
			.child(Route::<CalendarView>::new("calendar"))
			.child(Route::<CalendarView>::new("calendar/lff2018"))
			.child(Route::<DocumentPage>::new("document/**"))
			.child(Route::<HymnalView>::new("hymnal")
				.child(Route::<HymnalView>::new(":hymnal"))
			.child(Route::<HymnView>::new("hymnal/:hymnal/:hymn"))
		),
		&["en"]
	)
}