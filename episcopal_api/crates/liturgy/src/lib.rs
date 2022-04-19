#![feature(iter_intersperse)]

mod antiphon;
mod biblical_citation;
mod biblical_reading;
mod canticle;
mod choice;
mod condition;
mod display_format;
mod document;
mod document_error;
mod gloria_patri;
mod heading;
mod hymn_link;
mod invitatory;
mod lectionary_reading;
mod litany;
mod liturgy;
mod parallel;
pub mod parallel_table;
mod path;
mod preces;
mod preference;
mod psalm;
mod psalm_citation;
mod reference;
mod responsive_prayer;
mod rubric;
mod sentence;
mod series;
mod show;
mod slug;
mod text;
mod version;

pub use crate::liturgy::*;
pub use antiphon::*;
pub use biblical_citation::*;
pub use biblical_reading::*;
pub use canticle::*;
pub use choice::*;
pub use condition::Condition;
pub use display_format::DisplayFormat;
pub use document::{Content, Document};
pub use document_error::*;
pub use gloria_patri::*;
pub use heading::*;
pub use hymn_link::*;
pub use invitatory::*;
pub use lectionary_reading::*;
pub use litany::*;
pub use parallel::*;
pub use path::*;
pub use preces::*;
pub use preference::*;
pub use psalm::*;
pub use psalm_citation::*;
pub use reference::*;
pub use responsive_prayer::*;
pub use rubric::*;
pub use sentence::*;
pub use series::*;
pub use show::*;
pub use slug::*;
pub use text::*;
pub use version::*;

pub(crate) fn is_false(val: &bool) -> bool {
    !*val
}
