use lazy_static::lazy_static;
use liturgy::*;
use psalter::bcp1979::{PSALM_127, PSALM_128, PSALM_67};

use crate::marriage_alternatives::parallels::*;
use crate::rite2::LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL;

lazy_static! {
    pub static ref BCP_WEDDING_READINGS: Document = Document::from(Series::from(vec![
        // First Reading
        // note: there are no responses given in the BCP marriage liturgy except for the Gospel
        Document::from(Choice::from(vec![
            Document::from(BiblicalCitation::from("Genesis 1:26-28").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Genesis.")))).label("The First Reading").version_label("Genesis 1:26-28 (Male and female he created them)"),
            Document::from(BiblicalCitation::from("Genesis 2:4-9, 15-24").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Genesis.")))).label("The First Reading").version_label("Genesis 2:4-9, 15-24 (A man cleaves to his wife and they become one flesh)"),
            Document::from(BiblicalCitation::from("Song of Solomon 2:10-13; 8:6-7").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Song of Songs.")))).label("The First Reading").version_label("Song of Solomon 2:10-13; 8:6-7 (Many waters cannot quench love)"),
            Document::from(BiblicalCitation::from("Tobit 8:5b-8").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Tobit.")))).label("The First Reading").version_label("Tobit 8:5b-8 (New English Bible) (That she and I may grow old together)")
        ])).tags([FIRST_LESSON]),

        // Psalm/Hymn
        Document::from(Rubric::from("Between the Readings, a Psalm, hymn, or anthem may be sung or said. Appropriate Psalms are 67, 127, and 128.")).tags([PSALM]),
        Document::from(HymnLink::Tag("Marriage".into())),
        Document::from(Choice::from(vec![
            Document::from(PSALM_67.clone()),
            Document::from(PSALM_127.clone()),
            Document::from(PSALM_128.clone())
        ])).tags([PSALM]),

        // Second Reading
        Document::from(Choice::from(vec![
            Document::from(BiblicalCitation::from("1 Corinthians 13:1-13").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter to the Corinthians.")))).label("The Second Reading").version_label("1 Corinthians 13:1-13 (Love is patient and kind)"),
            Document::from(BiblicalCitation::from("Ephesians 3:14-19").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Ephesians.")))).label("The Second Reading").version_label("Ephesians 3:14-19 (The Father from whom every family is named)"),
            Document::from(BiblicalCitation::from("Ephesians 5:1-2, 21-33").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Ephesians.")))).label("The Second Reading").version_label("Ephesians 5:1-2, 21-33 (Walk in love, as Christ loved us)"),
            Document::from(BiblicalCitation::from("Colossians 3:12-17").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Colossians.")))).label("The Second Reading").version_label("Colossians 3:12-17 (Love which binds everything together in harmony)"),
            Document::from(BiblicalCitation::from("1 John 4:7-16").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter of John.")))).label("The Second Reading").version_label("1 John 4:7-16 (Let us love one another for love is of God)"),
        ])).tags([SECOND_LESSON]),

        // Gospel
        Document::from(Rubric::from("When a passage from the Gospel is to be read, all stand, and the Deacon or Minister appointed says")).tags([GOSPEL]),
        Document::from(Preces::from([
            ("", "The Holy Gospel of our Lord Jesus Christ according to _____________."),
            ("People", "Glory to you, Lord Christ.")
        ])).tags([GOSPEL]).display(Show::TemplateOnly),

        Document::from(Choice::from(vec![
            Document::from(
                BiblicalCitation::from("Matthew 5:1-10")
                    .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                        ("People", "Glory to you, Lord Christ.")
                    ]))))
            )
                .label("The Gospel").version_label("Matthew 5:1-10 (The Beatitudes)"),
            Document::from(
                BiblicalCitation::from("Matthew 5:13-16")
                    .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                        ("People", "Glory to you, Lord Christ.")
                    ]))))
            )
                .label("The Gospel").version_label("Matthew 5:13-16 (You are the light ... Let your light so shine)"),
            Document::from(
                BiblicalCitation::from("Matthew 7:21, 24-29")
                    .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                        ("People", "Glory to you, Lord Christ.")
                    ]))))
            )
                .label("The Gospel").version_label("Matthew 7:21, 24-29 (Like a wise man who built his house upon the rock)"),
            Document::from(
                BiblicalCitation::from("Mark 10:6-9, 13-16")
                    .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to Mark."),
                        ("People", "Glory to you, Lord Christ.")
                    ]))))
            )
                .label("The Gospel").version_label("Mark 10:6-9, 13-16 (They are no longer two but one)"),
            Document::from(
                BiblicalCitation::from("John 15:9-12")
                    .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                        ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                        ("People", "Glory to you, Lord Christ.")
                    ]))))
            )
                .label("The Gospel").version_label("John 15:9-12 (Love one another as I have loved you)"),
        ])).tags([GOSPEL]),

        Document::from(Rubric::from("After the Gospel, the Reader says")).tags([GOSPEL]),
        Document::from(Preces::from([
            ("", "The Gospel of the Lord."),
            ("People", "Praise to you, Lord Christ.")
        ])).tags([GOSPEL]),
    ]));

    pub static ref CELEBRATION_AND_BLESSING_OF_A_MARRIAGE: Document = Document::new()
        .label("Celebration and Blessing of a Marriage")
        .version(Version::BCP1979)
        .page(423)
        .explainer("The standard marriage service from the 1979 Book of Common Prayer.")
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Celebration and\nBlessing of a Marriage"))).tags([TITLE]),
            Document::from(Rubric::from("At the time appointed, the persons to be married, with their witnesses, assemble in the church or some other appropriate place.")).tags([PROCESSION_RUBRIC]),
            Document::from(Rubric::from("During their entrance, a hymn, psalm, or anthem may be sung, or instrumental music may be played.")).tags([PROCESSION_MUSIC_RUBRIC]),
            Document::from(HymnLink::Tag("Marriage".into())).tags([OPENING_HYMN]),

            // Opening Words
            Document::from(Rubric::from("Then the Celebrant, facing the people and the persons to be married, with the woman to the right and the man to the left, addresses the congregation and says")).tags([OPENING_ADDRESS_RUBRIC]),
            Document::from("Dearly beloved: We have come together in the presence of God to witness and bless the joining together of this man and this woman in Holy Matrimony. The bond and covenant of marriage was established by God in creation, and our Lord Jesus Christ adorned this manner of life by his presence and first miracle at a wedding in Cana of Galilee. It signifies to us the mystery of the union between Christ and his Church, and Holy Scripture commends it to be honored among all people.\n\nThe union of husband and wife in heart, body, and mind is intended by God for their mutual joy; for the help and comfort given one another in prosperity and adversity; and, when it is God’s will, for the procreation of children and their nurture in the knowledge and love of the Lord. Therefore marriage is not to be entered into unadvisedly or lightly, but reverently, deliberately, and in accordance with the purposes for which it was instituted by God.\n\nInto this holy union *N. N.* and *N. N.* now come to be joined.\n\nIf any of you can show just cause why they may not lawfully be married, speak now; or else for ever hold your peace.").tags([OPENING_ADDRESS]),
            Document::from(Rubric::from("Then the Celebrant says to the persons to be married")).tags([OPENING_WORDS_TO_COUPLE_RUBRIC]),
            Document::from("I require and charge you both, here in the presence of God, that if either of you know any reason why you may not be united in marriage lawfully, and in accordance with God’s Word, you do now confess it.").tags([OPENING_WORDS_TO_COUPLE]),

            // Declaration of Consent
            Document::from(Heading::from((HeadingLevel::Heading2, "The Declaration of Consent"))).tags([CONSENT_HEADER]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant says to the woman")),
                Document::from("*N.*, will you have this man to be your husband; to live together in the covenant of marriage? Will you love him, comfort him, honor and keep him, in sickness and in health; and, forsaking all others, be faithful to him as long as you both shall live?"),
                Document::from(Rubric::from("The Woman answers")),
                Document::from("I will."),
                Document::from(Rubric::from("The Celebrant says to the man")),
                Document::from("*N.*, will you have this woman to be your wife; to live together in the covenant of marriage? Will you love her, comfort her, honor and keep her, in sickness and in health; and, forsaking all others, be faithful to her as long as you both shall live?"),
                Document::from(Rubric::from("The Man answers")),
                Document::from("I will."),
            ])).tags([CONSENT]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant then addresses the congregation, saying")),
                Document::from("Will all of you witnessing these promises do all in your power to uphold these two persons in their marriage?"),
                Document::from(Preces::from([
                    ("", ""),
                    ("People", "We will.")
                ]))
            ])).tags([CONSENT_CONGREGATION]),


            Document::from(Rubric::from("If there is to be a presentation or a giving in marriage, it takes place at this time.")).tags([PRESENTATION_RUBRIC]),
            Document::from(Content::DocumentLink {
                label: "Additional Directions".into(),
                path: SlugPath::from([Slug::Marriage, Slug::AdditionalDirections]),
                rotate: false,
                link_only: false
            }).tags([ADDITIONAL_DIRECTIONS_PARALLEL]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("A hymn, psalm, or anthem may follow.")),
                Document::from(HymnLink::Tag("Marriage".into())),
            ])).tags([HYMN_PSALM_OR_ANTHEM]),

            // Liturgy of the Word
            Document::from(Heading::from((HeadingLevel::Heading2, "The Ministry of the Word"))).tags([MINISTRY_OF_THE_WORD]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant then says to the people")),
                Document::from(Preces::from([
                    ("", "The Lord be with you."),
                    ("People", "And also with you.")
                ])),
                Document::from("Let us pray."),
            ])).tags([SALUTATION]),
            Document::from(Text::from("O gracious and everliving God, you have created us male and female in your image: Look mercifully upon this man and this woman who come to you seeking your blessing, and assist them with your grace, that with true fidelity and steadfast love they may honor and keep the promises and vows they make; through Jesus Christ our Savior, who lives and reigns with you in the unity of the Holy Spirit, one God, for ever and ever.").response("Amen.")).tags([COLLECT]),

            // Readings
            Document::from(Rubric::from("Then one or more of the following passages from Holy Scripture is read. If there is to be a Communion, a passage from the Gospel always concludes the Readings.")).tags([SCRIPTURE_RUBRIC]),

            BCP_WEDDING_READINGS.clone(),

            Document::from(Rubric::from("A homily or other response to the Readings may follow.")).tags([HOMILY]),

            // The Marriage
            Document::from(Heading::from((HeadingLevel::Heading2, "The Marriage"))).tags([THE_MARRIAGE_HEADER]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Man, facing the woman and taking her right hand in his, says")),
                Document::from("In the Name of God, I, *N.*, take you, *N.*, to be my wife, to have and to hold from this day forward, for better for worse, for richer for poorer, in sickness and in health, to love and to cherish, until we are parted by death. This is my solemn vow."),
                Document::from(Rubric::from("Then they loose their hands, and the Woman, still facing the man, takes his right hand in hers, and says")),
                Document::from("In the Name of God, I, *N.*, take you, *N.*, to be my husband, to have and to hold from this day forward, for better for worse, for richer for poorer, in sickness and in health, to love and to cherish, until we are parted by death. This is my solemn vow."),
                Document::from(Rubric::from("They loose their hands.")),
            ])).tags([THE_MARRIAGE]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Priest may ask God’s blessing on a ring or rings as follows")),
                Document::from(Text::from("Bless, O Lord, *this ring* to be a *sign* of the vows by which this man and this woman have bound themselves to each other; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Rubric::from("The giver places the ring on the ring-finger of the other’s hand and says")),
                Document::from(Text::from("*N.*, I give you this ring as a symbol of my vow, and with all that I am, and all that I have, I honor you, in the Name of the Father, and of the Son, and of the Holy Spirit (*or* in the Name of God).")),
            ])).tags([THE_RINGS]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("Then the Celebrant joins the right hands of husband and wife and says")),
                Document::from("Now that *N.* and *N.* have given themselves to each other by solemn vows, with the joining of hands and the giving and receiving of *a ring*, I pronounce that they are husband and wife, in the Name of the Father, and of the Son, and of the Holy Spirit.\n\nThose whom God has joined together let no one put asunder."),
                Document::from(Preces::from([
                    ("", ""),
                    ("People", "Amen.")
                ])),
            ])).tags([PRONOUNCEMENT]),

            // The Prayers
            Document::from(Heading::from((HeadingLevel::Heading2, "The Prayers"))).tags([THE_PRAYERS_HEADER]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("All standing, the Celebrant says")),
                Document::from("Let us pray together in the words our Savior taught us."),
                Document::from(Rubric::from("People and Celebrant")),
                Document::from(LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL.clone()),
                Document::from(Rubric::from("If Communion is to follow, the Lord’s Prayer may be omitted here.")),
            ])).tags([LORDS_PRAYER]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Deacon or other person appointed reads the following prayers, to which the People respond, saying, Amen.\n\nIf there is not to be a Communion, one or more of the prayers may be omitted.")),
                Document::from(Litany::from((
                    "Amen.",
                    [
                        "Let us pray.\n\nEternal God, creator and preserver of all life, author of salvation, and giver of all grace: Look with favor upon the world you have made, and for which your Son gave his life, and especially upon this man and this woman whom you make one flesh in Holy Matrimony.",
                        "Give them wisdom and devotion in the ordering of their common life, that each may be to the other a strength in need, a counselor in perplexity, a comfort in sorrow, and a companion in joy.",
                        "Grant that their wills may be so knit together in your will, and their spirits in your Spirit, that they may grow in love and peace with you and one another all the days of their life.",
                        "Give them grace, when they hurt each other, to recognize and acknowledge their fault, and to seek each other’s forgiveness and yours.",
                        "Make their life together a sign of Christ’s love to this sinful and broken world, that unity may overcome estrangement, forgiveness heal guilt, and joy conquer despair.",
                        "| Bestow on them, if it is your will, the gift and heritage of children, and the grace to bring them up to know you, to love you, and to serve you.",
                        "Give them such fulfillment of their mutual affection that they may reach out in love and concern for others.",
                        "Grant that all married persons who have witnessed these vows may find their lives strengthened and their loyalties confirmed.",
                        "Grant that the bonds of our common humanity, by which all your children are united one to another, and the living to the dead, may be so transformed by your grace, that your will may be done on earth as it is in heaven; where, O Father, with your Son and the Holy Spirit, you live and reign in perfect unity, now and for ever."
                    ]
                ))),
            ])).tags([THE_PRAYERS]),

            // Blessing
            Document::from(Heading::from((HeadingLevel::Heading2, "The Blessing of the Marriage"))).tags([BLESSING_OF_THE_MARRIAGE]),
            Document::from(Rubric::from("The people remain standing. The husband and wife kneel, and the Priest says one of the following prayers")).tags([BLESSING_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Text::from("Most gracious God, we give you thanks for your tender love in sending Jesus Christ to come among us, to be born of a human mother, and to make the way of the cross to be the way of life. We thank you, also, for consecrating the union of man and woman in his Name. By the power of your Holy Spirit, pour out the abundance of your blessing upon this man and this woman. Defend them from every enemy. Lead them into all peace. Let their love for each other be a seal upon their hearts, a mantle about their shoulders, and a crown upon their foreheads. Bless them in their work and in their companionship; in their sleeping and in their waking; in their joys and in their sorrows; in their life and in their death. Finally, in your mercy, bring them to that table where your saints feast for ever in your heavenly home; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.").response("Amen.")),
                Document::from(Text::from("O God, you have so consecrated the covenant of marriage that in it is represented the spiritual unity between Christ and his Church: Send therefore your blessing upon these your servants, that they may so love, honor, and cherish each other in faithfulness and patience, in wisdom and true godliness, that their home may be a haven of blessing and peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.").response("Amen."))
            ])).tags([BLESSING_PRAYERS]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The husband and wife still kneeling, the Priest adds this blessing")),
                Document::from(Text::from("God the Father, God the Son, God the Holy Spirit, bless, preserve, and keep you; the Lord mercifully with his favor look upon you, and fill you with all spiritual benediction and grace; that you may faithfully live together in this life, and in the age to come have life everlasting.").response("Amen.")),
            ])).tags([BLESSING_PROPER]),

            // Peace and Dismissal
            Document::from(Heading::from((HeadingLevel::Heading3, "The Peace"))).tags([PEACE_HEADER]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant may say to the people")),
                Document::from(Preces::from([
                    ("", "The peace of the Lord be always with you."),
                    ("People", "And also with you.")
                ])),
            ])).tags([THE_PEACE]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The newly married couple then greet each other, after which greetings may be exchanged throughout the congregation.")),
                Document::from(Rubric::from("When Communion is not to follow, the wedding party leaves the church. A hymn, psalm, or anthem may be sung, or instrumental music may be played.")),
            ])).tags([POST_PEACE_RUBRICS]),

            Document::from(HymnLink::Tag("Marriage".into())).tags([POST_PEACE_HYMN]),

            // Instructions: "At the Eucharist"
            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "At the Eucharist"))).tags([AT_THE_EUCHARIST]),
                Document::from(Rubric::from("The liturgy continues with the Offertory, at which the newly married couple may present the offerings of bread and wine.")).tags([OFFERTORY_RUBRIC]),
                Document::from(Content::DocumentLink {
                    label: "Preface of Marriage".into(),
                    path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Marriage]),
                    rotate: false,
                    link_only: false
                }).tags([PROPER_PREFACE]),
                Document::from(Rubric::from("At the Communion, it is appropriate that the newly married couple receive Communion first, after the ministers.")).tags([POST_PREFACE_RUBRIC]),
                Document::from(Rubric::from("In place of the usual postcommunion prayer, the following is said")).tags([POSTCOMMUNION_PRAYER]),
                Document::from(Text::from("O God, the giver of all that is true and lovely and gracious: We give you thanks for binding us together in these holy mysteries of the Body and Blood of your Son Jesus Christ. Grant that by your Holy Spirit, *N.* and *N.*, now joined in Holy Matrimony, may become one in heart and soul, live in fidelity and peace, and obtain those eternal joys prepared for all who love you; for the sake of Jesus Christ our Lord.").response("Amen.").display_format(DisplayFormat::Unison)).tags(["Postcommunion Prayer", POSTCOMMUNION_PRAYER]),
                Document::from(Rubric::from("As the wedding party leaves the church, a hymn, psalm, or anthem may be sung; or instrumental music may be played.")).tags([CLOSING_HYMN]),
                Document::from(HymnLink::Tag("Marriage".into())).tags([CLOSING_HYMN]),
            ])).display(Show::TemplateOnly)
        ])
    )));
}
