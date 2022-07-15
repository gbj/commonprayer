use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CanticleId {
    None,
    Canticle1,
    Canticle2,
    Canticle3,
    Canticle4,
    Canticle5,
    Canticle6,
    Canticle7,
    Canticle8,
    Canticle9,
    Canticle10,
    Canticle11,
    Canticle12,
    Canticle13,
    Canticle14,
    Canticle15,
    Canticle16,
    Canticle17,
    Canticle18,
    Canticle19,
    Canticle20,
    Canticle21,
    CanticleA,
    CanticleB,
    CanticleC,
    CanticleD,
    CanticleE,
    CanticleF,
    CanticleG,
    CanticleH,
    CanticleI,
    CanticleJ,
    CanticleK,
    CanticleL,
    CanticleM,
    CanticleN,
    CanticleO,
    CanticleP,
    CanticleQ,
    CanticleR,
    CanticleS,
}

impl Default for CanticleId {
    fn default() -> Self {
        Self::None
    }
}

impl Display for CanticleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CanticleId::None => "None",
                CanticleId::Canticle1 => "1",
                CanticleId::Canticle2 => "2",
                CanticleId::Canticle3 => "3",
                CanticleId::Canticle4 => "4",
                CanticleId::Canticle5 => "5",
                CanticleId::Canticle6 => "6",
                CanticleId::Canticle7 => "7",
                CanticleId::Canticle8 => "8",
                CanticleId::Canticle9 => "9",
                CanticleId::Canticle10 => "10",
                CanticleId::Canticle11 => "11",
                CanticleId::Canticle12 => "12",
                CanticleId::Canticle13 => "13",
                CanticleId::Canticle14 => "14",
                CanticleId::Canticle15 => "15",
                CanticleId::Canticle16 => "16",
                CanticleId::Canticle17 => "17",
                CanticleId::Canticle18 => "18",
                CanticleId::Canticle19 => "19",
                CanticleId::Canticle20 => "20",
                CanticleId::Canticle21 => "21",
                CanticleId::CanticleA => "A",
                CanticleId::CanticleB => "B",
                CanticleId::CanticleC => "C",
                CanticleId::CanticleD => "D",
                CanticleId::CanticleE => "E",
                CanticleId::CanticleF => "F",
                CanticleId::CanticleG => "G",
                CanticleId::CanticleH => "H",
                CanticleId::CanticleI => "I",
                CanticleId::CanticleJ => "J",
                CanticleId::CanticleK => "K",
                CanticleId::CanticleL => "L",
                CanticleId::CanticleM => "M",
                CanticleId::CanticleN => "N",
                CanticleId::CanticleO => "O",
                CanticleId::CanticleP => "P",
                CanticleId::CanticleQ => "Q",
                CanticleId::CanticleR => "R",
                CanticleId::CanticleS => "S",
            },
        )
    }
}

impl TryFrom<&str> for CanticleId {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let without_citation = value.replace("canticle-", "");
        let without_citation = without_citation.replace("Canticle", "");
        let without_citation = without_citation.replace("Cántico", "");
        let without_citation = without_citation.trim();
        match without_citation {
            "1" => Ok(CanticleId::Canticle1),
            "2" => Ok(CanticleId::Canticle2),
            "3" => Ok(CanticleId::Canticle3),
            "4" => Ok(CanticleId::Canticle4),
            "5" => Ok(CanticleId::Canticle5),
            "6" => Ok(CanticleId::Canticle6),
            "7" => Ok(CanticleId::Canticle7),
            "8" => Ok(CanticleId::Canticle8),
            "9" => Ok(CanticleId::Canticle9),
            "10" => Ok(CanticleId::Canticle10),
            "11" => Ok(CanticleId::Canticle11),
            "12" => Ok(CanticleId::Canticle12),
            "13" => Ok(CanticleId::Canticle13),
            "14" => Ok(CanticleId::Canticle14),
            "15" => Ok(CanticleId::Canticle15),
            "16" => Ok(CanticleId::Canticle16),
            "17" => Ok(CanticleId::Canticle17),
            "18" => Ok(CanticleId::Canticle18),
            "19" => Ok(CanticleId::Canticle19),
            "20" => Ok(CanticleId::Canticle20),
            "21" => Ok(CanticleId::Canticle21),
            "A" => Ok(CanticleId::CanticleA),
            "B" => Ok(CanticleId::CanticleB),
            "C" => Ok(CanticleId::CanticleC),
            "D" => Ok(CanticleId::CanticleD),
            "E" => Ok(CanticleId::CanticleE),
            "F" => Ok(CanticleId::CanticleF),
            "G" => Ok(CanticleId::CanticleG),
            "H" => Ok(CanticleId::CanticleH),
            "I" => Ok(CanticleId::CanticleI),
            "J" => Ok(CanticleId::CanticleJ),
            "K" => Ok(CanticleId::CanticleK),
            "L" => Ok(CanticleId::CanticleL),
            "M" => Ok(CanticleId::CanticleM),
            "N" => Ok(CanticleId::CanticleN),
            "O" => Ok(CanticleId::CanticleO),
            "P" => Ok(CanticleId::CanticleP),
            "Q" => Ok(CanticleId::CanticleQ),
            "R" => Ok(CanticleId::CanticleR),
            "S" => Ok(CanticleId::CanticleS),
            _ => Err(()),
        }
    }
}
