use language::Language;

pub fn locale_to_language(locale: &str) -> Language {
    match locale {
        "en" => Language::En,
        "en-US" => Language::En,
        "es" => Language::Es,
        "es-ES" => Language::Es,
        "fr" => Language::Fr,
        "fr-FR" => Language::Fr,
        _ => Language::En,
    }
}
