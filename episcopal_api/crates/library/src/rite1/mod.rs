pub mod canticles;
pub mod collects;

pub use canticles::*;
use liturgy::GloriaPatri;

lazy_static! {
    pub static ref GLORIA_PATRI_TRADITIONAL: GloriaPatri = GloriaPatri::from((
        "Glory be to the Father, and to the Son, ",
        "and to the Holy Ghost: ",
        "as it was in the beginning, is now, and ever shall be, ",
        "world without end. Amen. "
    ));
}
