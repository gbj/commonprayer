#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Icon {
    Calendar,
    Checkbox,
    Close,
    Cursor,
    Download,
    Left,
    Link,
    Music,
    Play,
    Right,
    Settings,
    Swap,
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Icon::Calendar => "/static/icons/tabler-icon-calendar-event.svg",
                Icon::Checkbox => "/static/icons/checkbox.svg",
                Icon::Close => "/static/icons/tabler-icon-x.svg",
                Icon::Cursor => "/static/icons/cursor-text.svg",
                Icon::Download => "/static/icons/download.svg",
                Icon::Left => "/static/icons/tabler-icon-arrow-left.svg",
                Icon::Link => "/static/icons/tabler-icon-link.svg",
                Icon::Music => "/static/icons/tabler-icon-music.svg",
                Icon::Play => "/static/icons/tabler-icon-player-play.svg",
                Icon::Right => "/static/icons/tabler-icon-arrow-right.svg",
                Icon::Settings => "/static/icons/tabler-icon-settings.svg",
                Icon::Swap => "/static/icons/tabler-icon-arrows-left-right.svg",
            }
        )
    }
}
