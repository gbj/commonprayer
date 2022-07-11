use lazy_static::lazy_static;
use lectionary::ReadingType;
use liturgy::*;

use crate::rite2::{
    EUCHARIST_INTROS_II, LORDS_PRAYER_CONTEMPORARY_TEXT, LORDS_PRAYER_TRADITIONAL_TEXT,
};

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
            Document::from(Content::DocumentLink {
                label: "Additional Directions".into(),
                path: SlugPath::from([Slug::Baptism, Slug::AdditionalDirections]),
                rotate: false,
                link_only: false
            }),
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
            Document::from(Content::DocumentLink {
                label: "Additional Directions".into(),
                path: SlugPath::from([Slug::Baptism, Slug::AdditionalDirections]),
                rotate: false,
                link_only: false
            }),

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
            ])).display(Show::CompiledOnly),

            Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon"))),
            // TODO include as a preference?
            Document::from(Rubric::from("Or the Sermon may be preached after the Peace.")),

            // Presentation
            Document::from(Heading::from((HeadingLevel::Heading2, "Presentation and Examination of the Candidates"))),
            Document::from(Rubric::from("The Celebrant says")),
            Document::from("The Candidate(s) for Holy Baptism will now be presented."),
            Document::from(Heading::from((HeadingLevel::Heading3, "Adults and Older Children"))),
            Document::from(Rubric::from("The candidates who are able to answer for themselves are presented individually by their Sponsors, as follows")),
            Document::from(Preces::from([("Sponsor", "I present *N.* to receive the Sacrament of Baptism.")])),
            Document::from(Rubric::from("The Celebrant asks each candidate when presented")),
            Document::from(Preces::from([
                ("", "Do you desire to be baptized?"),
                ("Candidate", "I do.")
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "Infants and Younger Children"))),
            Document::from(Rubric::from("Then the candidates unable to answer for themselves are presented individually by their Parents and Godparents, as follows")),
            Document::from(Rubric::from("Parents and Godparents")),
            Document::from("I present *N.* to receive the Sacrament of Baptism."),
            Document::from(Rubric::from("When all have been presented the Celebrant asks the parents and godparents")),
            Document::from("Will you be responsible for seeing that the child you present is brought up in the Christian faith and life?"),
            Document::from(Rubric::from("Parents and Godparents")),
            Document::from("I will, with God’s help."),
            Document::from(Rubric::from("Then the Celebrant asks the following questions of the candidates who can speak for themselves, and of the parents and godparents who speak on behalf of the infants and younger children")),
            Document::from(Preces::from([
                ("Question", "Do you renounce Satan and all the spiritual forces of wickedness that rebel against God?"),
                ("Answer", "I renounce them."),
                ("Question", "Do you renounce the evil powers of this world which corrupt and destroy the creatures of God?"),
                ("Answer", "I renounce them."),
                ("Question", "Do you renounce all sinful desires that draw you from the love of God?"),
                ("Answer", "I renounce them."),
                ("Question", "Do you turn to Jesus Christ and accept him as your Savior?"),
                ("Answer", "I do."),
                ("Question", "Do you put your whole trust in his grace and love?"),
                ("Answer", "I do."),
                ("Question", "Do you promise to follow and obey him as your Lord?"),
                ("Answer", "I do."),
            ])),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("When there are others to be presented, the Bishop says")),
                Document::from("The other Candidate(s) will now be presented."),
                Document::from(Choice::from(vec![
                    Document::from(Preces::from([("Presenters", "I present *these persons* for Confirmation.")])),
                    Document::from(Preces::from([("Presenters", "I present *these persons* to be received into this Communion.")])),
                    Document::from(Preces::from([("Presenters", "I present *these persons* who *desire* to reaffirm *their* baptismal vows.")])),
                ])),
                Document::from(Rubric::from("The Bishop asks the candidates")),
                Document::from("Do you reaffirm your renunciation of evil?"),
                Document::from(Preces::from([("Candidate", "I do.")])),
                Document::from(Rubric::from("Bishop")),
                Document::from("Do you renew your commitment to Jesus Christ?"),
                Document::from(Rubric::from("Candidate")),
                Document::from("I do, and with God’s grace I will follow him as my Savior and Lord."),
            ])).optional(),

            Document::from(Rubric::from("After all have been presented, the Celebrant addresses the congregation, saying")),
            Document::from("Will you who witness these vows do all in your power to support these persons in their life in Christ?"),
            Document::from(Preces::from([
                ("", ""),
                ("People", "We will.")
            ])),

            // Baptismal Covenant
            Document::from(Rubric::from("The Celebrant then says these or similar words")),
            Document::from("Let us join with *those* who *are* committing *themselves* to Christ and renew our own baptismal covenant."),

            BAPTISMAL_COVENANT.clone(),

            // Prayers
            Document::from(Heading::from((HeadingLevel::Heading3, "Prayers for the Candidates"))),
            Document::from(Rubric::from("The Celebrant then says to the congregation")),
            Document::from("Let us now pray for *these persons* who *are* to receive the Sacrament of new birth [and for those (this person) who *have* renewed *their* commitment to Christ.]"),
            Document::from(Rubric::from("A Person appointed leads the following petitions")),
            Document::from(Preces::from([
                ("Leader", "Deliver *them*, O Lord, from the way of sin and death."),
                ("People", "Lord, hear our prayer.\n "),
                ("Leader", "Open *their* hearts to your grace and truth."),
                ("People", "Lord, hear our prayer.\n "),
                ("Leader", "Fill *them* with your holy and life-giving Spirit."),
                ("People", "Lord, hear our prayer.\n "),
                ("Leader", "Keep *them* in the faith and communion of your holy Church."),
                ("People", "Lord, hear our prayer.\n "),
                ("Leader", "Teach *them* to love others in the power of the Spirit."),
                ("People", "Lord, hear our prayer.\n "),
                ("Leader", "Send *them* into the world in witness to your love"),
                ("People", "Lord, hear our prayer.\n "),
                ("Leader", "Bring *them* to the fullness of your peace and glory."),
                ("People", "Lord, hear our prayer.\n "),
            ])),
            Document::from(Rubric::from("The Celebrant says")),
            Document::from(Text::from("Grant, O Lord, that all who are baptized into the death of Jesus Christ your Son may live in the power of his resurrection and look for him to come again in glory; who lives and reigns now and for ever.").response("Amen.")),

            // Thanksgiving over the Water
            Document::from(Heading::from((HeadingLevel::Heading3, "Thanksgiving over the Water"))),
            Document::from(Rubric::from("The Celebrant blesses the water, first saying")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you.\n "),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("People", "It is right to give him thanks and praise.")
            ])),
            Document::from(Rubric::from("Celebrant")),
            Document::from("We thank you, Almighty God, for the gift of water. Over it the Holy Spirit moved in the beginning of creation. Through it you led the children of Israel out of their bondage in Egypt into the land of promise. In it your Son Jesus received the baptism of John and was anointed by the Holy Spirit as the Messiah, the Christ, to lead us, through his death and resurrection, from the bondage of sin into everlasting life.\n\nWe thank you, Father, for the water of Baptism. In it we are buried with Christ in his death. By it we share in his resurrection. Through it we are reborn by the Holy Spirit. Therefore in joyful obedience to your Son, we bring into his fellowship those who come to him in faith, baptizing them in the Name of the Father, and of the Son, and of the Holy Spirit."),
            Document::from(Rubric::from("At the following words, the Celebrant touches the water")),
            Document::from(Text::from("Now sanctify this water, we pray you, by the power of your Holy Spirit, that those who here are cleansed from sin and born again may continue for ever in the risen life of Jesus Christ our Savior.\n\nTo him, to you, and to the Holy Spirit, be all honor and glory, now and for ever.").response("Amen.")),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading3, "Consecration of the Chrism"))),
                Document::from(Rubric::from("The Bishop may then consecrate oil of Chrism, placing a hand on the vessel of oil, and saying")),
                Document::from(Text::from("Eternal Father, whose blessed Son was anointed by the Holy Spirit to be the Savior and servant of all, we pray you to consecrate this oil, that those who are sealed with it may share in the royal priesthood of Jesus Christ; who lives and reigns with you and the Holy Spirit, for ever and ever.").response("Amen."))
            ])).optional(),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Baptism"))),
            Document::from(Rubric::from("Each candidate is presented by name to the Celebrant, or to an assisting priest or deacon, who then immerses, or pours water upon, the candidate, saying")),
            Document::from(Text::from("*N.*, I baptize you in the Name of the Father, and of the Son, and of the Holy Spirit.").response("Amen.")),
            Document::from(Rubric::from("When this action has been completed for all candidates, the Bishop or Priest, at a place in full sight of the congregation, prays over them, saying")),
            Document::from("Let us pray."),
            Document::from(Text::from("Heavenly Father, we thank you that by water and the Holy Spirit you have bestowed upon *these* your *servants* the forgiveness of sin, and have raised *them* to the new life of grace. Sustain *them*, O Lord, in your Holy Spirit. Give *them* an inquiring and discerning heart, the courage to will and to persevere, a spirit to know and to love you, and the gift of joy and wonder in all your works.").response("Amen.")),
            Document::from(Rubric::from("Then the Bishop or Priest places a hand on the person’s head, marking on the forehead the sign of the cross [using Chrism if desired] and saying to each one")),
            Document::from(Text::from("*N.*, you are sealed by the Holy Spirit in Baptism and marked as Christ’s own for ever.").response("Amen.")),

            // TODO preference?
            Document::from(Rubric::from("Or this action may be done immediately after the administration of the water and before the preceding prayer.")),
            Document::from(Rubric::from("When all have been baptized, the Celebrant says")),
            Document::from("Let us welcome the newly baptized."),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("We receive you into the household of God. Confess the faith of Christ crucified, proclaim his resurrection, and share with us in his eternal priesthood.").display_format(DisplayFormat::Unison)),

            // TODO preference: is there confirmation/reception/reaffirmation?
            Document::from(Rubric::from("If Confirmation, Reception, or the Reaffirmation of Baptismal Vows is not to follow, the Peace is now exchanged")),
            Document::from(Preces::from([
                ("Celebrant", "The peace of the Lord be always with you."),
                ("People", "And also with you.")
            ])),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading3, "At Confirmation, Reception, or Reaffirmation"))),
                Document::from(Rubric::from("The Bishop says to the congregation")),
                Document::from("Let us now pray for *these persons* who *have* renewed *their* commitment to Christ."),
                Document::from(Rubric::from("Silence may be kept.")),
                Document::from(Rubric::from("Then the Bishop says")),
                Document::from(Text::from("Almighty God, we thank you that by the death and resurrection of your Son Jesus Christ you have overcome sin and brought us to yourself, and that by the sealing of your Holy Spirit you have bound us to your service. Renew in *these* your *servants* the covenant you made with *them* at *their* Baptism. Send *them* forth in the power of that Spirit to perform the service you set before *them*; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.").response("Amen.")),

                Document::from(Heading::from((HeadingLevel::Heading3, "For Confirmation"))),
                Document::from(Rubric::from("The Bishop lays hands upon each one and says")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from("Strengthen, O Lord, your servant *N.* with your Holy Spirit; empower *him* for your service; and sustain *him* all the days of *his* life.").response("Amen.")),
                    Document::from(Text::from("Defend, O Lord, your servant *N.* with your heavenly grace, that *he* may continue yours for ever, and daily increase in your Holy Spirit more and more, until *he* comes to your everlasting kingdom.").response("Amen."))
                ])),

                Document::from(Heading::from((HeadingLevel::Heading3, "For Reception"))),
                Document::from(Text::from("*N.*, we recognize you as a member of the one holy catholic and apostolic Church, and we receive you into the fellowship of this Communion. God, the Father, Son, and Holy Spirit, bless, preserve, and keep you.").response("Amen.")),

                Document::from(Heading::from((HeadingLevel::Heading3, "For Reaffirmation"))),
                Document::from(Text::from("*N.*, may the Holy Spirit, who has begun a good work in you, direct and uphold you in the service of Christ and his kingdom.").response("Amen.")),

                Document::from(Rubric::from("Then the Bishop says")),
                Document::from(Text::from("Almighty and everliving God, let your fatherly hand ever be over *these* your *servants*; let your Holy Spirit ever be with *them*; and so lead *them* in the knowledge and obedience of your Word, that *they* may serve you in this life, and dwell with you in the life to come; through Jesus Christ our Lord.").response("Amen.")),

                Document::from(Rubric::from("The Peace is then exchanged")),
                Document::from(Preces::from([
                    ("Bishop", "The peace of the Lord be always with you."),
                    ("People", "And also with you.")
                ])),
            ])),

            // TODO preference is Eucharist celebrated?
            Document::from(Heading::from((HeadingLevel::Heading3, "At the Eucharist"))),
            Document::from(Rubric::from("The service then continues with the Prayers of the People or the Offertory of the Eucharist, at which the Bishop, when present, should be the principal Celebrant.\n\nExcept on Principal Feasts, the Proper Preface of Baptism may be used.")),

            // TODO preference is Eucharist celebrated?
            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading3, "Alternative Ending"))),
                Document::from(Rubric::from("If there is no celebration of the Eucharist, the service continues with the Lord’s Prayer")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from(LORDS_PRAYER_TRADITIONAL_TEXT).display_format(DisplayFormat::Unison).response("Amen.")),
                    Document::from(Text::from(LORDS_PRAYER_CONTEMPORARY_TEXT).display_format(DisplayFormat::Unison).response("Amen.")),
                ])),
                Document::from(Rubric::from("The Celebrant then says")),
                Document::from(Text::from("All praise and thanks to you, most merciful Father, for adopting us as your own children, for incorporating us into your holy Church, and for making us worthy to share in the inheritance of the saints in light; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.").response("Amen.")),
                Document::from(Rubric::from("Alms may be received and presented, and other prayers may be added, concluding with this prayer")),
                Document::from(Text::from("Almighty God, the Father of our Lord Jesus Christ, from whom every family in heaven and earth is named, grant you to be strengthened with might by his Holy Spirit, that, Christ dwelling in your hearts by faith, you may be filled with all the fullness of God.").response("Amen."))
            ]))
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

    pub static ref BAPTISMAL_COVENANT: Document = Document::from(Series::from(vec![
        Document::from(Heading::from((HeadingLevel::Heading2, "The Baptismal Covenant"))),
        Document::from(Preces::from([
            ("Celebrant", "Do you believe in God the Father?"),
            ("People", "I believe in God, the Father almighty,\n\tcreator of heaven and earth.\n "),
            ("Celebrant", "Do you believe in Jesus Christ, the Son of God?"),
            ("People", "I believe in Jesus Christ, his only Son, our Lord.\n\tHe was conceived by the power of the Holy Spirit\n\t\tand born of the Virgin Mary.\n\tHe suffered under Pontius Pilate,\n\t\twas crucified, died, and was buried.\n\tHe descended to the dead.\n\tOn the third day he rose again.\n\tHe ascended into heaven,\n\t\tand is seated at the right hand of the Father.\n\tHe will come again to judge the living and the dead.\n "),
            ("Celebrant", "Do you believe in God the Holy Spirit?"),
            ("People", "I believe in the Holy Spirit,\n\tthe holy catholic Church,\n\tthe communion of saints,\n\tthe forgiveness of sins,\n\tthe resurrection of the body,\n\tand the life everlasting.\n "),
            ("Celebrant", "Will you continue in the apostles’ teaching and fellowship, in the breaking of bread, and in the prayers?"),
            ("People", "I will, with God’s help.\n "),
            ("Celebrant", "Will you persevere in resisting evil, and, whenever you fall into sin, repent and return to the Lord?"),
            ("People", "I will, with God’s help.\n "),
            ("Celebrant", "Will you proclaim by word and example the Good News of God in Christ?"),
            ("People", "I will, with God’s help.\n "),
            ("Celebrant", "Will you seek and serve Christ in all persons, loving your neighbor as yourself?"),
            ("People", "I will, with God’s help.\n "),
            ("Celebrant", "Will you strive for justice and peace among all people, and respect the dignity of every human being?"),
            ("People", "I will, with God’s help.\n "),
        ]))
    ]));

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
