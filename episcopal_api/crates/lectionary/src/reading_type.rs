use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
