use crate::i18n::LocaleData;

mod en;

pub fn locale_data(_locale: &str) -> LocaleData {
    self::en::en()
}
