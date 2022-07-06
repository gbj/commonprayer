use std::collections::HashMap;

/// Parses the URL query string into a map of keys and values. This includes URL-decoding
/// and replacing '+' with a space.
/// ```
/// # use leptos2::parse_query;
/// let query = parse_query("q=Example+search&hymnal=Hymnal1982");
/// assert_eq!(query.get("q"), Some(&"Example search".to_string()));
/// assert_eq!(query.get("hymnal"), Some(&"Hymnal1982".to_string()));
/// let query = parse_query("");
/// assert!(query.is_empty());
/// let query = parse_query("q=");
/// assert_eq!(query.get("q"), Some(&"".to_string()));
/// let query = parse_query("q=Search&hymnal=Hymnal1982");
/// assert_eq!(query.get("q"), Some(&"Search".to_string()));
/// assert_eq!(query.get("hymnal"), Some(&"Hymnal1982".to_string()));
///```
pub fn parse_query(query_string: &str) -> HashMap<String, String> {
    query_string
        .split('&')
        .filter_map(|piece| {
            let mut parts = piece.split('=');
            let (k, v) = (parts.next(), parts.next());
            match k {
                Some(k) if !k.is_empty() => {
                    Some((unescape(k).to_string(), unescape(v.unwrap_or_default())))
                }
                _ => None,
            }
        })
        .collect()
}

fn unescape(s: &str) -> String {
    urlencoding::decode(s)
        .unwrap_or_else(|_| std::borrow::Cow::from(s))
        .replace('+', " ")
}
