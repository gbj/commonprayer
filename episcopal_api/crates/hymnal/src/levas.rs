use crate::Hymnal;
use lazy_static::lazy_static;

#[cfg(test)]
mod tests {
    use crate::LEVAS;

    #[test]
    fn contains_all_consecutive_hymn_numbers() {
        let mut last_number = 0;
        for hymn in &LEVAS.hymns {
            let num = match hymn.number {
                crate::HymnNumber::S(_) => panic!(), // no S section in WLP
                crate::HymnNumber::H(n) => n,
            };
            assert_eq!(num, last_number + 1);
            last_number = num;
        }
    }
}

lazy_static! {
    pub static ref LEVAS: Hymnal = serde_json::from_str(include_str!("./levas.json")).unwrap();
}
