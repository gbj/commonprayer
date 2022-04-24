pub mod burial;
pub mod canticles;
pub mod collects;
pub mod eucharist;

pub use canticles::*;
use liturgy::*;

lazy_static! {
    pub static ref GLORIA_PATRI_TRADITIONAL: GloriaPatri = GloriaPatri::from((
        "Glory be to the Father, and to the Son, ",
        "and to the Holy Ghost: ",
        "as it was in the beginning, is now, and ever shall be, ",
        "world without end. Amen. "
    ));

    pub static ref APOSTLES_CREED_TRADITIONAL: Document = Document::new()
        .version(Version::RiteI)
        .page(53)
        .label("The Apostles’ Creed")
        .content(Content::Series(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading3, "The Apostles’ Creed"))),
            Document::from(Rubric::from("Officiant and People together, all standing")),
            Document::from(Text::from("I believe in God, the Father almighty,\n	maker of heaven and earth;\nAnd in Jesus Christ his only Son our Lord;\n	who was conceived by the Holy Ghost,\n	born of the Virgin Mary,\n	suffered under Pontius Pilate,\n	was crucified, dead, and buried.\n	He descended into hell.\nThe third day he rose again from the dead.\n	He ascended into heaven,\n	and sitteth on the right hand of God the Father almighty.\n	From thence he shall come to judge the quick and the dead.\nI believe in the Holy Ghost,\n	the holy catholic Church,\n	the communion of saints,\n	the forgiveness of sins,\n	the resurrection of the body,\n	and the life everlasting. Amen.").display_format(DisplayFormat::Unison))
        ])));

    pub static ref PASCHA_NOSTRUM_TRADITIONAL: Invitatory = Invitatory {
        local_name: String::from("Christ our Passover"),
        citation: Some(String::from("1 Corinthians 5:7-8; Romans 6:9-11; 1 Corinthians 15:20-22 ")),
        latin_name: Some(String::from("Pascha Nostrum")),
        antiphon: SeasonalAntiphon::Omit,
        gloria_patri: None,
        sections: vec![
                InvitatorySection {
                        verses: vec![
                                InvitatoryVerse {
                                        a: String::from("Alleluia.\nChrist our Passover is sacrificed for us; * "),
                                        b: String::from("therefore let us keep the feast,")
                                },
                                InvitatoryVerse {
                                        a: String::from("Not with old leaven,\nneither with the leaven of malice and wickedness, *"),
                                        b: String::from("but with the unleavened bread of sincerity and truth. Alleluia.")
                                }
                        ]
                },
                InvitatorySection {
                        verses: vec![
                                InvitatoryVerse {
                                        a: String::from("Christ being raised from the dead dieth no more; *"),
                                        b: String::from("death hath no more dominion over him.")
                                },
                                InvitatoryVerse {
                                        a: String::from("For in that he died, he died unto sin once; *"),
                                        b: String::from("but in that he liveth, he liveth unto God.")
                                },
                                InvitatoryVerse {
                                        a: String::from("Likewise reckon ye also yourselves to be dead indeed unto sin, *"),
                                        b: String::from("but alive unto God through Jesus Christ our Lord. Alleluia.")
                                }
                        ]
                },
                InvitatorySection {
                        verses: vec![
                                InvitatoryVerse {
                                        a: String::from("Christ is risen from the dead, *"),
                                        b: String::from("and become the first fruits of them that slept.")
                                },
                                InvitatoryVerse {
                                        a: String::from("For since by man came death, * "),
                                        b: String::from("by man came also the resurrection of the dead.")
                                },
                                InvitatoryVerse {
                                        a: String::from("For as in Adam all die, *"),
                                        b: String::from("even so in Christ shall all be made alive. Alleluia.")
                                }
                        ]
                }
        ]
    };
}
