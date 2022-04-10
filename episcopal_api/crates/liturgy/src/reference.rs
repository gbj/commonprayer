use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub source: Source,
    pub page: u16,
}

impl Reference {
    pub fn as_url(&self) -> String {
        let base_url = self.source.as_url();

        format!("{}#page={}", base_url, self.page)
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} p. {}", self.source, self.page)
    }
}

impl From<u16> for Reference {
    fn from(page: u16) -> Self {
        Self {
            source: Source::default(),
            page,
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum Source {
    BCP1979,
    EOW1,
    EOW2,
    LFF2018,
    BookOfOccasionalServices2018,
    LibroDeOracionComun,
    LiturgicalResources1,
    LiturgicalResources2,
}

impl Source {
    pub fn as_url(&self) -> &'static str {
        match self {
            Source::BCP1979 => "https://www.episcopalchurch.org/wp-content/uploads/sites/2/2019/11/bcp_compressed.pdf",
            Source::EOW1 => "https://www.churchpublishing.org/siteassets/pdf/enriching-our-worship-1/enrichingourworship1.pdf",
            Source::EOW2 => "https://www.churchpublishing.org/siteassets/pdf/enriching-our-worship-2/enrichingourworship2.pdf",
            Source::LFF2018 => "https://www.episcopalcommonprayer.org/uploads/1/2/9/8/129843103/lesser_feasts_and_fasts_2018_final_pages.pdf",
            Source::BookOfOccasionalServices2018 => "https://extranet.generalconvention.org/staff/files/download/24493",
            Source::LibroDeOracionComun => "https://www.episcopalchurch.org/wp-content/uploads/sites/2/2019/11/ellibro_deoracion_comun.pdf",
            Source::LiturgicalResources1 => "https://extranet.generalconvention.org/staff/files/download/15668",
            Source::LiturgicalResources2 => "https://extranet.generalconvention.org/staff/files/download/21226",
        }
    }

    pub fn long_name(&self) -> &'static str {
        match self {
            Source::BCP1979 => "Book of Common Prayer (1979)",
            Source::EOW1 => "Enriching Our Worship 1",
            Source::EOW2 => "Enriching Our Worship 2",
            Source::LFF2018 => "Lesser Feasts and Fasts (2018)",
            Source::BookOfOccasionalServices2018 => "Book of Occasional Services (2018)",
            Source::LibroDeOracionComun => "Libro de Oración Común",
            Source::LiturgicalResources1 => "Liturgical Resources 1",
            Source::LiturgicalResources2 => "Liturgical Resources 2",
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Source::BCP1979 => "BCP",
            Source::EOW1 => "EOW 1",
            Source::EOW2 => "EOW 2",
            Source::LFF2018 => "LFF 2018",
            Source::LibroDeOracionComun => "LOC",
            Source::LiturgicalResources1 => "LR 1",
            Source::LiturgicalResources2 => "LR 2",
            Source::BookOfOccasionalServices2018 => "BOS 2018",
        };
        write!(f, "{}", name)
    }
}

impl Default for Source {
    fn default() -> Self {
        Self::BCP1979
    }
}
