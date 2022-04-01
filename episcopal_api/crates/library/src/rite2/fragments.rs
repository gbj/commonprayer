use liturgy::*;

use crate::conditions::{EASTER_SEASON, LENT};

pub const LORDS_PRAYER_TRADITIONAL_TEXT: &str = "Our Father, who art in heaven,\n	hallowed be thy Name,\n	thy kingdom come,\n	thy will be done,\n	on earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n	as we forgive those\n	who trespass against us.\nAnd lead us not into temptation,\n	but deliver us from evil.\nFor thine is the kingdom,\n	and the power, and the glory,\n	for ever and ever.";
pub const LORDS_PRAYER_CONTEMPORARY_TEXT: &str = "Our Father in heaven,\n	hallowed be your Name,\n	your kingdom come,\n	your will be done,\n	on earth as in heaven.\nGive us today our daily bread.\nForgive us our sins,\n	as we forgive those\n	who sin against us.\nSave us from the time of trial,\n	and deliver us from evil.\nFor the kingdom, the power,\n	and the glory are yours,\n	now and for ever.";

lazy_static! {
    pub static ref WORD_OF_THE_LORD: Preces = Preces::from([
        ("", "The Word of the Lord."),
        ("People", "Thanks be to God.")
    ]);

    pub static ref GLORIA_PATRI: GloriaPatri = GloriaPatri::from((
      "Glory to the Father, and to the Son, ",
      "and to the Holy Spirit: ",
      "as it was in the beginning, is now, ",
      "and will be for ever. Amen. "
    ));

    pub static ref LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL: Choice = Choice::from([
      Document::from(Text::from(LORDS_PRAYER_TRADITIONAL_TEXT).response("Amen.").display_format(DisplayFormat::Unison)).label("The Lord’s Prayer").version_label("Traditional"),
      Document::from(Text::from(LORDS_PRAYER_CONTEMPORARY_TEXT).response("Amen.").display_format(DisplayFormat::Unison)).label("The Lord’s Prayer").version_label("Contemporary")
    ]);

    pub static ref LORDS_PRAYER_ABBREV: Choice = Choice::from([
      Document::from(
        Text::from("Our Father, who art in heaven,\n\thallowed be thy Name,\n\tthy kingdom come,\n\tthy will be done,\n\ton earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n\tas we forgive those\n\twho trespass against us.\nAnd lead us not into temptation,\n\tbut deliver us from evil.")
          .display_format(DisplayFormat::Unison)
      ).version_label("Traditional"),
      Document::from(
        Text::from("Our Father in heaven,\n\thallowed be your Name,\n\tyour kingdom come,\n\tyour will be done,\n\ton earth as in heaven.\nGive us today our daily bread.\nForgive us our sins,\n\tas we forgive those\n\twho sin against us.\nSave us from the time of trial,\n\tand deliver us from evil.")
          .display_format(DisplayFormat::Unison)
      ).version_label("Contemporary")
    ]);

    pub static ref APOSTLES_CREED: Text = Text::from("I believe in God, the Father almighty,\n	creator of heaven and earth.\nI believe in Jesus Christ, his only Son, our Lord.\n	He was conceived by the power of the Holy Spirit\n	and born of the Virgin Mary.\n	He suffered under Pontius Pilate,\n	was crucified, died, and was buried. \n	He descended to the dead.\n	On the third day he rose again.\n	He ascended into heaven,\n	and is seated at the right hand of the Father.\n	He will come again to judge the living and the dead.\nI believe in the Holy Spirit,\n	the holy catholic Church,\n	the communion of saints,\n	the forgiveness of sins,\n	the resurrection of the body,\n	and the life everlasting.")
      .response("Amen.")
      .display_format(DisplayFormat::Unison);

    pub static ref PASCHA_NOSTRUM: Invitatory = Invitatory {
        local_name: String::from("Christ our Passover"),
        citation: Some(String::from("1 Corinthians 5:7-8; Romans 6:9-11; 1 Corinthians 15:20-22 ")),
        latin_name: Some(String::from("Pascha Nostrum")),
        antiphon: SeasonalAntiphon::Omit,
        gloria_patri: None,
        sections: vec![
                InvitatorySection {
                        verses: vec![
                                InvitatoryVerse {
                                        a: String::from("Alleluia.\nChrist our Passover has been sacrificed for us; * "),
                                        b: String::from("therefore let us keep the feast,")
                                },
                                InvitatoryVerse {
                                        a: String::from("Not with the old leaven, the leaven of malice and evil, *"),
                                        b: String::from("but with the unleavened bread of sincerity and truth. Alleluia.")
                                }
                        ]
                },
                InvitatorySection {
                        verses: vec![
                                InvitatoryVerse {
                                        a: String::from("Christ being raised from the dead will never die again; *"),
                                        b: String::from("death no longer has dominion over him.")
                                },
                                InvitatoryVerse {
                                        a: String::from("The death that he died, he died to sin, once for all; *"),
                                        b: String::from("but the life he lives, he lives to God.")
                                },
                                InvitatoryVerse {
                                        a: String::from("So also consider yourselves dead to sin, *"),
                                        b: String::from("and alive to God in Jesus Christ our Lord. Alleluia.")
                                }
                        ]
                },
                InvitatorySection {
                        verses: vec![
                                InvitatoryVerse {
                                        a: String::from("Christ has been raised from the dead, *"),
                                        b: String::from("the first fruits of those who have fallen asleep.")
                                },
                                InvitatoryVerse {
                                        a: String::from("For since by a man came death, * "),
                                        b: String::from("by a man has come also the resurrection of the dead.")
                                },
                                InvitatoryVerse {
                                        a: String::from("For as in Adam all die, *"),
                                        b: String::from("so also in Christ shall all be made alive. Alleluia.")
                                }
                        ]
                }
        ]
    };

    pub static ref EUCHARIST_INTROS_II: Document = Document::from(Series::from(vec![
        // Default opening
        Document::from(Preces::from([
            ("", "Blessed be God: Father, Son, and Holy Spirit."),
            ("People", "And blessed be his kingdom, now and for ever. Amen.")
        ])).condition(Condition::All(vec![
            Condition::Not(Box::new(EASTER_SEASON.clone())),
            Condition::Not(Box::new(LENT.clone())),
        ])),

        // Easter opening
        Document::from(Rubric::from("In place of the above, from Easter Day through the Day of Pentecost")).display(Show::TemplateOnly),
        Document::from(Preces::from([
            ("Celebrant", "Alleluia. Christ is risen."),
            ("People", "The Lord is risen indeed. Alleluia.")
        ])).condition(EASTER_SEASON.clone()),

        // Lent opening
        Document::from(Rubric::from("In Lent and on other penitential occasions")).display(Show::TemplateOnly),
        Document::from(Preces::from([
            ("Celebrant", "Bless the Lord who forgives all our sins;"),
            ("People", "His mercy endures for ever.")
        ])).condition(LENT.clone()),
    ]));
}
