use lazy_static::lazy_static;
use lectionary::ReadingType;
use liturgy::*;

use crate::rite2::EUCHARIST_INTROS_II;

lazy_static! {
    pub static ref CONCERNING_THE_SERVICE: Document = Document::new()
        .page(298)
        .label("Concerning the Service")
        .version(Version::BCP1979)
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((
                HeadingLevel::Heading2,
                "Concerning the Service"
            ))),
            Document::from(Rubric::from("Holy Baptism is full initiation by water and the Holy Spirit into Christ’s Body the Church. The bond which God establishes in Baptism is indissoluble.\n\nHoly Baptism is appropriately administered within the Eucharist as the chief service on a Sunday or other feast.\n\nThe bishop, when present, is the celebrant; and is expected to preach the Word and preside at Baptism and the Eucharist. At Baptism, the bishop officiates at the Presentation and Examination of the Candidates; says the Thanksgiving over the Water; [consecrates the Chrism;] reads the prayer, “Heavenly Father, we thank you that by water and the Holy Spirit;” and officiates at what follows.\n\nIn the absence of a bishop, a priest is the celebrant and presides at the service. If a priest uses Chrism in signing the newly baptized, it must have been previously consecrated by the bishop.\n\nEach candidate for Holy Baptism is to be sponsored by one or more baptized persons.\n\nSponsors of adults and older children present their candidates and thereby signify their endorsement of the candidates and their intention to support them by prayer and example in their Christian life. Sponsors of infants, commonly called godparents, present their candidates, make promises in their own names, and also take vows on behalf of their candidates.\n\nIt is fitting that parents be included among the godparents of their own children. Parents and godparents are to be instructed in the meaning of Baptism, in their duties to help the new Christians grow in the knowledge and love of God, and in their responsibilities as members of his Church").long()),
            Document::from(Content::DocumentLink(Version::BCP1979, "Additional Directions".into(), "baptism".into(), "additional-directions".into()))
        ]))));

    pub static ref HOLY_BAPTISM: Document = Document::new()
        .page(299)
        .label("Baptism")
        .version(Version::BCP1979)
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((
                HeadingLevel::Heading1,
                "Holy Baptism"
            ))),
            Document::from(Heading::InsertDate),
            Document::from(Heading::InsertDay),

            Document::from(Rubric::from("A hymn, psalm, or anthem may be sung.")),
            Document::from(Content::HymnLink(HymnLink::Tag("Baptism".into()))),
            Document::from(Rubric::from("The people standing, the Celebrant says")),

            // Intro
            EUCHARIST_INTROS_II.clone(),
            Document::from(Rubric::from("The Celebrant then continues")),
            Document::from(Preces::from([
                ("", "There is one Body and one Spirit;"),
                ("People", "There is one hope in God’s call to us;"),
                ("Celebrant", "One Lord, one Faith, one Baptism;"),
                ("People", "One God and Father of all.")
            ])),
            Document::from(Preces::from([
                ("Celebrant", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Let us pray."),
            ])),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Collect of the Day"))),
            Document::from(Content::CollectOfTheDay { allow_multiple: false }),
            Document::from(Rubric::from("At the principal service on a Sunday or other feast, the Collect and Lessons are properly those of the Day. On other occasions they are selected from “At Baptism.” (See Additional Directions, page 312.)")),
            Document::from(Content::DocumentLink(Version::BCP1979, "Additional Directions".into(), "baptism".into(), "additional-directions".into())).display(Show::TemplateOnly),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Lessons"))),

            // Readings Template
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The people sit. One or two Lessons, as appointed, are read, the Reader first saying")),
                Document::from(Text::from("A Reading (Lesson) from _____________.")),
                Document::from(Rubric::from("A citation giving chapter and verse may be added.")),
                Document::from(Rubric::from("After each Reading, the Reader may say")),
                Document::from(Choice::from(vec![
                    Document::from(Preces::from([
                        ("", "The Word of the Lord"),
                        ("People", "Thanks be to God.")
                    ])),
                    Document::from("Here ends the Reading (Epistle).")
                ])),
                Document::from(Rubric::from("Silence may follow.")),
                Document::from(Rubric::from("A Psalm, hymn, or anthem may follow each Reading.")),
                Document::from(Rubric::from("Then, all standing, the Deacon or a Priest reads the Gospel, first saying")),
                Document::from(Preces::from([
                    ("", "The Holy Gospel of our Lord Jesus Christ according to ____________."),
                    ("People", "Glory to you, Lord Christ.")
                ])),
                Document::from(Rubric::from("After the Gospel, the Reader says")),
                Document::from(Preces::from([
                    ("", "The Gospel of the Lord."),
                    ("People", "Praise to you, Lord Christ.")
                ]))
            ])).display(Show::TemplateOnly),

            // Actual readings inserted
            Document::from(Series::from(vec![
                Document::from(LectionaryReading {
                    reading_type: ReadingTypeTable::Selected(ReadingType::FirstReading),
                    reading_type_overridden_by: None,
                    lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
                    intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from {{long_name}}."))),
                }).label("The First Lesson"),
                Document::from(Choice::from(vec![
                    Document::from(Preces::from([
                        ("", "The Word of the Lord"),
                        ("People", "Thanks be to God.")
                    ])),
                    Document::from("Here ends the Reading (Epistle).")
                ])),

                Document::from(LectionaryReading {
                    reading_type: ReadingTypeTable::Selected(ReadingType::Psalm),
                    reading_type_overridden_by: None,
                    lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
                    intro: None,
                }),

                Document::from(LectionaryReading {
                    reading_type: ReadingTypeTable::Selected(ReadingType::SecondReading),
                    reading_type_overridden_by: None,
                    lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
                    intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from {{long_name}}."))),
                }).label("The Second Lesson"),
                Document::from(Choice::from(vec![
                    Document::from(Preces::from([
                        ("", "The Word of the Lord"),
                        ("People", "Thanks be to God.")
                    ])),
                    Document::from("Here ends the Reading (Epistle).")
                ])),

                Document::from(LectionaryReading {
                    reading_type: ReadingTypeTable::Selected(ReadingType::Gospel),
                    reading_type_overridden_by: None,
                    lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
                    intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to {{short_name}}."),
                        ("People", "Glory to you, Lord Christ.")
                    ])))),
                }).label("The Gospel"),
                Document::from(Preces::from([
                    ("", "The Gospel of the Lord."),
                    ("People", "Praise to you, Lord Christ.")
                ]))
            ])).display(Show::CompiledOnly)
        ]))
        .preferences([
            LiturgyPreference::from((
            PreferenceKey::from(GlobalPref::Lectionary),
            "Lectionary",
            [
                LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
                LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
            ]
            )).category("Lectionary"),
        ]))
    );

    pub static ref ADDITIONAL_DIRECTIONS: Document = Document::new()
        .page(312)
        .label("Additional Directions")
        .version(Version::BCP1979)
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((
                HeadingLevel::Heading2,
                "Additional Directions"
            ))),
            Document::from(Rubric::from("Holy Baptism is especially appropriate at the Easter Vigil, on the Day of Pentecost, on All Saints’ Day or the Sunday after All Saints’ Day, and on the Feast of the Baptism of our Lord (the First Sunday after the Epiphany). It is recommended that, as far as possible, Baptisms be reserved for these occasions or when a bishop is present.\n\nIf on any one of the above-named days the ministry of a bishop or priest cannot be obtained, the bishop may specially authorize a deacon to preside. In that case, the deacon omits the prayer over the candidates, page 308, and the formula and action which follow.\n\nThese omitted portions of the rite may be administered on some subsequent occasion of public baptism at which a bishop or priest presides.\n\nIf on the four days listed above there are no candidates for Baptism, the Renewal of Baptismal Vows, page 292, may take the place of the Nicene Creed at the Eucharist.\n\nIf desired, the hymn Gloria in excelsis may be sung immediately after the opening versicles and before the salutation “The Lord be with you.” When a bishop is present, or on other occasions for sufficient reason, the Collect (page 203 or 254) and one or more of the Lessons provided for use at Baptism (page 928) may be substituted for the Proper of the Day.\n\nLay persons may act as readers, and it is appropriate for sponsors to be assigned this function. The petitions (page 305) may also be led by one of the sponsors.\n\nThe Nicene Creed is not used at this service.\n\nIf the Presentation of the Candidates does not take place at the font, then before or during the petitions (page 305), the ministers, candidates, and sponsors go to the font for the Thanksgiving over the Water.\n\nIf the movement to the font is a formal procession, a suitable psalm, such as Psalm 42, or a hymn or anthem, may be sung.\n\nWhere practicable, the font is to be filled with clean water immediately before the Thanksgiving over the Water.\n\nAt the Thanksgiving over the Water, and at the administration of Baptism, the celebrant, whenever possible, should face the people across the font, and the sponsors should be so grouped that the people may have a clear view of the action.\n\nAfter the Baptism, a candle (which may be lighted from the Paschal Candle) may be given to each of the newly baptized or to a godparent. It may be found desirable to return to the front of the church for the prayer, “Heavenly Father, we thank you that by water and the Holy Spirit,” and the ceremonies that follow it. A suitable psalm, such as Psalm 23, or a hymn or anthem, may be sung during the procession.\n\nThe oblations of bread and wine at the baptismal Eucharist may be presented by the newly baptized or their godparents.").long()),

            Document::from(Heading::from((HeadingLevel::Heading3, "Conditional Baptism"))),
            Document::from(Rubric::from("If there is reasonable doubt that a person has been baptized with water, “In the Name of the Father, and of the Son, and of the Holy Spirit” (which are the essential parts of Baptism), the person is baptized in the usual manner, but this form of words is used")),
            Document::from(Text::from("If you are not already baptized, *N.*, I baptize you in the Name of the Father, and of the Son, and of the Holy Spirit")),

            Document::from(Heading::from((HeadingLevel::Heading3, "Emergency Baptism"))),
            Document::from(Rubric::from("In case of emergency, any baptized person may administer Baptism according to the following form.\n\nUsing the given name of the one to be baptized (if known), pour water on him or her, saying")),
            Document::from("I baptize you in the Name of the Father, and of the Son, and of the Holy Spirit."),
            Document::from(Rubric::from("The Lord’s Prayer is then said.\n\nOther prayers, such as the following, may be added")),
            Document::from("Heavenly Father, we thank you that by water and the Holy Spirit you have bestowed upon this your servant the forgiveness of sin and have raised *him* to the new life of grace. Strengthen *him*, O Lord, with your presence, enfold *him* in the arms of your mercy, and keep *him* safe for ever."),
            Document::from(Rubric::from("The person who administers emergency Baptism should inform the priest of the appropriate parish, so that the fact can be properly registered.\n\nIf the baptized person recovers, the Baptism should be recognized at a public celebration of the Sacrament with a bishop or priest presiding, and the person baptized under emergency conditions, together with the sponsors or godparents, taking part in everything except the administration of the water."))
        ]))));
}
