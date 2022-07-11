use crate::{
    bcp1979::burial::parallels::*,
    rite2::{
        CANTICLE_16, CANTICLE_17, LORDS_PRAYER_CONTEMPORARY_TEXT, LORDS_PRAYER_TRADITIONAL_TEXT,
        PASCHA_NOSTRUM,
    },
};
use lazy_static::lazy_static;
use liturgy::*;
use psalter::bcp1979::{
    PSALM_106, PSALM_116, PSALM_121, PSALM_130, PSALM_139, PSALM_23, PSALM_27, PSALM_42, PSALM_46,
    PSALM_90,
};

lazy_static! {
    pub static ref BURIAL_RITE_II: Document = Document::new()
        .version(Version::RiteII)
        .label("Burial II")
        .page(491)
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Burial of the Dead:\nRite Two"))).tags([TITLE]),
            Document::from(Rubric::from("All stand while one or more of the following anthems are sung or said.\n\nA hymn, psalm, or some other suitable anthem may be sung instead.")).tags([OPENING_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Text::from("I am Resurrection and I am Life, says the Lord.\nWhoever has faith in me shall have life,\neven though he die.\nAnd everyone who has life,\nand has committed himself to me in faith,\nshall not die for ever.\n\nAs for me, I know that my Redeemer lives\nand that at the last he will stand upon the earth.\nAfter my awaking, he will raise me up;\nand in my body I shall see God.\nI myself shall see, and my eyes behold him\nwho is my friend and not a stranger.\n\nFor none of us has life in himself,\nand none becomes his own master when he dies.\nFor if we have life, we are alive in the Lord,\nand if we die, we die in the Lord.\nSo, then, whether we live or die,\nwe are the Lord’s possession.\n\nHappy from now on\nare those who die in the Lord!\nSo it is, says the Spirit,\nfor they rest from their labors."))
                .version_label("I am Resurrection and I am Life"),
                Document::from(ResponsivePrayer::from([
                    "In the midst of life we are in death;\nfrom whom can we seek help?\nFrom you alone, O Lord,\nwho by our sins are justly angered.",
                    "Holy God, Holy and Mighty,\nHoly and merciful Savior,\ndeliver us not into the bitterness of eternal death.",
                    "Lord, you know the secrets of our hearts;\nshut not your ears to our prayers,\nbut spare us, O Lord.",
                    "Holy God, Holy and Mighty,\nHoly and merciful Savior,\ndeliver us not into the bitterness of eternal death.",
                    "O worthy and eternal Judge,\ndo not let the pains of death\nturn us away from you at our last hour.",
                    "Holy God, Holy and Mighty,\nHoly and merciful Savior,\ndeliver us not into the bitterness of eternal death.",
                ])).version_label("In the midst of life we are in death")
            ])).tags([ANTHEMS]),
            Document::from(Rubric::from("When all are in place, the Celebrant may address the congregation, acknowledging briefly the purpose of their gathering, and bidding their prayers for the deceased and the bereaved.")).tags([OPENING_PRAYERS]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant then says")),
                Preces::from([
                    ("", "The Lord be with you."),
                    ("People", "And also with you."),
                    ("Celebrant", "Let us pray.")
                ]).into(),
                Rubric::from("Silence may be kept; after which the Celebrant says one of the following Collects").into(),
                Choice::from(vec![
                    Document::from(Text::from("O God, who by the glorious resurrection of your Son Jesus Christ destroyed death, and brought life and immortality to light: Grant that your servant *N.*, being raised with him, may know the strength of his presence, and rejoice in his eternal glory; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.").response("Amen.")).label("At the Burial of an Adult"),
                    Document::from(Text::from("O God, whose mercies cannot be numbered: Accept our prayers on behalf of your servant *N.*, and grant *him* an entrance into the land of light and joy, in the fellowship of your saints; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.").response("Amen.")).label("At the Burial of an Adult"),
                    Document::from(Text::from("O God of grace and glory, we remember before you this day our brother (sister) *N.* We thank you for giving *him* to us, *his* family and friends, to know and to love as a companion on our earthly pilgrimage. In your boundless compassion, console us who mourn. Give us faith to see in death the gate of eternal life, so that in quiet confidence we may continue our course on earth, until, by your call, we are reunited with those who have gone before; through Jesus Christ our Lord.").response("Amen.")).label("At the Burial of an Adult"),
                    Document::from(Text::from("O God, whose beloved Son took children into his arms and blessed them: Give us grace to entrust *N.*, to your never-failing care and love, and bring us all to your heavenly kingdom; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.").response("Amen.")).label("At the Burial of a Child"),
                ]).into(),
                Rubric::from("The Celebrant may add the following prayer").into(),
                Text::from("Most merciful God, whose wisdom is beyond our understanding, deal graciously with *NN.* in *their* grief. Surround *them* with your love, that *they* may not be overwhelmed by their loss, but have confidence in your goodness, and strength to meet the days to come; through Jesus Christ our Lord.").response("Amen.").into(),
                Document::from(Rubric::from("The people sit."))
            ])).tags([OPENING_PRAYERS]),

            Document::from(Rubric::from("One or more of the following passages from Holy Scripture is read. If there is to be a Communion, a passage from the Gospel always concludes the Readings.")).tags([LESSONS_RUBRIC]),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Liturgy of the Word"))).tags([LESSONS_HEADING]),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Isaiah 25:6-9").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Isaiah.")))).label("From the Old Testament").version_label("Isaiah 25:6-9 (He will swallow up death for ever)"),
                Document::from(BiblicalCitation::from("Isaiah 61:1-3").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Isaiah.")))).label("From the Old Testament").version_label("Isaiah 61:1-3 (To comfort those who mourn)"),
                Document::from(BiblicalCitation::from("Lamentations 3:22-26, 31-33").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Lamentations.")))).label("From the Old Testament").version_label("Lamentations 3:22-26, 31-33 (The Lord is good to those who wait for him)"),
                Document::from(BiblicalCitation::from("Wisdom 3:1-5, 9").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Wisdom of Solomon.")))).label("From the Old Testament").version_label("Wisdom 3:1-5, 9 (The souls of the righteous are in the hands of God)"),
                Document::from(BiblicalCitation::from("Job 19:21-27a").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Job.")))).label("From the Old Testament").version_label("Job 19:21-27a (I know that my Redeemer lives)"),
            ])).tags([FIRST_LESSON]),

            Document::from(Rubric::from("A suitable psalm, hymn, or canticle may follow. The following Psalms are appropriate: 42:1-7, 46, 90:1-12, 121, 130, 139:1-11.")).tags([PSALM_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(PSALM_42.clone().citation("Psalm 42:1-7")),
                Document::from(PSALM_46.clone()),
                Document::from(PSALM_90.clone().citation("Psalm 90:1-12")),
                Document::from(PSALM_121.clone()),
                Document::from(PSALM_130.clone()),
                Document::from(PSALM_139.clone().citation("Psalm 139:1-11")),
            ])).tags([PSALM]),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Romans 8:14-19, 34-35, 37-39").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Romans.")))).label("From the New Testament").version_label("Romans 8:14-19, 34-35, 37-39 (The glory that shall be revealed)"),
                Document::from(BiblicalCitation::from("1 Corinthians 15:20-26, 35-38, 42-44, 53-58").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter to the Corinthians.")))).label("From the New Testament").version_label("1 Corinthians 15:20-26, 35-38, 42-44, 53-58 (The imperishable body)"),
                Document::from(BiblicalCitation::from("2 Corinthians 4:16-5:9").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Second Letter to the Corinthians.")))).label("From the New Testament").version_label("2 Corinthians 4:16–5:9 (Things that are unseen are eternal)"),
                Document::from(BiblicalCitation::from("1 John 3:1-2").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter of John.")))).label("From the New Testament").version_label("1 John 3:1-2 (We shall be like him)"),
                Document::from(BiblicalCitation::from("Revelation 7:9-17").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Revelation.")))).label("From the New Testament").version_label("Revelation 7:9-17 (God will wipe away every tear)"),
                Document::from(BiblicalCitation::from("Revelation 21:2-7").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Revelation.")))).label("From the New Testament").version_label("Revelation 21:2-7 (Behold, I make all things new)"),
            ])).tags([SECOND_LESSON]),

            Document::from(Rubric::from("A suitable psalm, hymn, or canticle may follow. The following Psalms are appropriate: 23, 27, 106:1-5, 116.")).tags([PSALM_RUBRIC_2]),
            Document::from(Choice::from(vec![
                Document::from(PSALM_23.clone()),
                Document::from(PSALM_27.clone()),
                Document::from(PSALM_106.clone().citation("Psalm 106:1-5")),
                Document::from(PSALM_116.clone()),
            ])).tags([PSALM_2]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("Then, all standing, the Deacon or Minister appointed reads the Gospel, first saying")),
                Document::from(Preces::from([
                    ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                    ("People", "Glory to you, Lord Christ.")
                ]))
            ]))
                .display(Show::TemplateOnly)
                .tags([GOSPEL_RUBRIC]),

            Document::from(Choice::from(vec![
                Document::from(
                    BiblicalCitation::from("John 5:24-27")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 5:24-27 (He who believes has everlasting life)"),
                Document::from(
                    BiblicalCitation::from("John 6:37-40")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 6:37-40 (All that the Father gives me will come to me)"),
               Document::from(
                    BiblicalCitation::from("John 10:11-16")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 10:11-16 (I am the good shepherd)"),
                Document::from(
                    BiblicalCitation::from("John 11:21-27")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 11:21-27 (I am the resurrection and the life)"),
                Document::from(
                    BiblicalCitation::from("John 14:1-6")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 14:1-6 (In my Father’s house are many rooms)"),
            ])).tags([GOSPEL]),
            Document::from(Rubric::from("At the end of the Gospel, the Reader says")).tags([GOSPEL]),
            Document::from(Preces::from([
                ("", "The Gospel of the Lord."),
                ("People", "Praise to you, Lord Christ.")
            ])).tags([GOSPEL]),

            Document::from(Rubric::from("Here there may be a homily by the Celebrant, or a member of the family, or a friend.")).tags([HOMILY]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Apostles’ Creed may then be said, all standing. The Celebrant may introduce the Creed with these or similar words")),
                Document::from("In the assurance of eternal life given at Baptism, let us proclaim our faith and say,"),
                Document::from(Rubric::from("Celebrant and People")),
                Document::from(Text::from("I believe in God, the Father almighty,\n\tcreator of heaven and earth.\nI believe in Jesus Christ, his only Son, our Lord.\n\tHe was conceived by the power of the Holy Spirit\n\t\tand born of the Virgin Mary.\n\tHe suffered under Pontius Pilate,\n\t\twas crucified, died, and was buried.\n\tHe descended to the dead.\n\tOn the third day he rose again.\n\tHe ascended into heaven,\n\t\tand is seated at the right hand of the Father.\n\tHe will come again to judge the living and the dead.\n\nI believe in the Holy Spirit,\n\tthe holy catholic Church,\n\tthe communion of saints,\n\tthe forgiveness of sins,\n\tthe resurrection of the body,\n\tand the life everlasting. Amen.").display_format(DisplayFormat::Unison))
            ])).tags([CREED]),

            // Prayers
            Document::from(Rubric::from("If there is not to be a Communion, the Lord’s Prayer is said here, and the service continues with the Prayers of the People, or with one or more suitable prayers (see pages 503-505).\n\nWhen there is a Communion, the following form of the Prayers of the People is used, or else the form on page 465 or 480.")).tags([RUBRIC_BEFORE_PRAYERS]),
            Document::from(Content::DocumentLink {
                label: "Additional Prayers".into(),
                path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::RiteII)]),
                rotate: false,
                link_only: false
            }).version(Version::RiteII).tags([RUBRIC_BEFORE_PRAYERS]),
            Document::from(Series::from(vec![
                Document::from(Litany::from((
                    "Hear us, Lord.",
                    [
                        "For our brother (sister) *N.*, let us pray to our Lord Jesus Christ who said, “I am Resurrection and I am Life.”\n\nLord, you consoled Martha and Mary in their distress; draw near to us who mourn for *N.*, and dry the tears of those who weep.",
                        "You wept at the grave of Lazarus, your friend; comfort us in our sorrow.",
                        "You raised the dead to life; give to our brother (sister) eternal life.",
                        "You promised paradise to the thief who repented; bring our brother (sister) to the joys of heaven.",
                        "| Our brother (sister) was washed in Baptism and anointed with the Holy Spirit; give *him* fellowship with all your saints.",
                        "| *He* was nourished with your Body and Blood; grant *him* a place at the table in your heavenly kingdom.",
                        "Comfort us in our sorrows at the death of our brother (sister); let our faith be our consolation, and eternal life our hope."
                    ]
                ))),
                Document::from(Rubric::from("Silence may be kept.")),
                Document::from(Rubric::from("The Celebrant concludes with one of the following or some other prayer")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from("Lord Jesus Christ, we commend to you our brother (sister) *N.*, who was reborn by water and the Spirit in Holy Baptism. Grant that *his* death may recall to us your victory over death, and be an occasion for us to renew our trust in your Father’s love. Give us, we pray, the faith to follow where you have led the way; and where you live and reign with the Father and the Holy Spirit, to the ages of ages.").response("Amen.")),
                    Document::from(Text::from("Father of all, we pray to you for *N.*, and for all those whom we love but see no longer. Grant to them eternal rest. Let light perpetual shine upon them. May *his* soul and the souls of all the departed, through the mercy of God, rest in peace.").response("Amen."))
                ])),
            ])).tags([PRAYERS]),

            Document::from(Rubric::from("When there is no Communion, the service continues with the Commendation, or with the Committal.")).tags([RUBRIC_AFTER_PRAYERS]),

            Document::from(Heading::from((HeadingLevel::Heading2, "At the Eucharist."))).tags([AT_THE_EUCHARIST_TITLE]),
            Document::from(Rubric::from("The service continues with the Peace and the Offertory.")).tags([AT_THE_EUCHARIST]),
            Document::from(Content::DocumentLink {
                label: "Preface of the Commemoration of the Dead".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Burial, Slug::Version(Version::RiteII)]),
                rotate: false,
                link_only: false
            }).tags([PROPER_PREFACE]),
            Document::from(Rubric::from("In place of the usual postcommunion prayer, the following is said")).tags([POSTCOMMUNION_PRAYER]),
            Document::from(Text::from("Almighty God, we thank you that in your great love you have fed us with the spiritual food and drink of the Body and Blood of your Son Jesus Christ, and have given us a foretaste of your heavenly banquet. Grant that this Sacrament may be to us a comfort in affliction, and a pledge of our inheritance in that kingdom where there is no death, neither sorrow nor crying, but the fullness of joy with all your saints; through Jesus Christ our Savior.").response("Amen.").display_format(DisplayFormat::Unison)).tags([POSTCOMMUNION_PRAYER]),

            Document::from(Rubric::from("If the body is not present, the service continues with the (blessing and) dismissal.\n\nUnless the Committal follows immediately in the church, the following Commendation is used.")).tags([RUBRICS_BEFORE_COMMENDATION]),

            // Commendation
            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Commendation"))),
                Document::from(Rubric::from("The Celebrant and other ministers take their places at the body.\n\nThis anthem, or some other suitable anthem, or a hymn, may be sung or said.")),
                Document::from(ResponsivePrayer::from([
                    "Give rest, O Christ, to your servant(s) with your saints,",
                    "where sorrow and pain are no more,\nneither sighing, but life everlasting.\n",
                    "You only are immortal, the creator and maker of mankind; and we are mortal, formed of the earth, and to earth shall we return. For so did you ordain when you created me, saying, “You are dust, and to dust you shall return.” All of us go down to the dust, yet even at the grave we make our song: Alleluia, alleluia, alleluia.",
                    "Give rest, O Christ, to your servant(s) with your saints,\nwhere sorrow and pain are no more,\nneither sighing, but life everlasting.",
                ])),
                Document::from(Rubric::from("The Celebrant, facing the body, says")),
                Document::from(Text::from("Into your hands, O merciful Savior, we commend your servant *N.* Acknowledge, we humbly beseech you, a sheep of your own fold, a lamb of your own flock, a sinner of your own redeeming. Receive *him* into the arms of your mercy, into the blessed rest of everlasting peace, and into the glorious company of the saints in light.").response("Amen.")),
                Document::from(Rubric::from("The Celebrant, or the Bishop if present, may then bless the people, and a Deacon or other Minister may dismiss them, saying")),
                Document::from(ResponsivePrayer::from([
                    "Let us go forth in the name of Christ.",
                    "Thanks be to God."
                ])),
                Document::from(Rubric::from("As the body is borne from the church, a hymn, or one or more of these anthems may be sung or said")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from("Christ is risen from the dead, trampling down death by death, and giving life to those in the tomb. \n\nThe Sun of Righteousness is gloriously risen, giving light to those who sat in darkness and in the shadow of death. \n\nThe Lord will guide our feet into the way of peace, having taken away the sin of the world. \n\nChrist will open the kingdom of heaven to all who believe in his Name, saying, Come, O blessed of my Father; inherit the kingdom prepared for you. \n\nInto paradise may the angels lead you. At your coming may the martyrs receive you, and bring you into the holy city Jerusalem.")).version_label("Anthems"),
                    CANTICLE_16.clone(),
                    CANTICLE_17.clone(),
                    Document::from(PASCHA_NOSTRUM.clone()).version_label("Pascha Nostrum"),
                ])),
            ])).tags([COMMENDATION]),

            // Committal
            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Committal"))),
                Document::from(Rubric::from("The following anthem or one of those on pages 491-492 is sung or said")),
                Document::from(Text::from("Everyone the Father gives to me will come to me; I will never turn away anyone who believes in me. \n\nHe who raised Jesus Christ from the dead will also give new life to our mortal bodies through his indwelling Spirit. My heart, therefore, is glad, and my spirit rejoices; my body also shall rest in hope. \n\nYou will show me the path of life; in your presence there is fullness of joy, and in your right hand are pleasures for evermore.")),
                Document::from(Rubric::from("Then, while earth is cast upon the coffin, the Celebrant says these words")),
                Document::from(Text::from("In sure and certain hope of the resurrection to eternal life through our Lord Jesus Christ, we commend to Almighty God our *brother N.*, and we commit *his* body to the ground;\\* earth to earth, ashes to ashes, dust to dust. The Lord bless *him* and keep *him*, the Lord make his face to shine upon *him* and be gracious to *him*, the Lord lift up his countenance upon *him* and give *him* peace.").response("Amen.")),
                Document::from(Text::from("\\* *Or* the deep, *or* the elements, *or* its resting place.")),
            ])).tags([COMMITTAL]),
            Document::from(Rubric::from("The Celebrant says")).tags([COMMITTAL_PRAYERS]),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Let us pray.")
            ])).tags([COMMITTAL_PRAYERS]),
            Document::from(Rubric::from("Celebrant and People")).tags([COMMITTAL_PRAYERS]),
            Document::from(Choice::from(vec![
                Document::from(Text::from(LORDS_PRAYER_CONTEMPORARY_TEXT).response("Amen.").display_format(DisplayFormat::Unison)).version_label("Contemporary"),
                Document::from(Text::from(LORDS_PRAYER_TRADITIONAL_TEXT).response("Amen.").display_format(DisplayFormat::Unison)).version_label("Traditional"),
            ])).tags([COMMITTAL_LORDS_PRAYER]),
            Document::from(Rubric::from("Other prayers may be added.")).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Content::DocumentLink {
                label: "Additional Prayers".into(),
                path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::RiteII)]),
                rotate: false,
                link_only: false
            }).version(Version::RiteII).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Rubric::from("Then may be said.")).tags([COMMITTAL_PRAYERS_2]),
            Document::from(ResponsivePrayer::from([
                "Rest eternal grant to *him*, O Lord;",
                "And let light perpetual shine upon *him*."
            ])).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Text::from("May *his* soul, and the souls of all the departed,\nthrough the mercy of God, rest in peace.").response("Amen.")).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Rubric::from("The Celebrant dismisses the people with these words")).tags([DISMISSAL]),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("", "Alleluia. Christ is risen."),
                    ("People", "The Lord is risen indeed. Alleluia."),
                    ("Celebrant", "Let us go forth in the name of Christ."),
                    ("People", "Thanks be to God.")
                ])),
                Document::from(Text::from("The God of peace, who brought again from the dead our Lord Jesus Christ, the great Shepherd of the sheep, through the blood of the everlasting covenant: Make you perfect in every good work to do his will, working in you that which is well-pleasing in his sight; through Jesus Christ, to whom be glory for ever and ever.").response("Amen."))
            ])).tags([DISMISSAL]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading3, "The Consecration of a Grave"))),
                Document::from(Rubric::from("If the grave is in a place that has not previously been set apart for Christian burial, the Priest may use the following prayer, either before the service of Committal or at some other convenient time")),
                Document::from(Text::from("O God, whose blessed Son was laid in a sepulcher in the garden: Bless, we pray, this grave, and grant that *he* whose body is (is to be) buried here may dwell with Christ in paradise, and may come to your heavenly kingdom; through your Son Jesus Christ our Lord.").response("Amen."))
            ])).tags([CONSECRATION_OF_A_GRAVE]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "Additional Prayers"))),
                Document::from(Content::DocumentLink {
                    label: "Additional Prayers".into(),
                    path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::RiteII)]),
                    rotate: false,
                    link_only: false
                }).version(Version::RiteII)
            ])).tags([ADDITIONAL_PRAYERS])
        ]))));

    pub static ref ADDITIONAL_PRAYERS_BURIAL: Vec<Document> = vec![
        Document::from(Text::from("Almighty God, with whom still live the spirits of those who die in the Lord, and with whom the souls of the faithful are in joy and felicity: We give you heartfelt thanks for the good examples of all your servants, who, having finished their course in faith, now find rest and refreshment. May we, with all who have died in the true faith of your holy Name, have perfect fulfillment and bliss in your eternal and everlasting glory, through Jesus Christ our Lord. ").response("Amen.")).page(503),
        Document::from(Text::from("O God, whose days are without end, and whose mercies cannot be numbered: Make us, we pray, deeply aware of the shortness and uncertainty of human life; and let your Holy Spirit lead us in holiness and righteousness all our days; that, when we shall have served you in our generation, we may be gathered to our ancestors, having the testimony of a good conscience, in the communion of the Catholic Church, in the confidence of a certain faith, in the comfort of a religious and holy hope, in favor with you, our God, and in perfect charity with the world. All this we ask through Jesus Christ our Lord.").response("Amen.")).page(504),
        Document::from(Text::from("O God, the King of saints, we praise and glorify your holy Name for all your servants who have finished their course in your faith and fear: for the blessed Virgin Mary; for the holy patriarchs, prophets, apostles, and martyrs; and for all your other righteous servants, known to us and unknown; and we pray that, encouraged by their examples, aided by their prayers, and strengthened by their fellowship, we also may be partakers of the inheritance of the saints in light; through the merits of your Son Jesus Christ our Lord.").response("Amen.")).page(504),
        Document::from(Text::from("Lord Jesus Christ, by your death you took away the sting of death: Grant to us your servants so to follow in faith where you have led the way, that we may at length fall asleep peacefully in you and wake up in your likeness; for your tender mercies’ sake.").response("Amen.")).page(504),
        Document::from(Text::from("Father of all, we pray to you for those we love, but see no longer: Grant them your peace; let light perpetual shine upon them; and, in your loving wisdom and almighty power, work in them the good purpose of your perfect will; through Jesus Christ our Lord.").response("Amen.")).page(504),
        Document::from(Text::from("Merciful God, Father of our Lord Jesus Christ who is the Resurrection and the Life: Raise us, we humbly pray, from the death of sin to the life of righteousness; that when we depart this life we may rest in him, and at the resurrection receive that blessing which your well-beloved Son shall then pronounce: “Come, you blessed of my Father, receive the kingdom prepared for you from the beginning of the world.” Grant this, O merciful Father, through Jesus Christ, our Mediator and Redeemer.").response("Amen.")).page(505),
        Document::from(Text::from("Grant, O Lord, to all who are bereaved the spirit of faith and courage, that they may have strength to meet the days to come with steadfastness and patience; not sorrowing as those without hope, but in thankful remembrance of your great goodness, and in the joyful expectation of eternal life with those they love. And this we ask in the Name of Jesus Christ our Savior.").response("Amen.")).page(505),
        Document::from(Text::from("Almighty God, Father of mercies and giver of comfort: Deal graciously, we pray, with all who mourn; that, casting all their care on you, they may know the consolation of your love; through Jesus Christ our Lord.").response("Amen.")).page(505)
    ];
}
