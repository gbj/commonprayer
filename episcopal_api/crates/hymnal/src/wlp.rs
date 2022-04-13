use crate::Hymnal;

use lazy_static::lazy_static;

#[cfg(test)]
mod tests {
    use crate::WLP;

    #[test]
    fn contains_all_consecutive_hymn_numbers() {
        let mut last_number = 720;
        let mut last_title = String::default();
        for hymn in &WLP.hymns {
            let num = match hymn.number {
                crate::HymnNumber::S(_) => panic!(), // no S section in WLP
                crate::HymnNumber::H(n) => n,
            };
            if !(last_title == hymn.title || num == last_number + 1) {
                panic!("chain broke at {}", num);
            }
            last_number = num;
            last_title = hymn.title.clone();
        }
    }
}

lazy_static! {
    pub static ref WLP: Hymnal = serde_json::from_str(include_str!("./wlp.json")).unwrap();
}
