use std::collections::HashMap;

///```
/// # use leptos2::parse_query;
/// let query = parse_query("q=Search&hymnal=Hymnal1982");
/// assert_eq!(query.get("q"), Some(&"Search".to_string()));
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
                    Some((k.to_string(), v.unwrap_or_default().to_string()))
                }
                _ => None,
            }
        })
        .collect()
}
