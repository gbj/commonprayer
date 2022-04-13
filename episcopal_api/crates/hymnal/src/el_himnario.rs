use lazy_static::lazy_static;

use crate::Hymnal;

lazy_static! {
    pub static ref EL_HIMNARIO: Hymnal =
        serde_json::from_str(include_str!("./el_himnario.json")).unwrap();
}
