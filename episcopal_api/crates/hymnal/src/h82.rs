use crate::Hymnal;
use lazy_static::lazy_static;

#[cfg(test)]
mod tests {
    use crate::{HymnNumber, HYMNAL_1982};

    #[test]
    fn contains_all_consecutive_hymn_numbers() {
        let mut last_number = HymnNumber::S(0);
        let mut last_title = String::default();
        for hymn in &HYMNAL_1982.hymns {
            match (last_number, hymn.number) {
                (crate::HymnNumber::S(a), crate::HymnNumber::S(b)) => {
                    if !(last_title == hymn.title || b == a + 1) {
                        panic!("chain broke at S{}", b);
                    }
                }
                (crate::HymnNumber::S(_), crate::HymnNumber::H(h)) => assert_eq!(h, 1),
                (crate::HymnNumber::H(_), crate::HymnNumber::S(_)) => panic!(),
                (crate::HymnNumber::H(a), crate::HymnNumber::H(b)) => {
                    if !(last_title == hymn.title || b == a + 1) {
                        panic!("chain broke at {}", b);
                    }
                }
            }
            last_number = hymn.number;
            last_title = hymn.title.clone();
        }
    }
}

lazy_static! {
    pub static ref HYMNAL_1982: Hymnal = serde_json::from_str(include_str!("./h82.json")).unwrap();
}
