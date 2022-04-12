use lazy_static::lazy_static;

use crate::{Hymnal, Hymnals};

lazy_static! {
    pub static ref EL_HIMNARIO: Hymnal = Hymnal {
        id: Hymnals::ElHimnario,
        title: "El Himnario".to_string(),
        subtitle: "".to_string(),
        copyright: "Copyright © 1998 by Church Publishing Incorporated".to_string(),
        year: 1998,
        hymns: vec![]
    };
}
