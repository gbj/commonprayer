use leptos2::*;
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum DocumentActionType {
    MarkFavorite,
    RemoveFavorite,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FavoriteId(pub i64);
