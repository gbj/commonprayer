use calendar::Season;
use lazy_static::lazy_static;
use lectionary::ReadingType;
use liturgy::*;
pub mod parallel;
use parallel::*;

use crate::bcp1979;

lazy_static! {
    pub static ref GLORIA_IN_EXCELSIS: Text = Text::from("Glory to God in the highest,\n\tand peace to his people on earth. \n\nLord God, heavenly King,\nalmighty God and Father,\n\twe worship you, we give you thanks,\n\twe praise you for your glory. \n\nLord Jesus Christ, only Son of the Father,\nLord God, Lamb of God,\nyou take away the sin of the world:\n\thave mercy on us;\nyou are seated at the right hand of the Father:\n\treceive our prayer. \n\nFor you alone are the Holy One,\nyou alone are the Lord,\nyou alone are the Most High,\n\tJesus Christ,\n\twith the Holy Spirit,\n\tin the glory of God the Father. Amen.");

    pub static ref KYRIE_ELEISON: Choice = Choice::from(vec![
        Document::from(ResponsivePrayer::from([
            "Lord, have mercy.",
            "Christ, have mercy.",
            "Lord, have mercy."
        ])).version_label("Lord, have mercy."),
        Document::from(ResponsivePrayer::from([
            "Kyrie eleison.",
            "Christe eleison.",
            "Kyrie eleison."
        ])).version_label("Kyrie eleison.")
    ]);

    pub static ref TRISAGION: ResponsivePrayer = ResponsivePrayer::from([
        "Holy God,\nHoly and Mighty,\nHoly Immortal One,",
        "Have mercy upon us."
    ]);

    pub static ref NICENE_CREED_II: Document = Document::from(Text::from("We believe in one God,\n\tthe Father, the Almighty,\n\tmaker of heaven and earth,\n\tof all that is, seen and unseen.\n\nWe believe in one Lord, Jesus Christ,\n\tthe only Son of God,\n\teternally begotten of the Father,\n\tGod from God, Light from Light,\n\ttrue God from true God,\n\tbegotten, not made,\n\tof one Being with the Father.\n\tThrough him all things were made.\n\tFor us and for our salvation\n\t\the came down from heaven:\n\tby the power of the Holy Spirit\n\t\the became incarnate from the Virgin Mary,\n\t\tand was made man.\n\tFor our sake he was crucified under Pontius Pilate;\n\t\the suffered death and was buried.\n\t\tOn the third day he rose again\n\t\t\tin accordance with the Scriptures;\n\t\the ascended into heaven\n\t\t\tand is seated at the right hand of the Father.\n\tHe will come again in glory to judge the living and the dead,\n\t\tand his kingdom will have no end.\n\nWe believe in the Holy Spirit, the Lord, the giver of life,\n\twho proceeds from the Father and the Son.\n\tWith the Father and the Son he is worshiped and glorified.\n\tHe has spoken through the Prophets.\n\tWe believe in one holy catholic and apostolic Church.\n\tWe acknowledge one baptism for the forgiveness of sins.\n\tWe look for the resurrection of the dead,\n\t\tand the life of the world to come. Amen.").display_format(DisplayFormat::Unison)).label("The Nicene Creed").page(358);

    pub static ref HOLY_EUCHARIST_II: Document = Document::new()
        .label("Eucharist")
        .page(355)
        .version(Version::RiteII)
        .content(Liturgy::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Holy Eucharist:\nRite Two"))).tags([TITLE]),
            Document::from(Heading::InsertDate),
            Document::from(Heading::InsertDay),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Word of God"))).tags([WORD_OF_GOD]),
            Document::from(Rubric::from("A hymn, psalm, or anthem may be sung.")).tags([OPENING_HYMN]),
            Document::from(HymnLink::Hymnals).tags([OPENING_HYMN]),

            Document::from(Rubric::from("The people standing, the Celebrant says")).tags([OPENING_ACCLAMATION]),
            Document::from(Preces::from([
                ("", "Blessed be God: Father, Son, and Holy Spirit."),
                ("People", "And blessed be his kingdom, now and for ever.\nAmen.")
            ])).condition(Condition::None(vec![
                Condition::Season(Season::Lent),
                Condition::Season(Season::HolyWeek),
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).tags([OPENING_ACCLAMATION]),

            Document::new()
                .display(Show::TemplateOnly)
                .content(Rubric::from("In place of the above, from Easter Day through the Day of Pentecost"))
                .tags([OPENING_ACCLAMATION_EASTER]),
            Document::from(Preces::from([
                ("Celebrant", "Alleluia. Christ is risen."),
                ("People", "The Lord is risen indeed. Alleluia.")
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).tags([OPENING_ACCLAMATION_EASTER]),
            Document::new().display(Show::TemplateOnly)
                .content(Rubric::from("In Lent and on other penitential occasions"))
                .tags([OPENING_ACCLAMATION_LENT]),
            Document::from(Preces::from([
                ("Celebrant", "Bless the Lord who forgives all our sins."),
                ("People", "His mercy endures for ever.")
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Lent),
                Condition::Season(Season::HolyWeek)
            ])).tags([OPENING_ACCLAMATION_LENT]),

            Document::from(Rubric::from("The Celebrant may say")).tags([COLLECT_FOR_PURITY]),
            Document::from(Text::from("Almighty God, to you all hearts are open, all desires known, and from you no secrets are hid: Cleanse the thoughts of our hearts by the inspiration of your Holy Spirit, that we may perfectly love you, and worthily magnify your holy Name; through Christ our Lord.").response("Amen.")).tags([COLLECT_FOR_PURITY]),

            Document::from(Choice::from(vec![
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("When appointed, the following hymn or some other song of praise is sung or said, all standing")),
                    Document::from(GLORIA_IN_EXCELSIS.clone()),
                ])).version_label("Gloria").tags([GLORIA_IN_EXCELSIS_TAG]),
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("On other occasions the following is used")),
                    Document::from(ResponsivePrayer::from([
                        "Lord, have mercy.",
                        "Christ, have mercy.",
                        "Lord, have mercy."
                    ]))
                ])).version_label("Kyrie (“Lord, have mercy.”)").tags([KYRIE_TAG]),
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("On other occasions the following is used")),
                    Document::from(ResponsivePrayer::from([
                        "Kyrie eleison.",
                        "Christe eleison.",
                        "Kyrie eleison."
                    ]))
                ])).version_label("Kyrie (“Kyrie eleison.”)").tags([KYRIE_TAG]),
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("or this")),
                    Document::from(TRISAGION.clone())
                ])).version_label("Trisagion").tags([TRISAGION_TAG])
            ])).tags([GLORIA_SECTION_TAG]),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Collect of the Day"))).tags([COLLECT_OF_THE_DAY]),
            Document::from(Rubric::from("The Celebrant says to the people")).tags([COLLECT_OF_THE_DAY]),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Let us pray.")
            ])).tags([COLLECT_OF_THE_DAY]),
            Document::from(Rubric::from("The Celebrant says the Collect.")).tags([COLLECT_OF_THE_DAY]),
            Document::new().version(Version::RiteII)
                .content(Content::CollectOfTheDay { allow_multiple: false })
                .tags([COLLECT_OF_THE_DAY]),
            Document::new().display(Show::TemplateOnly)
                .content(Preces::from([
                ("", ""),
                ("People", "Amen.")
            ])).tags([COLLECT_OF_THE_DAY]),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Lessons"))).tags([THE_LESSONS]),
            Document::new().display(Show::TemplateOnly)
                .content(Series::from(vec![
                    Document::from(Rubric::from("The people sit. One or two Lessons, as appointed, are read, the Reader first saying")),
                    Document::from(Text::from("A Reading (Lesson) from ____________.")),
                    Document::from(Rubric::from("A citation giving chapter and verse may be added.")),
                    Document::from(Rubric::from("After each Reading, the Reader may say")),
                    Document::from(Preces::from([
                        ("", "The Word of the Lord."),
                        ("People", "Thanks be to God.")
                    ])),
                    Document::from(Rubric::from("or the Reader may say")),
                    Document::from(Text::from("Here ends the Reading (Epistle).")),
                    Document::from(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::FirstReading),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Selected(Lectionaries::RCLTrack2),
                        intro: None,
                    }),
                    Document::from(Rubric::from("Silence may follow.\n\nA Psalm, hymn, or anthem may follow each Reading.")),
                    Document::from(Content::HymnLink(HymnLink::Hymnals)),
                    Document::from(Rubric::from("Then, all standing, the Deacon or a Priest reads the Gospel, first saying")),
                    Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to ____________."),
                        ("People", "Glory to you, Lord Christ.")
                    ])),
                    Document::from(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::Gospel),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Selected(Lectionaries::RCLTrack2),
                        intro: None,
                    }),
                    Document::from(Rubric::from("After the Gospel, the Reader says")),
                    Document::from(Preces::from([
                        ("", "The Gospel of the Lord."),
                        ("People", "Praise to you, Lord Christ.")
                    ])),
            ])).tags([LESSONS_RUBRICS]),
            Document::new().display(Show::CompiledOnly)
                .content(Series::from(vec![
                    Document::from(Rubric::from("The people sit.")).tags([LESSONS_RUBRICS]),
                    Document::new().label("The First Lesson")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Preference(PreferenceKey::Global(GlobalPref::ReadingA)),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Text::from("A Reading from {{long_name}}."))))
                    }).tags([FIRST_LESSON]),
                    Document::from(Preces::from([
                        ("", "The Word of the Lord."),
                        ("People", "Thanks be to God.")
                    ])),
                    Document::from(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::Psalm),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: None
                    }).tags([PSALM]),
                    Document::new().label("The Second Lesson")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Preference(PreferenceKey::Global(GlobalPref::ReadingB)),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Text::from("A Reading from {{long_name}}."))))
                    }).tags([SECOND_LESSON]),
                    Document::from(Preces::from([
                        ("", "The Word of the Lord."),
                        ("People", "Thanks be to God.")
                    ])).condition(Condition::Not(Box::new(Condition::Preference(PreferenceKey::from(GlobalPref::ReadingB), PreferenceValue::from(ReadingType::Empty))))),
                    Document::new().label("The Gospel")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::Gospel),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to {{short_name}}."),
                        ("People", "Glory to you, Lord Christ.")
                    ]))))
                }).tags([GOSPEL]),
                Document::from(Preces::from([
                    ("", "The Gospel of the Lord."),
                    ("People", "Praise to you, Lord Christ.")
                ])),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon"))).tags([SERMON]),
            Document::from(Rubric::from("On Sundays and other Major Feasts there follows, all standing")).tags([NICENE_CREED]),
            NICENE_CREED_II.clone().tags([NICENE_CREED]),

            Document::from(Heading::from((HeadingLevel::Heading3, "The Prayers of the People"))).tags([PRAYERS_OF_THE_PEOPLE]),
            Document::from(Rubric::from("Prayer is offered with intercession for\n\nThe Universal Church, its members, and its mission\nThe Nation and all in authority\nThe welfare of the world\nThe concerns of the local community\nThose who suffer and those in any trouble\nThe departed (with commemoration of a saint when appropriate)\n\nSee the forms beginning on page 383.")).tags([POP_RUBRIC]),
            Document::from(Content::DocumentLink {
                label: "Prayers of the People".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::PrayersOfThePeople, Slug::Version(Version::BCP1979)]),
                rotate: false,
                link_only: false
            }).tags([POP_FORMS]).display(Show::TemplateOnly),
            Document::from(Choice::from(bcp1979::eucharist::PRAYERS_OF_THE_PEOPLE.clone(),)),

            Document::from(Rubric::from("If there is no celebration of the Communion, or if a priest is not available, the service is concluded as directed on page 406.")).tags([NO_COMMUNION_RUBRIC]),
            Document::from(Heading::from((HeadingLevel::Heading3, "Confession of Sin"))).tags([CONFESSION]),
            Document::from(Rubric::from("A Confession of Sin is said here if it has not been said earlier. On occasion, the Confession may be omitted.")).tags([CONFESSION]),
            Document::from(Rubric::from("One of the sentences from the Penitential Order on page 351 may be said.")).tags([CONFESSION]),
            Document::from(Content::DocumentLink {
                label: "Penitential Sentences".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::PenitentialSentences, Slug::Version(Version::RiteII)]),
                rotate: false,
                link_only: false
            }).tags([CONFESSION]),
            Document::from(Rubric::from("The Deacon or Celebrant says")).tags([CONFESSION]),
            Document::from(Text::from("Let us confess our sins against God and our neighbor.")).tags([CONFESSION]),
            Document::from(Rubric::from("Silence may be kept.")).tags([CONFESSION]),
            Document::from(Rubric::from("Minister and People")).tags([CONFESSION]),
            Document::from(Text::from("Most merciful God,\nwe confess that we have sinned against you\nin thought, word, and deed,\nby what we have done,\nand by what we have left undone.\nWe have not loved you with our whole heart;\nwe have not loved our neighbors as ourselves.\nWe are truly sorry and we humbly repent.\nFor the sake of your Son Jesus Christ,\nhave mercy on us and forgive us;\nthat we may delight in your will,\nand walk in your ways,\nto the glory of your Name. Amen.").display_format(DisplayFormat::Unison)).tags([CONFESSION]),
            Document::from(Rubric::from("The Bishop when present, or the Priest, stands and says")).tags([CONFESSION]),
            Document::from(Text::from("Almighty God have mercy on you, forgive you all your sins through our Lord Jesus Christ, strengthen you in all goodness, and by the power of the Holy Spirit keep you in eternal life.").response("Amen.")).tags([CONFESSION]),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Peace"))).tags([THE_PEACE]),
            Document::from(Rubric::from("All stand. The Celebrant says to the people")).tags([THE_PEACE]),
            Document::from(Preces::from([
                ("", "The peace of the Lord be always with you."),
                ("People", "And also with you.")
            ])).tags([THE_PEACE]),
            Document::from(Rubric::from("Then the Ministers and People may greet one another in the name of the Lord.")).tags([THE_PEACE]),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Holy Communion"))).tags([HOLY_COMMUNION]),
            Document::from(Rubric::from("The Celebrant may begin the Offertory with one of the sentences on page 376, or with some other sentence of Scripture.")).tags([OFFERTORY]),

            Document::from(Content::DocumentLink {
                label: "Offertory Sentences".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::OffertorySentences, Slug::Version(Version::RiteII)]),
                rotate: false,
                link_only: false
            }).version(Version::RiteII).tags([OFFERTORY]),

            Document::from(Rubric::from("During the Offertory, a hymn, psalm, or anthem may be sung.")).tags([OFFERTORY_HYMN]),
            Document::from(Content::HymnLink(HymnLink::Hymnals)).tags([OFFERTORY_HYMN]),
            Document::from(Rubric::from("Representatives of the congregation bring the people’s offerings of bread and wine, and money or other gifts, to the deacon or celebrant. \n\nThe people stand while the offerings are presented and placed on the Altar.")).tags([OFFERTORY_RUBRIC_2]),

            Document::from(Heading::from((HeadingLevel::Heading3, "The Great Thanksgiving"))).tags([GREAT_THANKSGIVING_HEADER]),
            Document::from(Content::DocumentLink {
                label: "Eucharistic Prayers".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::GreatThanksgiving]),
                rotate: false,
                link_only: false
            }).tags([EUCHARISTIC_PRAYERS]).display(Show::TemplateOnly),
            Document::from(Choice::from(vec![
                PRAYER_A.clone(),
                PRAYER_B.clone(),
                PRAYER_C.clone(),
                PRAYER_D.clone()
            ])).tags([EUCHARISTIC_PRAYERS]),

            Document::from(Choice::from(vec![
                Document::new().version_label("Traditional")
                    .content(Series::from(vec![
                        Document::from(Text::from("And now, as our Savior\nChrist has taught us,\nwe are bold to say,")),
                        Document::from(Rubric::from("People and Celebrant")),
                        Document::from(Text::from("Our Father, who art in heaven,\n\thallowed be thy Name,\n\tthy kingdom come,\n\tthy will be done,\n\t\ton earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n\tas we forgive those\n\t\twho trespass against us.\nAnd lead us not into temptation,\n\tbut deliver us from evil.\nFor thine is the kingdom,\n\tand the power, and the glory,\n\tfor ever and ever. Amen.").display_format(DisplayFormat::Unison))
                ])),
                Document::new().version_label("Contemporary")
                    .content(Series::from(vec![
                        Document::from(Text::from("As our Savior Christ\nhas taught us,\nwe now pray,")),
                        Document::from(Rubric::from("People and Celebrant")),
                        Document::from(Text::from("Our Father in heaven,\n\thallowed be your Name,\n\tyour kingdom come,\n\tyour will be done,\n\t\ton earth as in heaven.\nGive us today our daily bread.\nForgive us our sins\n\tas we forgive those\n\t\twho sin against us.\nSave us from the time of trial,\n\tand deliver us from evil.\nFor the kingdom, the power,\n\tand the glory are yours,\n\tnow and for ever. Amen.").display_format(DisplayFormat::Unison))
                ]))
            ])).tags([LORDS_PRAYER_TAG]),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Breaking of the Bread"))).tags([FRACTION]),
            Document::from(Rubric::from("The Celebrant breaks the consecrated Bread.\n\nA period of silence is kept.\n\nThen may be sung or said")).tags([FRACTION]),
            Document::from(ResponsivePrayer::from([
                "[Alleluia.] Christ our Passover is sacrificed for us;",
                "Therefore let us keep the feast. [Alleluia.]"
            ]))
                .tags([FRACTION_ANTHEM])
                .condition(Condition::None(vec![
                    Condition::Season(Season::Lent),
                    Condition::Season(Season::HolyWeek),
                    Condition::Season(Season::Easter),
                    Condition::Season(Season::Ascension),
                    Condition::Season(Season::Pentecost),
                ])),
            Document::from(ResponsivePrayer::from([
                "Alleluia. Christ our Passover is sacrificed for us;",
                "Therefore let us keep the feast. Alleluia."
            ]))
                .display(Show::CompiledOnly)
                .tags([FRACTION_ANTHEM])
                .condition(Condition::Any(vec![
                    Condition::Season(Season::Easter),
                    Condition::Season(Season::Ascension),
                    Condition::Season(Season::Pentecost),
                ])),
            Document::from(ResponsivePrayer::from([
                "Christ our Passover is sacrificed for us;",
                "Therefore let us keep the feast."
            ]))
                .display(Show::CompiledOnly)
                .tags([FRACTION_ANTHEM])
                .condition(Condition::Any(vec![
                    Condition::Season(Season::Lent),
                    Condition::Season(Season::HolyWeek),
                ])),
            Document::from(Rubric::from("In Lent, Alleluia is omitted, and may be omitted at other times except during Easter Season.")).display(Show::TemplateOnly).tags([FRACTION_ANTHEM_RUBRIC]),
            Document::from(Rubric::from("In place of, or in addition to, the preceding, some other suitable anthem may be used.")).tags([FRACTION_ANTHEM_RUBRIC]),
            Document::from(HymnLink::TagWithLabel("Fraction".into(), "Fraction Anthems".into())).tags([FRACTION_ANTHEM_RUBRIC]),
            Document::from(Rubric::from("Facing the people, the Celebrant says the following Invitation")).tags([COMMUNION_INVITATION]),
            Document::from(Text::from("The Gifts of God for the People of God.")).tags([COMMUNION_INVITATION]),
            Document::from(Rubric::from("and may add")).tags([COMMUNION_INVITATION]),
            Document::from(Text::from("Take them in remembrance that Christ died for you, and feed on him in your hearts by faith, with thanksgiving.")).tags([COMMUNION_INVITATION]),
            Document::from(Rubric::from("The ministers receive the Sacrament in both kinds, and then immediately deliver it to the people.")).tags([DISTRIBUTION_RUBRIC]),
            Document::from(Rubric::from("The Bread and the Cup are given to the communicants with these words")).tags([DISTRIBUTION_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Text::from("The Body (Blood) of our Lord Jesus Christ keep you in everlasting life.").response("[Amen.]")),
                Document::from(Series::from(vec![
                    Document::from(Text::from("The Body of Christ, the bread of heaven.").response("[Amen.]")),
                    Document::from(Text::from("The Blood of Christ, the cup of salvation.").response("[Amen.]"))
                ]))
            ])).tags([DISTRIBUTION_RUBRIC]),
            Document::from(Rubric::from("During the ministration of Communion, hymns, psalms, or anthems may be sung.")).tags([COMMUNION_HYMN]),
            Document::from(Content::HymnLink(HymnLink::Hymnals)).tags([COMMUNION_HYMN]),
            Document::from(Rubric::from("When necessary, the Celebrant consecrates additional bread and wine, using the form on page 408.")).tags([ADDITIONAL_CONSECRATION_TAG]),
            Document::from(Content::DocumentLink {
                label: "Form for Consecrating Additional Bread and Wine".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ConsecratingAdditional]),
                rotate: false,
                link_only: false
            }).tags([ADDITIONAL_CONSECRATION_TAG]),
            Document::from(Rubric::from("After Communion, the Celebrant says")).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Text::from("Let us pray.")).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Rubric::from("Celebrant and People")).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Choice::from(vec![
                Document::new().page(365)
                    .content(Text::from("Eternal God, heavenly Father,\nyou have graciously accepted us as living members\nof your Son our Savior Jesus Christ,\nand you have fed us with spiritual food\nin the Sacrament of his Body and Blood.\nSend us now into the world in peace,\nand grant us strength and courage\nto love and serve you\nwith gladness and singleness of heart;\nthrough Christ our Lord. Amen.").display_format(DisplayFormat::Unison)),
                Document::new().page(366)
                    .content(Text::from("Almighty and everliving God,\nwe thank you for feeding us with the spiritual food\nof the most precious Body and Blood\nof your Son our Savior Jesus Christ;\nand for assuring us in these holy mysteries\nthat we are living members of the Body of your Son,\nand heirs of your eternal kingdom.\nAnd now, Father, send us out\nto do the work you have given us to do,\nto love and serve you\nas faithful witnesses of Christ our Lord.\nTo him, to you, and to the Holy Spirit,\nbe honor and glory, now and for ever. Amen.").display_format(DisplayFormat::Unison))
            ])).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Rubric::from("The Bishop when present, or the Priest, may bless the people.")).tags([BLESSING]),
            Document::from(Rubric::from("The Deacon, or the Celebrant, dismisses them with these words")).tags([DISMISSAL_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("", "Let us go forth in the name of Christ."),
                    ("People", "Thanks be to God.")
                ])),
                Document::from(Preces::from([
                    ("Deacon", "Go in peace to love and serve the Lord."),
                    ("People", "Thanks be to God.")
                ])),
                Document::from(Preces::from([
                    ("Deacon", "Let us go forth into the world, rejoicing in the power of the Spirit."),
                    ("People", "Thanks be to God.")
                ])),
                Document::from(Preces::from([
                    ("Deacon", "Let us bless the Lord."),
                    ("People", "Thanks be to God.")
                ]))
            ])).condition(Condition::None(vec![
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).tags([DISMISSAL]),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("", "Let us go forth in the name of Christ. Alleluia, alleluia."),
                    ("People", "Thanks be to God. Alleluia, alleluia.")
                ])),
                Document::from(Preces::from([
                    ("Deacon", "Go in peace to love and serve the Lord. Alleluia, alleluia."),
                    ("People", "Thanks be to God. Alleluia, alleluia.")
                ])),
                Document::from(Preces::from([
                    ("Deacon", "Let us go forth into the world, rejoicing in the power of the Spirit. Alleluia, alleluia."),
                    ("People", "Thanks be to God. Alleluia, alleluia.")
                ])),
                Document::from(Preces::from([
                    ("Deacon", "Let us bless the Lord. Alleluia, alleluia."),
                    ("People", "Thanks be to God. Alleluia, alleluia.")
                ]))
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).display(Show::CompiledOnly).tags([DISMISSAL]),
            Document::new().display(Show::TemplateOnly)
                .content(Rubric::from("From the Easter Vigil through the Day of Pentecost “Alleluia, alleluia” may be added to any of the dismissals."))
                .tags([DISMISSAL_RUBRIC_2]),
            Document::new().display(Show::TemplateOnly)
                .content(Preces::from([
                ("", ""),
                ("The People respond", "Thanks be to God. Alleluia, alleluia.")
            ])).tags([DISMISSAL_EASTER])
        ]).preferences([
            // Translations
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::BibleVersion),
                "Bible Version",
                [
                    LiturgyPreferenceOption::from(Version::NRSV),
                    LiturgyPreferenceOption::from(Version::CEB),
                    LiturgyPreferenceOption::from(Version::ESV),
                    LiturgyPreferenceOption::from(Version::KJV)
                ]
            )).category("Translations"),

            // Lectionary
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::Lectionary),
                "Lectionary",
                [
                    LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
                    LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
                ]
            )).category("Lectionary"),

            // Readings
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::ReadingA),
                "First Lesson",
                [
                    LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
                    LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
                ]
            )).default_value(PreferenceValue::from(ReadingType::FirstReading)).category("Lessons"),
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::ReadingB),
                "Second Lesson",
                [
                    LiturgyPreferenceOption::from(("—", PreferenceValue::from(ReadingType::Empty))),
                    LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
                ]
            )).default_value(PreferenceValue::from(ReadingType::SecondReading)).category("Lessons"),
        ]));

    pub static ref OFFERTORY_SENTENCES_II: Vec<Document> = vec![
        Document::from(Sentence::from("Offer to God a sacrifice of thanksgiving, and make good your vows to the Most High.").citation("Psalm 50:14")),
        Document::from(Sentence::from("Ascribe to the Lord the honor due his Name; bring offerings and come into his courts.").citation("Psalm 96:8")),
        Document::from(Sentence::from("Walk in love, as Christ loved us and gave himself for us, an offering and sacrifice to God.").citation("Ephesians 5:2")),
        Document::from(Sentence::from("I appeal to you, brethren, by the mercies of God, to present yourselves as a living sacrifice, holy and acceptable to God, which is your spiritual worship.").citation("Romans 12:1")),
        Document::from(Sentence::from("If you are offering your gift at the altar, and there remember that your brother has something against you, leave your gift there before the altar and go; first be reconciled to your brother, and then come and offer your gift.").citation("Matthew 5:23, 24")),
        Document::from(Sentence::from("Through Christ let us continually offer to God the sacrifice of praise, that is, the fruit of lips that acknowledge his Name. But do not neglect to do good and to share what you have, for such sacrifices are pleasing to God.").citation("Hebrews 13:15, 16")),
        Document::from(Sentence::from("O Lord our God, you are worthy to receive glory and honor and power; because you have created all things, and by your will they were created and have their being.").citation("Revelation 4:11")),
        Document::from(Sentence::from("Yours, O Lord, is the greatness, the power, the glory, the victory, and the majesty. For everything in heaven and on earth is yours. Yours, O Lord, is the kingdom, and you are exalted as head over all.").citation("1 Chronicles 29:11")),
        Document::from("Let us with gladness present the offerings and oblations of our life and labor to the Lord.")
    ];

    pub static ref PENITENTIAL_SENTENCES_II: Vec<Document> = vec![
        Document::from(Sentence::from("Jesus said, “The first commandment is this: Hear, O Israel: The Lord our God is the only Lord. Love the Lord your God with all your heart, with all your soul, with all your mind, and with all your strength. The second is this: Love your neighbor as yourself. There is no other commandment greater than these.”").citation("Mark 12:29-31")),
        Document::from(Sentence::from("If we say that we have no sin, we deceive ourselves, and the truth is not in us. But if we confess our sins, God, who is faithful and just, will forgive our sins and cleanse us from all unrighteousness.").citation("1 John 1:8, 9")),
        Document::from(Sentence::from("Since we have a great high priest who has passed through the heavens, Jesus, the Son of God, let us with confidence draw near to the throne of grace, that we may receive mercy and find grace to help in time of need.").citation("Hebrews 4:14, 16")),
    ];

    pub static ref PRAYER_A: Document = Document::new()
        .label("Eucharistic Prayer A")
        .version_label("Prayer A")
        .page(361)
        .version(Version::RiteII)
        .content(Series::from(vec![
            Document::from(Rubric::from("The people remain standing. The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Lift up your hearts."),
                ("People", "We lift them to the Lord."),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("People", "It is right to give him thanks and praise.")
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(Text::from("It is right, and a good and joyful thing, always and everywhere to give thanks to you, Father Almighty, Creator of heaven and earth.")),
            Document::from(Rubric::from("Here a Proper Preface is sung or said on all Sundays, and on other occasions as appointed.")),
            Document::from(Content::DocumentLink {
                label: "Proper Prefaces".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Version(Version::RiteII)]),
                rotate: true,
                link_only: false
            }),
            Document::from(Text::from("Therefore we praise you, joining our voices with Angels and Archangels and with all the company of heaven, who for ever sing this hymn to proclaim the glory of your Name:")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is he who comes in the name of the Lord.\n\tHosanna in the highest. ").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The people stand or kneel.\n\nThen the Celebrant continues")),
            Document::from(Text::from("Holy and gracious Father: In your infinite love you made us for yourself; and, when we had fallen into sin and become subject to evil and death, you, in your mercy, sent Jesus Christ, your only and eternal Son, to share our human nature, to live and die as one of us, to reconcile us to you, the God and Father of all.\n\nHe stretched out his arms upon the cross, and offered himself in obedience to your will, a perfect sacrifice for the whole world.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated.")),
            Document::from(Text::from("On the night he was handed over to suffering and death, our Lord Jesus Christ took bread; and when he had given thanks to you, he broke it, and gave it to his disciples, and said, “Take, eat: This is my Body, which is given for you. Do this for the remembrance of me.”\n\nAfter supper he took the cup of wine; and when he had given thanks, he gave it to them, and said, “Drink this, all of you: This is my Blood of the new Covenant, which is shed for you and for many for the forgiveness of sins. Whenever you drink it, do this for the remembrance of me.”\n\nTherefore we proclaim the mystery of faith:")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Christ has died.\nChrist is risen.\nChrist will come again.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("We celebrate the memorial of our redemption, O Father, in this sacrifice of praise and thanksgiving. Recalling his death, resurrection, and ascension, we offer you these gifts.\n\nSanctify them by your Holy Spirit to be for your people the Body and Blood of your Son, the holy food and drink of new and unending life in him. Sanctify us also that we may faithfully receive this holy Sacrament, and serve you in unity, constancy, and peace; and at the last day bring us with all your saints into the joy of your eternal kingdom.\n\nAll this we ask through your Son Jesus Christ. By him, and with him, and in him, in the unity of the Holy Spirit all honor and glory is yours, Almighty Father, now and for ever.").response("AMEN."))
    ]));

    pub static ref PRAYER_B: Document = Document::new()	.label("Eucharistic Prayer B")
        .version_label("Prayer B")
        .page(367)
        .version(Version::RiteII)
        .content(Series::from(vec![
            Document::from(Rubric::from("The people remain standing. The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Lift up your hearts."),
                ("People", "We lift them to the Lord."),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("People", "It is right to give him thanks and praise.")
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(Text::from("It is right, and a good and joyful thing, always and everywhere to give thanks to you, Father Almighty, Creator of heaven and earth.")),
            Document::from(Rubric::from("Here a Proper Preface is sung or said on all Sundays, and on other occasions as appointed.")),
            Document::from(Content::DocumentLink {
                label: "Proper Prefaces".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Version(Version::RiteII)]),
                rotate: true,
                link_only: false
            }),
            Document::from(Text::from("Therefore we praise you, joining our voices with Angels and Archangels and with all the company of heaven, who for ever sing this hymn to proclaim the glory of your Name:")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is he who comes in the name of the Lord.\n\tHosanna in the highest. ").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The people stand or kneel.\n\nThen the Celebrant continues")),
            Document::from(Text::from("We give thanks to you, O God, for the goodness and love which you have made known to us in creation; in the calling of Israel to be your people; in your Word spoken through the prophets; and above all in the Word made flesh, Jesus, your Son. For in these last days you sent him to be incarnate from the Virgin Mary, to be the Savior and Redeemer of the world. In him, you have delivered us from evil, and made us worthy to stand before you. In him, you have brought us out of error into truth, out of sin into righteousness, out of death into life.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated.")),
            Document::from(Text::from("On the night before he died for us, our Lord Jesus Christ took bread; and when he had given thanks to you, he broke it, and gave it to his disciples, and said, “Take, eat: This is my Body, which is given for you. Do this for the remembrance of me.”\n\nAfter supper he took the cup of wine; and when he had given thanks, he gave it to them, and said, “Drink this, all of you: This is my Blood of the new Covenant, which is shed for you and for many for the forgiveness of sins. Whenever you drink it, do this for the remembrance of me.”\n\nTherefore, according to his command, O Father,")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("We remember his death,\nWe proclaim his resurrection,\nWe await his coming in glory.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("And we offer our sacrifice of praise and thanksgiving to you, O Lord of all; presenting to you, from your creation, this bread and this wine.\n\nWe pray you, gracious God, to send your Holy Spirit upon these gifts that they may be the Sacrament of the Body of Christ and his Blood of the new Covenant. Unite us to your Son in his sacrifice, that we may be acceptable through him, being sanctified by the Holy Spirit. In the fullness of time, put all things in subjection under your Christ, and bring us to that heavenly country where, with [ ___________ and] all your saints, we may enter the everlasting heritage of your sons and daughters; through Jesus Christ our Lord, the firstborn of all creation, the head of the Church, and the author of our salvation.\n\nBy him, and with him, and in him, in the unity of the Holy Spirit all honor and glory is yours, Almighty Father, now and for ever.").response("AMEN."))
    ]));

    pub static ref PRAYER_C: Document = Document::new()
        .version(Version::RiteII)
        .label("Eucharistic Prayer C")
        .version_label("Prayer C")
        .page(369)
        .content(Series::from(vec![
            Document::from(Rubric::from("In this prayer, the lines in italics are spoken by the People.")),
            Document::from(Rubric::from("The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(ResponsivePrayer::from([
                "The Lord be with you.",
                "And also with you.\n",
                "Lift up your hearts.",
                "We lift them to the Lord.\n",
                "Let us give thanks to the Lord our God.",
                "It is right to give him thanks and praise.\n"
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(ResponsivePrayer::from([
                "God of all power, Ruler of the Universe, you are worthy of glory and praise.",
                "Glory to you for ever and ever.\n",
                "At your command all things came to be: the vast expanse of interstellar space, galaxies, suns, the planets in their courses, and this fragile earth, our island home.",
                "By your will they were created and have their being.\n",
                "From the primal elements you brought forth the human race, and blessed us with memory, reason, and skill. You made us the rulers of creation. But we turned against you, and betrayed your trust; and we turned against one another.",
                "Have mercy, Lord, for we are sinners in your sight.\n",
                "Again and again, you called us to return. Through prophets and sages you revealed your righteous Law. And in the fullness of time you sent your only Son, born of a woman, to fulfill your Law, to open for us the way of freedom and peace.",
                "By his blood, he reconciled us.\nBy his wounds, we are healed.\n",
                "And therefore we praise you, joining with the heavenly chorus, with prophets, apostles, and martyrs, and with all those in every generation who have looked to you in hope, to proclaim with them your glory, in their unending hymn:"
            ])),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is he who comes in the name of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("And so, Father, we who have been redeemed by him, and\nmade a new people by water and the Spirit, now bring before\nyou these gifts. Sanctify them by your Holy Spirit to be the\nBody and Blood of Jesus Christ our Lord.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it,\nor lay a hand upon it; and at the words concerning the cup, to hold or\nplace a hand upon the cup and any other vessel containing wine to be\nconsecrated.")),
            Document::from(Text::from("On the night he was betrayed he took bread, said the\nblessing, broke the bread, and gave it to his friends, and\nsaid, “Take, eat: This is my Body, which is given for you. Do\nthis for the remembrance of me.”\n\nAfter supper, he took the cup of wine, gave thanks, and\nsaid, “Drink this, all of you: This is my Blood of the new\nCovenant, which is shed for you and for many for the\nforgiveness of sins. Whenever you drink it, do this for the\nremembrance of me.”")),
            Document::from(ResponsivePrayer::from([
                "Remembering now his work of redemption, and offering to\nyou this sacrifice of thank",
                "We celebrate his death and resurrection,\nas we await the day of his coming."
            ])),
            Document::from(Text::from("Lord God of our Fathers; God of Abraham, Isaac, and\nJacob; God and Father of our Lord Jesus Christ: Open our\neyes to see your hand at work in the world about us. Deliver\nus from the presumption of coming to this Table for solace\nonly, and not for strength; for pardon only, and not for\nrenewal. Let the grace of this Holy Communion make us one\nbody, one spirit in Christ, that we may worthily serve the\nworld in his name.\n\nRisen Lord, be known to us in the breaking of the Bread.\nAccept these prayers and praises, Father, through Jesus\nChrist our great High Priest, to whom, with you and the\nHoly Spirit, your Church gives honor, glory, and worship,\nfrom generation to generation.").response("AMEN."))
    ]));
    pub static ref PRAYER_D: Document = Document::new()
        .label("Eucharistic Prayer D")
        .version_label("Prayer D")
        .page(372)
        .version(Version::RiteII)
        .content(Series::from(vec![
            Document::from(Rubric::from("The people remain standing. The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Lift up your hearts."),
                ("People", "We lift them to the Lord."),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("People", "It is right to give him thanks and praise.")
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(Text::from("It is truly right to glorify you, Father, and to give you thanks; for you alone are God, living and true, dwelling in light inaccessible from before time and for ever.\n\nFountain of life and source of all goodness, you made all things and fill them with your blessing; you created them to rejoice in the splendor of your radiance.\n\nCountless throngs of angels stand before you to serve you night and day; and, beholding the glory of your presence, they offer you unceasing praise. Joining with them, and giving voice to every creature under heaven, we acclaim you, and glorify your Name, as we sing (say),")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is he who comes in the name of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The people stand or kneel.\n\nThen the Celebrant continues")),
            Document::from(Text::from("We acclaim you, holy Lord, glorious in power. Your mighty works reveal your wisdom and love. You formed us in your own image, giving the whole world into our care, so that, in obedience to you, our Creator, we might rule and serve all your creatures. When our disobedience took us far from you, you did not abandon us to the power of death. In your mercy you came to our help, so that in seeking you we might find you. Again and again you called us into covenant with you, and through the prophets you taught us to hope for salvation.\n\nFather, you loved the world so much that in the fullness of time you sent your only Son to be our Savior. Incarnate by the Holy Spirit, born of the Virgin Mary, he lived as one of us, yet without sin. To the poor he proclaimed the good news of salvation; to prisoners, freedom; to the sorrowful, joy. To fulfill your purpose he gave himself up to death; and, rising from the grave, destroyed death, and made the whole creation new.\n\nAnd, that we might live no longer for ourselves, but for him who died and rose for us, he sent the Holy Spirit, his own first gift for those who believe, to complete his work in the world, and to bring to fulfillment the sanctification of all.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated.")),
            Document::from(Text::from("When the hour had come for him to be glorified by you, his heavenly Father, having loved his own who were in the world, he loved them to the end; at supper with them he took bread, and when he had given thanks to you, he broke it, and gave it to his disciples, and said, “Take, eat: This is my Body, which is given for you. Do this for the remembrance of me.”\n\nAfter supper he took the cup of wine; and when he had given thanks, he gave it to them, and said, “Drink this, all of you. This is my Blood of the new Covenant, which is shed for you and for many for the forgiveness of sins. Whenever you drink it, do this for the remembrance of me.”\n\nFather, we now celebrate this memorial of our redemption. Recalling Christ’s death and his descent among the dead, proclaiming his resurrection and ascension to your right hand, awaiting his coming in glory; and offering to you, from the gifts you have given us, this bread and this cup, we praise you and we bless you.")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("We praise you, we bless you,\nwe give thanks to you,\nand we pray to you, Lord our God.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("Lord, we pray that in your goodness and mercy your Holy Spirit may descend upon us, and upon these gifts, sanctifying them and showing them to be holy gifts for your holy people, the bread of life and the cup of salvation, the Body and Blood of your Son Jesus Christ.\n\nGrant that all who share this bread and cup may become one body and one spirit, a living sacrifice in Christ, to the praise of your Name.\n\nRemember, Lord, your one holy catholic and apostolic Church, redeemed by the blood of your Christ. Reveal its unity, guard its faith, and preserve it in peace.\n\n[Remember (*NN.* and) all who minister in your Church.]\n[Remember all your people, and those who seek your truth.]\n[Remember ___________.]\n[Remember all who have died in the peace of Christ, and\nthose whose faith is known to you alone; bring them into\nthe place of eternal joy and light.]\n\nAnd grant that we may find our inheritance with [the Blessed Virgin Mary, with patriarchs, prophets, apostles, and martyrs, (with _________) and] all the saints who have found favor with you in ages past. We praise you in union with them and give you glory through your Son Jesus Christ our Lord.\n\nThrough Christ, and with Christ, and in Christ, all honor and glory are yours, Almighty God and Father, in the unity of the Holy Spirit, for ever and ever.").response("AMEN."))
    ]));
}
