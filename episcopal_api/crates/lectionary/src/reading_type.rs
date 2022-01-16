use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ReadingType {
    Empty,
    FirstReadingAlternateYear,
    FirstReading,
    Psalm,
    SecondReading,
    Gospel,
    MorningPsalm,
    EveningPsalm,
    Morning1,
    Morning2,
    Evening1,
    Evening2,
    // Special Services
    PalmsGospel,
    // Easter Vigil and Pentecost Vigil
    Vigil1,
    Vigil2,
    Vigil3,
    Vigil4,
    Vigil5,
    Vigil6,
    Vigil7,
    Vigil8,
    Vigil9,
    VigilPsalm1,
    VigilPsalm2,
    VigilPsalm3,
    VigilPsalm4,
    VigilPsalm5,
    VigilPsalm6,
    VigilPsalm7,
    VigilPsalm8,
    VigilPsalm9,
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
