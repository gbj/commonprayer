#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Icon {
    Calendar,
    Close,
    Music,
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
                Icon::Close => "/static/icons/tabler-icon-x.svg",
                Icon::Music => "/static/icons/tabler-icon-music.svg",
                Icon::Settings => "/static/icons/tabler-icon-settings.svg",
                Icon::Swap => "/static/icons/tabler-icon-arrows-left-right.svg",
            }
        )
    }
}
