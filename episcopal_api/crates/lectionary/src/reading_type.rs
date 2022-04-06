use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Serialize, Deserialize, Display, EnumIter, EnumString)]
pub enum ReadingType {
    Empty,
    // Special Services
    PalmsGospel,
    // Easter Vigil and Pentecost Vigil
    Vigil1,
    VigilPsalm1,
    Vigil2,
    VigilPsalm2,
    Vigil3,
    VigilPsalm3,
    Vigil4,
    VigilPsalm4,
    Vigil5,
    VigilPsalm5,
    Vigil6,
    VigilPsalm6,
    Vigil7,
    VigilPsalm7,
    Vigil8,
    VigilPsalm8,
    Vigil9,
    VigilPsalm9,
    // Ordinary Office/Eucharistic Readings
    MorningPsalm,
    EveningPsalm,
    FirstReadingAlternateYear,
    FirstReading,
    Psalm,
    SecondReading,
    Gospel,
    // Holy Day office readings
    Morning1,
    Morning2,
    Evening1,
    Evening2,
}

impl Default for ReadingType {
    fn default() -> Self {
        Self::Empty
    }
}

pub const VIGIL_READING_TYPES: [ReadingType; 18] = [
    ReadingType::Vigil1,
    ReadingType::VigilPsalm1,
    ReadingType::Vigil2,
    ReadingType::VigilPsalm2,
    ReadingType::Vigil3,
    ReadingType::VigilPsalm3,
    ReadingType::Vigil4,
    ReadingType::VigilPsalm4,
    ReadingType::Vigil5,
    ReadingType::VigilPsalm5,
    ReadingType::Vigil6,
    ReadingType::VigilPsalm6,
    ReadingType::Vigil7,
    ReadingType::VigilPsalm7,
    ReadingType::Vigil8,
    ReadingType::VigilPsalm8,
    ReadingType::Vigil9,
    ReadingType::VigilPsalm9,
];

impl ReadingType {
    pub fn is_psalm(&self) -> bool {
        matches!(
            self,
            ReadingType::Psalm
                | ReadingType::MorningPsalm
                | ReadingType::EveningPsalm
                | ReadingType::VigilPsalm1
                | ReadingType::VigilPsalm2
                | ReadingType::VigilPsalm3
                | ReadingType::VigilPsalm4
                | ReadingType::VigilPsalm5
                | ReadingType::VigilPsalm6
                | ReadingType::VigilPsalm7
                | ReadingType::VigilPsalm8
                | ReadingType::VigilPsalm9
        )
    }
}
