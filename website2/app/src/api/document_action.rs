use leptos2::*;
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum DocumentActionType {
    MarkFavorite,
    RemoveFavorite,
}
