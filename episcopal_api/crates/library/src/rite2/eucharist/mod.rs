use calendar::Season;
use lazy_static::lazy_static;
use lectionary::ReadingType;
use liturgy::*;

lazy_static! {
    pub static ref GLORIA_IN_EXCELSIS: Text = Text::from("Glory to God in the highest,\n\tand peace to his people on earth. \n\nLord God, heavenly King,\nalmighty God and Father,\n\twe worship you, we give you thanks,\n\twe praise you for your glory. \n\nLord Jesus Christ, only Son of the Father,\nLord God, Lamb of God,\nyou take away the sin of the world:\n\thave mercy on us;\nyou are seated at the right hand of the Father:\n\treceive our prayer. \n\nFor you alone are the Holy One,\nyou alone are the Lord,\nyou alone are the Most High,\n\tJesus Christ,\n\twith the Holy Spirit,\n\tin the glory of God the Father. Amen.");

    pub static ref KYRIE_ELEISON: Choice = Choice::from(vec![
        Document::from(ResponsivePrayer::from([
            "Lord, have mercy.",
            "Christ, have mercy.",
            "Lord, have mercy."
        ])),
        Document::from(ResponsivePrayer::from([
            "Kyrie eleison.",
            "Christe eleison.",
            "Kyrie eleison."
        ]))
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
            Document::new()
                .display(Show::TemplateOnly)
                .content(Rubric::from("In place of the above, from Easter Day through the Day of Pentecost")),
            Document::from(Preces::from([
                ("Celebrant", "Alleluia. Christ is risen."),
                ("People", "The Lord is risen indeed. Alleluia.")
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])),
            Document::new().display(Show::TemplateOnly)
                .content(Rubric::from("In Lent and on other penitential occasions")),
            Document::from(Preces::from([
                ("Celebrant", "Bless the Lord who forgives all our sins."),
                ("People", "His mercy endures for ever.")
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Lent),
                Condition::Season(Season::HolyWeek)
            ])),
            Document::from(Choice::from(vec![
                        Document::new().version_label("Gloria")
                            .content(Text::from("Glory to God in the highest,\n\tand peace to his people on earth.\n\nLord God, heavenly King,\nalmighty God and Father,\n\twe worship you, we give you thanks,\n\twe praise you for your glory.\n\nLord Jesus Christ, only Son of the Father,\nLord God, Lamb of God,\nyou take away the sin of the world:\n\thave mercy on us;\nyou are seated at the right hand of the Father:\n\treceive our prayer.\n\nFor you alone are the Holy One,\nyou alone are the Lord,\nyou alone are the Most High,\n\tJesus Christ,\n\twith the Holy Spirit,\n\tin the glory of God the Father. Amen.")),
                        Document::new().version_label("Kyrie")
                            .content(ResponsivePrayer::from([
                            "Lord, have mercy.",
                            "Christ, have mercy.",
                            "Lord, have mercy."
                        ])),
                        Document::new().version_label("Kyrie (Greek)")
                            .content(ResponsivePrayer::from([
                            "Kyrie eleison.",
                            "Christe eleison.",
                            "Kyrie eleison."
                        ])),
                        Document::new().version_label("Trisagion")
                            .content(ResponsivePrayer::from([
                            "Holy God,\nHoly and Mighty,\nHoly Immortal One,",
                            "Have mercy upon us."
                        ]))
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Collect of the Day"))),
            Document::from(Rubric::from("The Celebrant says to the people")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Let us pray.")
            ])),
            Document::from(Rubric::from("The Celebrant says the Collect.")),
            Document::new().version(Version::RiteII)
                .content(Content::CollectOfTheDay { allow_multiple: false }),
            Document::new().display(Show::TemplateOnly)
                .content(Preces::from([
                ("", ""),
                ("People", "Amen.")
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Lessons"))),
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
                    Document::from(Rubric::from("Silence may follow.\n\nA Psalm, hymn, or anthem may follow each Reading.")),
                    Document::from(Content::HymnLink(HymnLink::Hymnals)),
                    Document::from(Rubric::from("Then, all standing, the Deacon or a Priest reads the Gospel, first saying")),
                    Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to ____________."),
                        ("People", "Glory to you, Lord Christ.")
                    ]))
            ])),
            Document::new().display(Show::CompiledOnly)
                .content(Series::from(vec![
                            Document::from(Rubric::from("The people sit.")),
                            Document::new().label("The First Lesson")
                                .content(LectionaryReading {
                                reading_type: ReadingTypeTable::Preference(PreferenceKey::Global(GlobalPref::ReadingA)),
                                reading_type_overridden_by: None,
                                lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                                intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Text::from("A Reading from {{long_name}}."))))
                            }),
                            Document::from(LectionaryReading {
                                reading_type: ReadingTypeTable::Selected(ReadingType::Psalm),
                                reading_type_overridden_by: None,
                                lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                                intro: None
                            }),
                            Document::new().label("The Second Lesson")
                                .content(LectionaryReading {
                                reading_type: ReadingTypeTable::Preference(PreferenceKey::Global(GlobalPref::ReadingB)),
                                reading_type_overridden_by: None,
                                lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                                intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Text::from("A Reading from {{long_name}}."))))
                            }),
                            Document::new().label("The Gospel")
                                .content(LectionaryReading {
                                reading_type: ReadingTypeTable::Selected(ReadingType::Gospel),
                                reading_type_overridden_by: None,
                                lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                                intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Preces::from([
                                ("", "The Holy Gospel of our Lord Jesus Christ according to {{short_name}}."),
                                ("People", "Glory to you, Lord Christ.")
                            ]))))
                            })
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon"))),
            Document::from(Rubric::from("On Sundays and other Major Feasts there follows, all standing")),
            Document::from(Text::from("We believe in one God,\n\tthe Father, the Almighty,\n\tmaker of heaven and earth,\n\tof all that is, seen and unseen.\n\nWe believe in one Lord, Jesus Christ,\n\tthe only Son of God,\n\teternally begotten of the Father,\n\tGod from God, Light from Light,\n\ttrue God from true God,\n\tbegotten, not made,\n\tof one Being with the Father.\n\tThrough him all things were made.\n\tFor us and for our salvation\n\t\the came down from heaven:\n\tby the power of the Holy Spirit\n\t\the became incarnate from the Virgin Mary,\n\t\tand was made man.\n\tFor our sake he was crucified under Pontius Pilate;\n\t\the suffered death and was buried.\n\t\tOn the third day he rose again\n\t\t\tin accordance with the Scriptures;\n\t\the ascended into heaven\n\t\t\tand is seated at the right hand of the Father.\n\tHe will come again in glory to judge the living and the dead,\n\t\tand his kingdom will have no end.\n\nWe believe in the Holy Spirit, the Lord, the giver of life,\n\twho proceeds from the Father and the Son.\n\tWith the Father and the Son he is worshiped and glorified.\n\tHe has spoken through the Prophets.\n\tWe believe in one holy catholic and apostolic Church.\n\tWe acknowledge one baptism for the forgiveness of sins.\n\tWe look for the resurrection of the dead,\n\t\tand the life of the world to come. Amen.\n")),

            Document::from(Heading::from((HeadingLevel::Heading3, "The Prayers of the People"))),
            Document::from(Rubric::from("Prayer is offered with intercession for\nThe Universal Church, its members, and its mission\nThe Nation and all in authority\nThe welfare of the world\nThe concerns of the local community\nThose who suffer and those in any trouble\nThe departed (with commemoration of a saint when appropriate)\nSee the forms beginning on page 383.")),
            Document::from(Content::DocumentLink {
                label: "Prayers of the People".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::PrayersOfThePeople]),
                rotate: false
            }),

            Document::from(Rubric::from("If there is no celebration of the Communion, or if a priest is not available, the service is concluded as directed on page 406.")),
            Document::from(Heading::from((HeadingLevel::Heading3, "Confession of Sin"))),
            Document::from(Rubric::from("A Confession of Sin is said here if it has not been said earlier. On occasion, the Confession may be omitted.")),
            Document::from(Rubric::from("One of the sentences from the Penitential Order on page 351 may be said.")),
            Document::from(Rubric::from("The Deacon or Celebrant says")),
            Document::from(Text::from("Let us confess our sins against God and our neighbor.")),
            Document::from(Rubric::from("Silence may be kept.")),
            Document::from(Rubric::from("Minister and People")),
            Document::from(Text::from("Most merciful God,\nwe confess that we have sinned against you\nin thought, word, and deed,\nby what we have done,\nand by what we have left undone.\nWe have not loved you with our whole heart;\nwe have not loved our neighbors as ourselves.\nWe are truly sorry and we humbly repent.\nFor the sake of your Son Jesus Christ,\nhave mercy on us and forgive us;\nthat we may delight in your will,\nand walk in your ways,\nto the glory of your Name. Amen.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Bishop when present, or the Priest, stands and says")),
            Document::from(Text::from("Almighty God have mercy on you, forgive you all your sins through our Lord Jesus Christ, strengthen you in all goodness, and by the power of the Holy Spirit keep you in eternal life.").response("Amen.")),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Peace"))),
            Document::from(Rubric::from("All stand. The Celebrant says to the people")),
            Document::from(Preces::from([
                ("", "The peace of the Lord be always with you."),
                ("People", "And also with you.")
            ])),
            Document::from(Rubric::from("Then the Ministers and People may greet one another in the name of the Lord.")),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Holy Communion"))),
            Document::from(Rubric::from("The Celebrant may begin the Offertory with one of the sentences on page 376, or with some other sentence of Scripture.")),

            Document::from(Content::DocumentLink {
                label: "Offertory Sentences".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::OffertorySentences, Slug::Version(Version::RiteII)]),
                rotate: false
            }).version(Version::RiteII),

            Document::from(Rubric::from("During the Offertory, a hymn, psalm, or anthem may be sung.")),
            Document::from(Content::HymnLink(HymnLink::Hymnals)),

            // TODO add Eucharistic Prayers

            Document::from(Rubric::from("Representatives of the congregation bring the people’s offerings of bread and wine, and money or other gifts, to the deacon or celebrant. \n\nThe people stand while the offerings are presented and placed on the Altar.")),
            Document::from(Choice::from(vec![
                        Document::new().version_label("Traditional")
                            .content(Series::from(vec![
                                            Document::from(Text::from("")),
                                            Document::from(Text::from("And now, as our Savior\nChrist has taught us,\nwe are bold to say,")),
                                            Document::from(Text::from("Our Father, who art in heaven,\n\thallowed be thy Name,\n\tthy kingdom come,\n\tthy will be done,\n\t\ton earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n\tas we forgive those\n\t\twho trespass against us.\nAnd lead us not into temptation,\n\tbut deliver us from evil.\nFor thine is the kingdom,\n\tand the power, and the glory,\n\tfor ever and ever. Amen.").display_format(DisplayFormat::Unison))
                        ])),
                        Document::new().version_label("Contemporary")
                            .content(Series::from(vec![
                                            Document::new().version_label("Contemporary")
                                                .content(Text::from("As our Savior Christ\nhas taught us,\nwe now pray,")),
                                            Document::from(Text::from("Our Father in heaven,\n\thallowed be your Name,\n\tyour kingdom come,\n\tyour will be done,\n\t\ton earth as in heaven.\nGive us today our daily bread.\nForgive us our sins\n\tas we forgive those\n\t\twho sin against us.\nSave us from the time of trial,\n\tand deliver us from evil.\nFor the kingdom, the power,\n\tand the glory are yours,\n\tnow and for ever. Amen.").display_format(DisplayFormat::Unison))
                        ]))
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Breaking of the Bread"))),
            Document::from(Rubric::from("The Celebrant breaks the consecrated Bread.\n\nA period of silence is kept.\n\nThen may be sung or said")),
            Document::from(ResponsivePrayer::from([
                "[Alleluia.] Christ our Passover is sacrificed for us;",
                "Therefore let us keep the feast. [Alleluia.]"
            ])),
            Document::from(Rubric::from("In Lent, Alleluia is omitted, and may be omitted at other times except during Easter Season.\n\nIn place of, or in addition to, the preceding, some other suitable anthem may be used.")),
            Document::from(Rubric::from("Facing the people, the Celebrant says the following Invitation")),
            Document::from(Text::from("The Gifts of God for the People of God.")),
            Document::from(Rubric::from("and may add")),
            Document::from(Text::from("Take them in remembrance that Christ died for you, and feed on him in your hearts by faith, with thanksgiving.")),
            Document::from(Rubric::from("The ministers receive the Sacrament in both kinds, and then immediately deliver it to the people.")),
            Document::from(Rubric::from("The Bread and the Cup are given to the communicants with these words")),
            Document::from(Choice::from(vec![
                        Document::from(Text::from("The Body (Blood) of our Lord Jesus Christ keep you in\neverlasting life.").response("[Amen.]")),
                        Document::from(Series::from(vec![
                                            Document::from(Text::from("The Body of Christ, the bread of heaven.").response("[Amen.]")),
                                            Document::from(Text::from("The Blood of Christ, the cup of salvation.").response("[Amen.]"))
                        ]))
            ])),
            Document::from(Rubric::from("During the ministration of Communion, hymns, psalms, or anthems may be sung.")),
            Document::from(Content::HymnLink(HymnLink::Hymnals)),
            Document::from(Rubric::from("When necessary, the Celebrant consecrates additional bread and wine, using the form on page 408.")),
            Document::from(Rubric::from("After Communion, the Celebrant says")),
            Document::from(Text::from("Let us pray.")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Choice::from(vec![
                        Document::new().page(365)
                            .content(Text::from("Eternal God, heavenly Father,\nyou have graciously accepted us as living members\nof your Son our Savior Jesus Christ,\nand you have fed us with spiritual food\nin the Sacrament of his Body and Blood.\nSend us now into the world in peace,\nand grant us strength and courage\nto love and serve you\nwith gladness and singleness of heart;\nthrough Christ our Lord. Amen.").display_format(DisplayFormat::Unison)),
                        Document::new().page(366)
                            .content(Text::from("Almighty and everliving God,\nwe thank you for feeding us with the spiritual food\nof the most precious Body and Blood\nof your Son our Savior Jesus Christ;\nand for assuring us in these holy mysteries\nthat we are living members of the Body of your Son,\nand heirs of your eternal kingdom.\nAnd now, Father, send us out\nto do the work you have given us to do,\nto love and serve you\nas faithful witnesses of Christ our Lord.\nTo him, to you, and to the Holy Spirit,\nbe honor and glory, now and for ever. Amen.").display_format(DisplayFormat::Unison))
            ])),
            Document::from(Rubric::from("The Bishop when present, or the Priest, may bless the people.")),
            Document::from(Rubric::from("The Deacon, or the Celebrant, dismisses them with these words")),
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
            ])),
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
            ])),
            Document::new().display(Show::TemplateOnly)
                .content(Rubric::from("From the Easter Vigil through the Day of Pentecost “Alleluia, alleluia” may be added to any of the dismissals.")),
            Document::new().display(Show::TemplateOnly)
                .content(Preces::from([
                ("", ""),
                ("The People respond", "Thanks be to God. Alleluia, alleluia.")
            ]))
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
}
