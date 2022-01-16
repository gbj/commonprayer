use serde::{Deserialize, Serialize};

/// Language that can be assigned to a [Document](liturgy::Document)
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Language {
    /// English
    En,
    /// Spanish
    Es,
    /// French
    Fr,
    /// Haitian Creole
    Ht,
}

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}

impl Language {
    // TODO i18n when other languages are added
    pub fn i18n(&self, string: &str) -> String {
        match (self, string) {
            (Language::En, "Mon") => "Monday",
            (Language::En, "Tue") => "Tuesday",
            (Language::En, "Wed") => "Wednesday",
            (Language::En, "Thu") => "Thursday",
            (Language::En, "Fri") => "Friday",
            (Language::En, "Sat") => "Saturday",
            (Language::En, "Sun") => "Sunday",
            _ => string,
        }
        .to_string()
    }

    pub fn month_name(&self, month: u8) -> &'static str {
        match (self, month) {
            (Language::En, 1) => "January",
            (Language::En, 2) => "February",
            (Language::En, 3) => "March",
            (Language::En, 4) => "April",
            (Language::En, 5) => "May",
            (Language::En, 6) => "June",
            (Language::En, 7) => "July",
            (Language::En, 8) => "August",
            (Language::En, 9) => "September",
            (Language::En, 10) => "October",
            (Language::En, 11) => "November",
            (Language::En, 12) => "December",
            _ => todo!(),
        }
    }
}
