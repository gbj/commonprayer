pub mod canticles;

pub use canticles::*;
use liturgy::Text;

lazy_static! {
    pub static ref GLORIA_PATRI: Text = Text::from(
        "Praise to the holy and undivided Trinity, one God: as it was in the beginning, is now, and will be for ever. Amen."
    );
}
