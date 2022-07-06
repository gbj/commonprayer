#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Icon {
    Book,
    Calendar,
    Checkbox,
    Close,
    Cursor,
    Download,
    Folder,
    Halo,
    Harp,
    Left,
    Link,
    Music,
    Pause,
    Play,
    Prayer,
    Right,
    Settings,
    Stop,
    Swap,
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Icon::Book => "/static/icons/tabler-icon-book-2.svg",
                Icon::Calendar => "/static/icons/tabler-icon-calendar-event.svg",
                Icon::Checkbox => "/static/icons/checkbox.svg",
                Icon::Close => "/static/icons/tabler-icon-x.svg",
                Icon::Cursor => "/static/icons/cursor-text.svg",
                Icon::Download => "/static/icons/download.svg",
                Icon::Folder => "/static/icons/tabler-icon-folder.svg",
                Icon::Halo => "/static/icons/tabler-icon-oval-vertical.svg",
                Icon::Harp => "/static/icons/harp.svg",
                Icon::Left => "/static/icons/tabler-icon-arrow-left.svg",
                Icon::Link => "/static/icons/tabler-icon-link.svg",
                Icon::Music => "/static/icons/tabler-icon-music.svg",
                Icon::Pause => "/static/icons/tabler-icon-player-pause.svg",
                Icon::Play => "/static/icons/tabler-icon-player-play.svg",
                Icon::Prayer => "/static/icons/fa-praying-hands.svg",
                Icon::Right => "/static/icons/tabler-icon-arrow-right.svg",
                Icon::Settings => "/static/icons/tabler-icon-settings.svg",
                Icon::Stop => "/static/icons/tabler-icon-player-stop.svg",
                Icon::Swap => "/static/icons/tabler-icon-arrows-left-right.svg",
            }
        )
    }
}
