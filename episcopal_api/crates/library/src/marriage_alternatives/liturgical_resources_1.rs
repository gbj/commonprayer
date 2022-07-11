use lazy_static::lazy_static;
use liturgy::*;
use psalter::bcp1979::*;
use status::Status;

use crate::marriage_alternatives::parallels::*;
use crate::{
    bcp1979::marriage::BCP_WEDDING_READINGS, conditions::EASTER_SEASON,
    rite2::LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL,
};

// Easy way to build the intro to the gospel readings, given the name of the book
// This is done automatically for [BiblicalReading] but not for [BiblicalCitation]
fn gospel_reading_intro(book_name: &str) -> BiblicalReadingIntro {
    BiblicalReadingIntro::from(Document::from(Choice::from(vec![
        Document::from(Preces::from([
            (
                "",
                format!("The Holy Gospel of our Lord Jesus Christ according to {book_name}."),
            ),
            ("People", "Glory to you, Lord Christ.".to_string()),
        ])),
        Document::from(Preces::from([
            (
                "",
                format!("The Holy Gospel of our Savior Jesus Christ according to {book_name}."),
            ),
            ("People", "Glory to you, Lord Christ.".to_string()),
        ])),
    ])))
}

lazy_static! {
    pub static ref ADDITIONAL_DIRECTIONS : Document = Document::from(Series::from(vec![
        Document::from(Rubric::from("If Banns are to be published, the following form is used")),
        Document::from("I publish the Banns of Marriage between *N.N.* of ____________ and *N.N.* of ___________. If any of you know just cause why they may not be joined together in Holy Matrimony, you are bidden to declare it. This is the first [*or* second, *or* third] time of asking."),
        Document::from(Rubric::from("The Celebration and Blessing of a Marriage may be used with any authorized liturgy for the Holy Eucharist. This service then replaces the Ministry of the Word, and the Eucharist begins with the Offertory.\n\nAfter the Declaration of Consent, if there is to be a giving in marriage, or presentation, the Celebrant asks,").long()),
        Document::from("Who presents [gives] these two people to be married to each other?"),
        Document::from(Rubric::from("or the following")),
        Document::from(Rubric::from("For the Ministry of the Word it is fitting that the couple to be married remain where they may conveniently hear the reading of Scripture. They may approach the Altar, either for the exchange of vows, or for the Blessing of the Marriage.\n\nIt is appropriate that all remain standing until the conclusion of the Collect. Seating may be provided for the wedding party, so that all may be seated for the Lessons and the homily.\n\nThe Apostles’ Creed may be recited after the Lessons, or after the homily, if there is one. When desired, some other suitable symbol of the vows may be used in place of the ring.\n\nAt the Offertory, it is desirable that the bread and wine be presented to the ministers by the newly married persons. They may then remain before the Lord’s Table and receive Holy Communion before other members of the congregation.").long())
    ]))
    .label("Additional Directions")
    .source(Reference {
        source: Source::LiturgicalResources1,
        page: 107
    })
    .version(Version::Expansive)
    .status(Status::TrialUse);

    // Variety of reading responses given for these liturgies
    static ref READING_RESPONSE: Document = Document::from(Choice::from(vec![
        Document::from(Preces::from([
            ("", "The Word of the Lord."),
            ("People", "Thanks be to God.")
        ])),
        Document::from(Preces::from([
            ("", "Hear what the Spirit is saying to God’s people."),
            ("People", "Thanks be to God.")
        ])),
        Document::from(Preces::from([
            ("", "Hear what the Spirit is saying to the Churches."),
            ("People", "Thanks be to God.")
        ]))
    ]));

    static ref LR1_OPENING: Document = Document::from(Series::from(vec![
            Document::from(Rubric::from("The Presider says the following, the People standing")),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("Presider", "Blessed be God: Father, Son, and Holy Spirit."),
                    ("People", "Blessed be God, now and for ever. Amen."),
                ])),
                Document::from(Preces::from([
                    ("Presider", "Blessed be the one, holy, and living God."),
                    ("People", "Glory to God for ever and ever."),
                ])),
            ])).condition(Condition::Not(Box::new(EASTER_SEASON.clone()))),
            Document::from(Rubric::from("From Easter Day through the Day of Pentecost")).display(Show::TemplateOnly),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("Presider", "Alleluia. Christ is risen."),
                    ("People", "The Lord is risen indeed. Alleluia."),
                ])),
                Document::from(Preces::from([
                    ("Presider", "Alleluia. Christ is risen."),
                    ("People", "Christ is risen indeed. Alleluia."),
                ])),
            ])).condition(EASTER_SEASON.clone()),

            Document::from(Rubric::from("Then may be said")),
            Document::from(Preces::from(vec![
                ("Presider", "Beloved, let us love one another,"),
                ("People", "For love is of God."),
                ("Presider", "Whoever does not love does not know God,"),
                ("People", "For God is love."),
                ("Presider", "Since God so loves us,"),
                ("People", "Let us love one another."),
            ]))
    ])).tags([OPENING_ACCLAMATION]);

    static ref LR1_COLLECT: Document = Document::from(Series::from(vec![
        Document::from(Heading::from((HeadingLevel::Heading3, "The Collect of the Day"))).tags([SALUTATION]),
        Document::from(Choice::from(vec![
            Document::from(Preces::from([
                ("Presider", "The Lord be with you."),
                ("People", "And also with you."),
                ("Presider", "Let us pray.")
            ])),
            Document::from(Preces::from([
                ("Presider", "God be with you."),
                ("People", "And also with you."),
                ("Presider", "Let us pray.")
            ]))
        ])).tags([SALUTATION]),
        Document::from(Choice::from(vec![
            Document::from(Text::from("God of abundance:\nassist by your grace *N.* and *N.*,\nwhose covenant of love and fidelity we witness this day.\nGrant them your protection, that with firm resolve\nthey may honor and keep the vows they make;\nthrough Jesus Christ our Savior,\nwho lives and reigns with you in the unity of the Holy Spirit,\none God, for ever and ever.").response("Amen")),
            Document::from(Text::from("Almighty and everliving God:\nlook tenderly upon *N.* and *N.*,\nwho stand before you in the company of your Church.\nLet their life together bring them great joy.\nGrant them so to love selflessly and live humbly,\nthat they may be to one another and to the world\na witness and a sign of your never-failing love and care;\nthrough Jesus Christ your Son our Lord,\nwho lives and reigns with you and the Holy Spirit,\none God, to the ages of ages.").response("Amen.")),
            Document::from(Text::from("O God, faithful and true,\nwhose steadfast love endures for ever:\nwe give you thanks for sustaining *N.* and *N.* in the life they share\nand for bringing them to this day.\nNurture them and fill them with joy in their life together,\ncontinuing the good work you have begun in them;\nand grant us, with them, a dwelling place eternal in the heavens\nwhere all your people will share the joy of perfect love,\nand where you, with the Son and the Holy Spirit, live and reign,\none God, now and for ever.").response("Amen.")),
            Document::from(Text::from("Holy Trinity, one God,\nthree Persons perfect in unity and equal in majesty:\nDraw together with bonds of love and affection\n*N.* and *N.*, who with *their families*\nseek to live in harmony and forbearance all their days,\nthat their joining together will be to us\na reflection of that perfect communion\nwhich is your very essence and life,\nO Father, Son, and Holy Spirit,\nwho live and reign in glory everlasting.").response("Amen."))
                .version_label("For Those Who Bring Children")
        ])).tags([COLLECT]),
    ]));

    static ref LR1_FIRST_READING: Document = Document::from(Series::from(vec![
        Document::from(Choice::from(vec![
            Document::from(BiblicalCitation::from("Ruth 1:16–17")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Ruth."))))
                .label("The First Lesson"),
            Document::from(BiblicalCitation::from("1 Samuel 18:1b, 3; 20:16–17; 42a")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Book of Samuel."))))
                .label("The First Lesson"),
            Document::from(BiblicalCitation::from("1 Samuel 18:1-4")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Book of Samuel."))))
                .label("The First Lesson"),
            Document::from(BiblicalCitation::from("Ecclesiastes 4:9-12")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Ecclesiastes."))))
                .label("The First Lesson"),
            Document::from(BiblicalCitation::from("Song of Solomon 2:10–13; 8:6–7")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Song of Songs."))))
                .label("The First Lesson"),
            Document::from(BiblicalCitation::from("Micah 4:1-4")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Micah."))))
                .label("The First Lesson"),
        ])),
        Document::from(Rubric::from("After the Reading, the Reader may say")),
        READING_RESPONSE.clone()
    ])).tags([FIRST_LESSON]);

    static ref LR1_PSALMS: Document = Document::from(Series::from(vec![
        Document::from(Rubric::from("Between the Readings, a psalm, hymn, or anthem may be sung or said. Appropriate psalms are Psalm 65, Psalm 67, Psalm 85:7–13, Psalm 98, Psalm 100, Psalm 126, Psalm 127, Psalm 133, Psalm 148, and Psalm 149:1–5.")),
        Document::from(Choice::from(vec![
            Document::from(PSALM_65.clone()),
            Document::from(PSALM_67.clone()),
            Document::from(PSALM_85.clone().citation("Psalm 85:7-13")),
            Document::from(PSALM_98.clone()),
            Document::from(PSALM_100.clone()),
            Document::from(PSALM_126.clone()),
            Document::from(PSALM_127.clone()),
            Document::from(PSALM_133.clone()),
            Document::from(PSALM_148.clone()),
            Document::from(PSALM_149.clone().citation("Psalm 149:1-5")),
        ])),
    ])).tags([PSALM]);

    static ref LR1_SECOND_READING: Document = Document::from(Series::from(vec![
        Document::from(Choice::from(vec![
            Document::from(BiblicalCitation::from("Romans 12:9–18")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Romans."))))
                .label("The Second Lesson"),
            Document::from(BiblicalCitation::from("1 Corinthians 12:31b–13:13")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter to the Corinthians."))))
                .label("The Second Lesson"),
            Document::from(BiblicalCitation::from("2 Corinthians 5:17–20")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Second Letter to the Corinthians."))))
                .label("The Second Lesson"),
            Document::from(BiblicalCitation::from("Galatians 5:14, 22–26")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Galatians."))))
                .label("The Second Lesson"),
            Document::from(BiblicalCitation::from("Ephesians 3:14–21")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Ephesians."))))
                .label("The Second Lesson"),
            Document::from(BiblicalCitation::from("Colossians 3:12–17")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Colossians."))))
                .label("The Second Lesson"),
            Document::from(BiblicalCitation::from("1 John 3:18–24")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter of John."))))
                .label("The Second Lesson"),
            Document::from(BiblicalCitation::from("1 John 4:7–16, 21")
                .intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter of John."))))
                .label("The Second Lesson"),
        ])),
        Document::from(Rubric::from("After the Reading, the Reader may say")),
        READING_RESPONSE.clone(),
    ])).tags([SECOND_LESSON]);

    static ref LR1_GOSPEL: Document = Document::from(Series::from(vec![
        Document::from(Rubric::from("All standing, the Deacon or Priest reads the Gospel, first saying")),
        Document::from(Choice::from(vec![
            Document::from(Preces::from([
                ("", "The Holy Gospel of our Lord Jesus Christ according to _____________."),
                ("People", "Glory to you, Lord Christ.")
            ])),
            Document::from(Preces::from([
                ("", "The Holy Gospel of our Savior Jesus Christ according to _____________."),
                ("People", "Glory to you, Lord Christ.")
            ])),
        ])).tags([GOSPEL]).display(Show::TemplateOnly),
        Document::from(Choice::from(vec![
            Document::from(BiblicalCitation::from("Matthew 5:1-16").intro(gospel_reading_intro("Matthew"))).label("The Gospel"),
            Document::from(BiblicalCitation::from("Mark 12:28–34").intro(gospel_reading_intro("Mark"))).label("The Gospel"),
            Document::from(BiblicalCitation::from("Luke 6:32-38").intro(gospel_reading_intro("Luke"))).label("The Gospel"),
            Document::from(BiblicalCitation::from("John 15:9–17").intro(gospel_reading_intro("John"))).label("The Gospel"),
            Document::from(BiblicalCitation::from("John 17:1–2, 18–26").intro(gospel_reading_intro("John"))).label("The Gospel"),
        ])),
        Document::from(Rubric::from("After the Gospel, the Reader says")),
        Document::from(Preces::from([
            ("", "The Gospel of the Lord."),
            ("People", "Praise to you, Lord Christ.")
        ]))
    ])).tags([GOSPEL]);

    static ref LR1_PRESENTATION_RINGS_VOWS: Document = Document::from(Series::from(vec![
        Document::from(Heading::from((HeadingLevel::Heading2, "The Witnessing of the Vows and the Blessing of the Covenant"))),
        Document::from(Rubric::from("The couple comes before the assembly. If there is to be a presentation, the presenters stand with the couple, and the Presider says to them")).tags([PRESENTATION_RUBRIC]),
        Document::from(Preces::from([
            ("Presider", "Who presents *N.* and *N.*, as they seek the blessing of God and the Church on their love and life together?"),
            ("Presenters", "We do."),
            ("Presider", "Will you love, respect, and pray for *N.* and *N.*, and do all in your power to stand with them in the life they will share?"),
            ("Presenters", "We will.")
        ])).tags([PRESENTATION]),
        Document::from(Series::from(vec![
            Document::from(Rubric::from("The Presider then addresses the couple, saying")),
            Document::from(Preces::from([
                ("Presider", "*N.* and *N.*, you have come before God and the Church to exchange [*and renew*] solemn vows with one another and to ask God’s blessing."),
            ])),
            Document::from(Rubric::from("The Presider addresses one member of the couple, saying")),
            Document::from(Preces::from([
                ("Presider", "*N.*, do you freely and unreservedly offer yourself to *N.*?"),
                ("Answer", "I do."),
                ("Presider", "Will you [*continue to*] live together in faithfulness and holiness of life as long as you both shall live?"),
                ("Answer", "I will, with God’s help."),
            ])),
            Document::from(Rubric::from("The Presider addresses the other member of the couple, saying")),
            Document::from(Preces::from([
                ("Presider", "*N.*, do you freely and unreservedly offer yourself to *N.*?"),
                ("Answer", "I do."),
                ("Presider", "Will you [*continue to*] live together in faithfulness and holiness of life as long as you both shall live?"),
                ("Answer", "I will, with God’s help."),
            ])),
        ])).tags([CONSENT]),
        Document::from(Series::from(vec![
            Document::from(Rubric::from("The assembly stands, the couple faces the People, and the Presider addresses them, saying")),
            Document::from(Preces::from([
                ("Presider", "Will all of you gathered to witness these vows do all in your power to uphold and honor this couple in the covenant they make?"),
                ("Answer", "We will."),
                ("Presider", "Will you pray for them, especially in times of trouble, and celebrate with them in times of joy?"),
                ("Answer", "We will."),
            ])),
        ])).tags([CONSENT_CONGREGATION])
    ]));

    static ref LR1_PRAYERS: Document = Document::from(Series::from(vec![
        Document::from(Heading::from((HeadingLevel::Heading2, "The Prayers"))).tags([THE_PRAYERS_HEADER]),
        Document::from(Series::from(vec![
            Document::from(Rubric::from("The Presider then introduces the prayers")),
            Document::from(Preces::from([
                ("Presider", "Then let us pray for *N.* and *N.* in their life together and for the concerns of this community."),
            ])),
            Document::from(Rubric::from("A Deacon or another leader bids prayers for the couple.\n\nPrayers for the Church and for the world, for the concerns of the local community, for those who suffer or face trouble, and for the departed are also appropriate. If the rite takes place in the principal Sunday worship of the congregation, the rubric concerning the Prayers of the People on page 359 of the Book of Common Prayer is followed.\n\nAdaptations or insertions may be made to the form that follows.\n\nA bar in the margin indicates a bidding that may be omitted.")),
            Document::from(Series::from(vec![
                Document::from(Litany::from((
                    "Hear our prayer.",
                    [
                        "For *N.* and *N.*, seeking your blessing and the blessing of your holy people;\nLoving God	*or*	Lord, in your mercy,",
                        "For a spirit of loving-kindness to shelter them all their days;\nLoving God	*or*	Lord, in your mercy,",
                        "For friends to support them and communities to enfold them;\nLoving God	*or*	Lord, in your mercy,",
                        "For peace in their home and love in their family;\nLoving God	*or*	Lord, in your mercy,",
                        "| For the grace and wisdom to care for the children you entrust to them [*or* may entrust to them];\nLoving God	*or*	Lord, in your mercy,",
                        "For the honesty to acknowledge when they hurt each other, and the humility to seek each other’s forgiveness and yours;\nLoving God	*or*	Lord, in your mercy,",
                        "For the outpouring of your love through their work and witness;\nLoving God	*or*	Lord, in your mercy,",
                        "For the strength to keep the vows each of us has made;\nLoving God	*or*	Lord, in your mercy,",
                    ]
                ))),
                Document::from(Rubric::from("The leader may add one or more of the following biddings")),
                Document::from(Litany::from((
                    "Hear our prayer.",
                    [
                        "| For all who have been reborn and made new in the waters of Baptism;\nLoving God	*or*	Lord, in your mercy,",
                        "| For those who lead and serve in communities of faith;\nLoving God	*or*	Lord, in your mercy,",
                        "| For those who seek justice, peace, and concord among nations;\nLoving God",
                        "| For those who are sick or suffering, homeless or poor;\nLoving God	*or*	Lord, in your mercy,",
                        "| For victims of violence and those who inflict it;\nLoving God	*or*	Lord, in your mercy,",
                        "| For communion with all who have died, [especially those whom we remember this day:	___________];\nLoving God	*or*	Lord, in your mercy,"
                    ]
                ))),
            ])),
            Document::from(Rubric::from("The Presider concludes the Prayers with the following or another appropriate Collect")),
            Document::from(Text::from("Giver of every gift, source of all goodness,\nhear the prayers we bring before you for N. and N., \nwho seek your blessing this day.\nStrengthen them as they share in the saving work of Jesus, \nand bring about for them and for all you have created\nthe fullness of life he promised,\nwho now lives and reigns for ever and ever.").response("Amen.")),
        ])).tags([THE_PRAYERS]),

        Document::from(Series::from(vec![
            Document::from(Rubric::from("If the Eucharist is to follow, the Lord’s Prayer is omitted here.")),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("Leader", "As our Savior Christ\nhas taught us,\nwe now pray,\n"),
                    ("People\nand Leader", "Our Father in heaven,\nhallowed be your Name,\n	your kingdom come,\n	your will be done,\n	on earth as in heaven.\nGive us today our daily bread.\nForgive us our sins\n	as we forgive those\n	who sin against us.\nSave us from the time of trial,\n	and deliver us from evil.\nFor the kingdom, the power,\n	and the glory are yours,\n	now and for ever. Amen.\n")
                ]))
                .version_label("Contemporary"),
                Document::from(Preces::from([
                    ("Leader", "And now, as our Savior\nChrist has taught us,\nwe are bold to say,"),
                    ("People\nand Leader", "Our Father, who art in heaven,\nhallowed be thy Name,\n	thy kingdom come,\n	thy will be done,\n	on earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n	as we forgive those \n	who trespass against us.\nAnd lead us not into temptation,\n	but deliver us from evil.\nFor thine is the kingdom,\n	and the power, and the glory,\n	for ever and ever. Amen.\n")
                ]))
                .version_label("Traditional")
            ])),
        ])).tags([LORDS_PRAYER])
    ]));

    static ref LR1_COMMITMENT: Document = Document::from(Series::from(vec![
        Document::from(Heading::from((HeadingLevel::Heading2, "The Commitment"))).tags([THE_MARRIAGE_HEADER]),
        Document::from(Series::from(vec![
            Document::from(Rubric::from("The People sit. The couple stands, facing the Presider.")),
            Document::from(Preces::from([
                ("Presider", "*N.* and *N.*, I invite you now, illumined by the Word of God and strengthened by the prayer of this community, to make your covenant before God and the Church.")
            ])),
            Document::from(Rubric::from("Each member of the couple, in turn, takes the right hand of the other and says")),
            Document::from(Choice::from(vec![
                Document::from("In the name of God,\nI, N., give myself to you, N., and take you to myself.\nI will support and care for you by the grace of God:\nin times of sickness, in times of health.\nI will hold and cherish you in the love of Christ:\nin times of plenty, in times of want.\nI will honor and keep you with the Spirit’s help:\nin times of anguish, in times of joy,\nforsaking all others, as long as we both shall live.\nThis is my solemn vow."),
                Document::from("In the name of God,\nI, N., give myself to you, N., and take you to myself.\nI will support and care for you:\nin times of sickness, in times of health.\nI will hold and cherish you:\nin times of plenty, in times of want.\nI will honor and love you:\nin times of anguish, in times of joy,\nforsaking all others, as long as we both shall live.\nThis is my solemn vow.")
            ])),
        ])).tags([THE_MARRIAGE]),
        Document::from(Choice::from(vec![
            Document::from(Series::from(vec![
                Document::from(Rubric::from("If rings are to be exchanged, they are brought before the Presider, who prays using the following words")),
                Document::from(Text::from("Let us pray.\n\nBless, O God, these rings\nas signs of the enduring covenant\n*N.* and *N.* have made with each other,\nthrough Jesus Christ our Lord.").response("Amen.")),
                Document::from(Rubric::from("The two people place the rings on the fingers of one another, first the one, then the other, saying")),
                Document::from(Choice::from(vec![
                    Document::from("*N.*, I give you this ring as a symbol of my vow,\nand with all that I am, and all that I have, I honor you,\nin the name of God."),
                    Document::from("*N.*, I give you this ring as a symbol of my vow,\nand with all that I am, and all that I have, I honor you,\nin the name of the Father, and of the Son,\nand of the Holy Spirit.")
                ])),
            ]))
            .version_label("Rings to be Given"),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("If the two have previously given and worn rings as a symbol of their commitment, the rings may be blessed on the hands of the couple, the Presider saying")),
                Document::from(Text::from("Let us pray.\n\nBy the rings which they have worn, faithful God,\n*N.* and *N.* have shown to one another and the world\ntheir love and faithfulness.\nBless now these rings,\nthat from this day forward\nthey may be signs of the vows *N.* and *N.* have exchanged\nin your presence and in the communion of your Church,\nthrough Christ our Lord.").response("Amen."))
            ]))
            .version_label("Rings Already Worn")
        ])).tags([THE_RINGS])
    ]));

    static ref LR1_MARRIAGE: Document = Document::from(Series::from(vec![
        Document::from(Heading::from((HeadingLevel::Heading2, "The Marriage"))).tags([THE_MARRIAGE_HEADER]),
        Document::from(Series::from(vec![
            Document::from(Rubric::from("The People sit. The couple stands, facing the Presider.")),
            Document::from(Preces::from([
                ("Presider", "*N.* and *N.*, I invite you now, illumined by the Word of God and strengthened by the prayer of this community, to make your covenant before God and the Church.")
            ])),
            Document::from(Rubric::from("Each member of the couple, in turn, takes the right hand of the other and says")),
            Document::from(Choice::from(vec![
                Document::from("In the name of God,\nI, N., give myself to you, N., and take you to myself.\nI will support and care for you by the grace of God:\nin times of sickness, in times of health.\nI will hold and cherish you in the love of Christ:\nin times of plenty, in times of want.\nI will honor and love you with the Spirit’s help:\nin times of anguish, in times of joy,\nforsaking all others, as long as we both shall live.\nThis is my solemn vow."),
                Document::from("In the name of God,\nI, N., give myself to you, N., and take you to myself.\nI will support and care for you:\nin times of sickness, in times of health.\nI will hold and cherish you:\nin times of plenty, in times of want.\nI will honor and love you:\nin times of anguish, in times of joy,\nforsaking all others, as long as we both shall live.\nThis is my solemn vow.")
            ])),
        ])).tags([THE_MARRIAGE]),
        Document::from(Series::from(vec![
            Document::from(Rubric::from("If rings are to be exchanged, they are brought before the Presider, who prays using the following words")),
            Document::from(Text::from("Let us pray.\n \nBless, O God, these rings\nas signs of the enduring covenant\n*N.* and *N.* have made with each other,\nthrough Jesus Christ our Lord.").response("Amen.")),
            Document::from(Rubric::from("The two people place the rings on the fingers of one another, first the one, then the other, saying")),
            Document::from(Choice::from(vec![
                Document::from("*N.*, I give you this ring as a symbol of my vow,\nand with all that I am, and all that I have, I honor you,\nin the name of God."),
                Document::from("*N.*, I give you this ring as a symbol of my vow,\nand with all that I am, and all that I have, I honor you,\nin the name of the Father, and of the Son,\nand of the Holy Spirit.")
            ])),
        ]))
            .tags([THE_RINGS])
            .version_label("Rings to be Given"),
        Document::from(Series::from(vec![
            Document::from(Rubric::from("If the two have previously given and worn rings as a symbol of their commitment, the rings may be blessed on the hands of the couple, the Presider saying")),
            Document::from(Text::from("Let us pray.\n \nBy the rings which they have worn, faithful God,\n*N.* and *N.* have shown to one another and the world\ntheir love and faithfulness.\nBless now these rings,\nthat from this day forward\nthey may be signs of the vows *N.* and *N.* have exchanged\nin your presence and in the communion of your Church,\nthrough Christ our Lord.").response("Amen."))
        ]))
        .version_label("Rings Already Worn")
        .tags([RINGS_ALREADY_GIVEN])
    ]));

    static ref LR1_BLESSING_AND_EUCHARIST: Document = Document::from(Series::from(vec![
        Document::from(Heading::from((HeadingLevel::Heading3, "Blessing of the Couple"))).tags([BLESSING_OF_THE_MARRIAGE]),
        Document::from(Rubric::from("As the couple stands or kneels, the Presider invokes God’s blessing upon them, saying")).tags([BLESSING_RUBRIC]),
        Document::from("Let us pray.\n \nMost gracious God,\nwe praise you for the tender mercy and unfailing care revealed to us in Jesus the Christ\nand for the great joy and comfort bestowed upon us\nin the gift of human love.\nWe give you thanks for *N.* and *N.*,\nand the covenant of faithfulness they have made.\nPour out the abundance of your Holy Spirit upon them.\nKeep them in your steadfast love;\nprotect them from all danger;\nfill them with your wisdom and peace;\nlead them in holy service to each other and the world.").tags([BLESSING_PRAYERS]),
        Document::from(Rubric::from("The Presider continues with one of the following")).tags([BLESSING_PROPER]),
        Document::from(Choice::from(vec![
            Document::from(Text::from("God the Father,\nGod the Son,\nGod the Holy Spirit,\nbless, preserve, and keep you,\nand mercifully grant you rich and boundless grace,\nthat you may please God in body and soul.\nGod make you a sign of the loving-kindness and steadfast fidelity\nmanifest in the life, death, and resurrection of our Savior,\nand bring you at last to the delight of the heavenly banquet,\nwhere he lives and reigns for ever and ever.").response("Amen.")),
            Document::from(Text::from("God, the holy and undivided Trinity, \nbless, preserve, and keep you,\nand mercifully grant you rich and boundless grace, \nthat you may please God in body and soul.\nGod make you a sign of the loving-kindness and steadfast fidelity \nmanifest in the life, death, and resurrection of our Savior,\nand bring you at last to the delight of the heavenly banquet, \nwhere he lives and reigns for ever and ever.").response("Amen.")),
        ])).tags([BLESSING_PROPER]),

        Document::from(Heading::from((HeadingLevel::Heading3, "The Peace"))).tags([PEACE_HEADER]),
        Document::from(Rubric::from("The Presider bids the Peace.")).tags([THE_PEACE]),
        Document::from(Choice::from(vec![
            Document::from(Preces::from([
                ("Presider", "The peace of the Lord be always with you."),
                ("People", "And also with you.")
            ])),
            Document::from(Preces::from([
                ("Presider", "The peace of Christ be always with you."),
                ("People", "And also with you.")
            ]))
        ])).tags([THE_PEACE]),

        Document::from(Rubric::from("The liturgy continues with the Holy Communion. When the Eucharist is not celebrated, the Presider blesses the people. The Deacon, or in the absence of a Deacon, the Priest, dismisses them.")).tags([POST_PEACE_RUBRICS]),

        // At the Eucharist
        Document::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading2, "At the Eucharist"))).tags([AT_THE_EUCHARIST]),
            Document::from(Rubric::from("The liturgy continues with the Offertory, at which the couple may present the offerings of bread and wine.")).tags([OFFERTORY_RUBRIC]),
            Document::from(Rubric::from("The following proper preface may be said.")).tags([PROPER_PREFACE]),
            Document::from("Because in the giving of two people to each other in faithful love \nyou reveal the joy and abundant life you share \nwith your Son Jesus Christ and the Holy Spirit.").tags([PROPER_PREFACE]),
            Document::from(Rubric::from("The following postcommunion prayer may be said.")).tags([POSTCOMMUNION_PRAYER]),
            Document::from(Text::from("God our strength and joy,\nwe thank you for the communion of our life together, \nfor the example of holy love that you give us in *N.* and *N.*, \nand for the Sacrament of the Body and Blood\nof our Savior Jesus Christ.\nGrant that it may renew our hope\nand nourish us for the work you set before us\nto witness to the presence of Christ in the world, \nthrough the power of your Spirit,\nand to the glory of your Name.").response("Amen.").display_format(DisplayFormat::Unison)).tags(["Postcommunion Prayer", POSTCOMMUNION_PRAYER])
        ]))
    ]));

    pub static ref WITNESSING_AND_BLESSING_OF_A_LIFELONG_COVENANT: Document = Document::new()
        .label("The Witnessing and Blessing of a Lifelong Covenant")
        .version(Version::Expansive)
        .status(Status::TrialUse)
        .source(Reference {
            source: Source::LiturgicalResources1,
            page: 79
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Witnessing and Blessing\nof a Lifelong Covenant"))).tags(["Title"]),

            // Concerning the Service
            Document::from(Heading::from((HeadingLevel::Heading3, "Concerning the Service"))),
            Document::from(Rubric::from("This rite is appropriately celebrated in the context of the Holy Eucharist and may take place at the principal Sunday Liturgy. This rite then replaces the Ministry of the Word. A bishop or priest normally presides. Parallel texts from Enriching Our Worship 1 are included as options for elements of this rite.\n\nAt least one of the couple must be a baptized Christian.\n\nTwo or more presenters, who may be friends, parents, family members, or drawn from the local assembly, may present the couple to the presider and the assembly.\n\nAs indicated in the opening address, the consent, and the blessing of the rings, the rite may be modified for use with a couple who have previously made a lifelong commitment to one another.").long()),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Word of God")))
                .source(Reference {
                    source: Source::LiturgicalResources1,
                    page: 80
                }),
            Document::from(Heading::from((HeadingLevel::Heading3, "Gathering"))),
            Document::from(Rubric::from("The couple to be blessed joins the assembly.")),
            Document::from(Rubric::from("A hymn of praise, psalm, or anthem may be sung, or instrumental music may be played.")),
            Document::from(HymnLink::Tag("Marriage".into())),

            LR1_OPENING.clone(),

            Document::from(Rubric::from("The Presider may address the assembly in these words")),
            Document::from(Choice::from(vec![
                Document::from("Dear friends in Christ, [*or* Dearly beloved], \nin the name of God and the Church\nwe have come together today with *N. N.* and *N. N.*,\nto witness the vows they make, committing themselves to one another. Forsaking all others,\nthey will bind themselves to one another\nin a covenant of mutual fidelity and steadfast love, remaining true to one another in heart, body, and mind, as long as they both shall live.\nSuch a lifelong commitment\nis not to be entered into lightly or thoughtlessly, but responsibly and with reverence.\nLet us pray, then, that God will give them the strength to remain steadfast in what they vow this day.\nLet us also pray for the generosity to support them in the commitment they undertake,\nand for the wisdom to see God at work in their life together."),
                Document::from("Dear friends in Christ [*or* Dearly beloved], \nin the name of God and the Church,\nwe have come together with *N. N.* and *N. N.*, to witness the sacred vows they make\nas they solemnize [or reaffirm] their commitment to one another. Today they renew their covenant of mutual fidelity and steadfast love,\nforsaking all others and remaining true to one another in heart, body, and mind, as long as they both shall live.\nLet us pray, then, that God will give them the strength to remain steadfast in what they vow this day.\nLet us also pray for the generosity\nto support them in the commitment they undertake,\nand for the wisdom to see God at work in their life together.")
                    .version_label("For those who have previously made a lifelong commitment to one another")
            ])),

            LR1_COLLECT.clone(),

            // The Readings
            Document::from(Heading::from((HeadingLevel::Heading2, "The Lessons"))),
            Document::from(Rubric::from("The people sit. Then one or more of the following passages of Scripture is read. If the Holy Communion is to be celebrated, a passage from the Gospels always concludes the Readings. When the blessing is celebrated in the context of the Sunday Eucharist, the Readings of the Sunday are used, except with the permission of the Bishop.")),

            // First Lesson
            LR1_FIRST_READING.clone(),

            // Psalm
            LR1_PSALMS.clone(),

            // Second Lesson
            LR1_SECOND_READING.clone(),

            // Gospel
            LR1_GOSPEL.clone(),

            // Sermon
            Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon"))),

            // Vows
            LR1_PRESENTATION_RINGS_VOWS.clone(),

            // Prayers
            LR1_PRAYERS.clone(),

            // The Commitment
            LR1_COMMITMENT.clone(),

            Document::from(Heading::from((HeadingLevel::Heading3, "Pronouncement"))).tags([PRONOUNCEMENT_HEADER]),
            Document::from(Rubric::from("The Presider joins the right hands and says")).tags([PRONOUNCEMENT]),
            Document::from(Text::from("Now that N. and N. have exchanged vows of love and fidelity\nin the presence of God and the Church,\nI now pronounce that they are bound to one another\nas long as they both shall live.").response("Amen.")).tags([PRONOUNCEMENT]),

            LR1_BLESSING_AND_EUCHARIST.clone()
    ]))));

    pub static ref WITNESSING_AND_BLESSING_OF_A_MARRIAGE: Document = Document::new()
        .label("The Witnessing and Blessing of a Marriage")
        .version(Version::Expansive)
        .status(Status::TrialUse)
        .source(Reference {
            source: Source::LiturgicalResources1,
            page: 89
        })
        .alternate_source(Reference {
            source: Source::LiturgicalResources2,
            page: 14
        })
        .explainer("An alternative marriage service approved for trial use in 2012, designed for couples of any gender and shaped by a desire to include the experiences of same-sex couples in the marriage rite.")
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Witnessing and Blessing\nof a Marriage"))).tags(["Title"]).tags([TITLE]),

            // Concerning the Service
            Document::from(Heading::from((HeadingLevel::Heading3, "Concerning the Service"))),
            Document::from(Rubric::from("This rite is appropriately celebrated in the context of the Holy Eucharist and may take place at the principal Sunday Liturgy. This rite then replaces the Ministry of the Word. A bishop or priest normally presides. Parallel texts from Enriching Our Worship 1 are included as options for elements of this rite.\n\nAt least one of the couple must be a baptized Christian, and the marriage shall conform to the laws of the state and canons of this church.\n\nTwo or more presenters, who may be friends, parents, family members, or drawn from the local assembly, may present the couple to the presider and the assembly.\n\nAs indicated in the opening address, the consent, and the blessing of the rings, the rite may be modified for use with a couple who have previously made a lifelong commitment to one another.").long()),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Word of God")))
                    .source(Reference {
                        source: Source::LiturgicalResources1,
                        page: 90
                    }),
                Document::from(Heading::from((HeadingLevel::Heading3, "Gathering")))
            ])).tags([WORD_OF_GOD_HEADING]),

            Document::from(Rubric::from("The couple joins the assembly.")).tags([PROCESSION_RUBRIC]),
            Document::from(Rubric::from("A hymn of praise, psalm, or anthem may be sung, or instrumental music may be played.")).tags([PROCESSION_MUSIC_RUBRIC]),
            Document::from(HymnLink::Tag("Marriage".into())).tags([OPENING_HYMN]),

            LR1_OPENING.clone(),

            Document::from(Rubric::from("The Presider may address the assembly in these words")).tags([OPENING_ADDRESS_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from("Dear friends in Christ,	[*or* Dearly beloved], \nin the name of God and the Church\n\nwe have come together today with N. N. and N. N.,\n\nto witness the vows they make, committing themselves to one another\n\nin marriage [according to the laws of the state [or civil jurisdiction of X].\n\nForsaking all others,\n\nthey will bind themselves to one another\n\nin a covenant of mutual fidelity and steadfast love,\n\nremaining true to one another in heart, body, and mind,\n\nas long as they both shall live.\n\n\n\nThe lifelong commitment of marriage\n\nis not to be entered into lightly or thoughtlessly, but responsibly and with reverence.\n\nLet us pray, then, that God will give them the strength to remain steadfast in what they vow this day.\n\nLet us also pray for the generosity\n\nto support them in the commitment they undertake\n\nand for the wisdom to see God at work in their life together."),
                Document::from("Dear friends in Christ [*or* Dearly beloved], \nin the name of God and the Church\n\nwe have come together today with *N. N.* and *N. N.*\n\nto witness the sacred vows they make this day\n\nas they are married\n\n[according to the laws of the state or civil jurisdiction of X],\n\nand reaffirm their commitment to one another.\n\nForsaking all others,\n\nthey will renew their covenant of mutual fidelity and steadfast love,\n\nremaining true to one another in heart, body, and mind,\n\nas long as they both shall live.\n\n\n\nLet us pray, then, that God will give them the strength\n\nto remain steadfast in what they vow this day.\n\nLet us also pray for the generosity\n\nto support them in the commitment they undertake,\n\nand for the wisdom to see God at work in their life together.")
                    .version_label("For those who have previously made a lifelong commitment to one another")
            ])).tags([OPENING_ADDRESS]),

            LR1_COLLECT.clone(),

            // The Readings
            Document::from(Heading::from((HeadingLevel::Heading2, "The Lessons"))).tags([SCRIPTURE_RUBRIC]),
            Document::from(Rubric::from("The people sit. Then one or more of the following passages of Scripture is read. If the Holy Communion is to be celebrated, a passage from the Gospels always concludes the Readings. When the blessing is celebrated in the context of the Sunday Eucharist, the Readings of the Sunday are used, except with the permission of the Bishop.")).tags([SCRIPTURE_RUBRIC]),
            LR1_FIRST_READING.clone(),
            LR1_PSALMS.clone(),
            LR1_SECOND_READING.clone(),

            LR1_GOSPEL.clone(),

            // Sermon
            Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon"))).tags([HOMILY]),

            // Vows
            LR1_PRESENTATION_RINGS_VOWS.clone(),

            // Prayers
            LR1_PRAYERS.clone(),

            // The Marriage
            LR1_MARRIAGE.clone(),

            Document::from(Heading::from((HeadingLevel::Heading3, "Pronouncement"))).tags([PRONOUNCEMENT_HEADER]),
            Document::from(Rubric::from("The Presider joins the right hands of the couple and says")).tags([PRONOUNCEMENT]),
            Document::from(Text::from("Now that *N.* and *N.* have exchanged vows of love and fidelity \nin the presence of God and the Church,\nI pronounce that they are\nmarried [according to the laws of the state or civil jurisdiction of *X*].\nand bound to one another\nas long as they both shall live.").response("Amen.")).tags([PRONOUNCEMENT]),

            LR1_BLESSING_AND_EUCHARIST.clone()
        ]))));

    pub static ref CELEBRATION_AND_BLESSING_OF_A_MARRIAGE_2: Document = Document::new()
        .label("Celebration and Blessing of a Marriage (2)")
        .version(Version::Expansive)
        .status(Status::TrialUse)
        .source(Reference {
            source: Source::LiturgicalResources1,
            page: 99
        })
        .alternate_source(Reference {
            source: Source::LiturgicalResources2,
            page: 7
        })
        .explainer("A gender-neutral adaptation of the 1979 marriage service, approved for trial use in 2015/2018.")
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Celebration and Blessing \nof a Marriage (2)"))).tags([TITLE]),
            Document::from(Rubric::from("At the time appointed, the persons to be married, with their witnesses, assemble in the church or some other appropriate place.")).tags([PROCESSION_RUBRIC]),
            Document::from(Rubric::from("During their entrance, a hymn, psalm, or anthem may be sung, or instrumental music may be played.")).tags([PROCESSION_MUSIC_RUBRIC]),
            Document::from(HymnLink::Tag("Marriage".into())).tags([OPENING_HYMN]),

            // Opening Words
            Document::from(Rubric::from("Then the Celebrant, facing the people and the persons to be married, addresses the congregation and says")).tags([OPENING_ADDRESS_RUBRIC]),
            Document::from("Dearly beloved: We have come together in the presence of God to witness and bless the joining together of *N.* and *N.* in Holy Matrimony. The joining of two people in a life of mutual fidelity signifies to us the mystery of the union between Christ and his Church, and so it is worthy of being honored among all people.\n\nThe union of two people in heart, body, and mind is intended by God for their mutual joy; for the help and comfort given one another in prosperity and adversity; and, when it is God’s will, for the gift of children and their nurture in the knowledge and love of the Lord. Therefore marriage is not to be entered into unadvisedly or lightly, but reverently, deliberately, and in accordance with the purposes for which it was instituted by God.\n\nInto this holy union *N. N.* and *N. N.* now come to be joined.\n\nIf any of you can show just cause why they may not lawfully be married, speak now; or else for ever hold your peace.").tags([OPENING_ADDRESS]),
            Document::from(Rubric::from("Then the Celebrant says to the persons to be married")).tags([OPENING_WORDS_TO_COUPLE_RUBRIC]),
            Document::from("I require and charge you both, here in the presence of God, that if either of you knows any reason why you may not be united in marriage lawfully, and in accordance with God’s Word, you do now confess it.").tags([OPENING_WORDS_TO_COUPLE]),

            // Declaration of Consent
            Document::from(Heading::from((HeadingLevel::Heading2, "The Declaration of Consent"))).tags([CONSENT_HEADER]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant says to one member of the couple, then to the other")),
                Document::from(Preces::from([
                    ("", "*N.*, will you have this *woman/man/person* to be your *wife/husband/spouse*; to live together in the covenant of marriage? Will you love *her/him*, comfort *her/him*, honor and keep *her/him*, in sickness and in health; and, forsaking all others, be faithful to *her/him* as long as you both shall live?"),
                    ("Answer", "I will.")
                ])),
            ])).tags([CONSENT]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant then addresses the congregation, saying")),
                Document::from("Will all of you witnessing these promises do all in your power to uphold these two persons in their marriage?"),
                Document::from(Preces::from([
                    ("", ""),
                    ("People", "We will.")
                ])),
            ])).tags([CONSENT_CONGREGATION]),

            Document::from(Rubric::from("If there is to be a presentation or a giving in marriage, it takes place at this time.")).tags([PRESENTATION_RUBRIC]),
            Document::from(Content::DocumentLink {
                label: "Additional Directions".into(),
                path: SlugPath::from([Slug::Marriage, Slug::AdditionalDirections, Slug::Version(Version::Expansive)]),
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
                    ("People", "And also with you."),
                    ("Celebrant", "Let us pray.")
                ])),
            ])).tags([SALUTATION]),

            Document::from(Text::from("O gracious and everliving God, you have created humankind in your image: Look mercifully upon *N.* and *N.* who come to you seeking your blessing, and assist them with your grace, that with true fidelity and steadfast love they may honor and keep the promises and vows they make; through Jesus Christ our Savior, who lives and reigns with you in the unity of the Holy Spirit, one God, for ever and ever.").response("Amen.")).tags([COLLECT]),

            // Readings
            Document::from(Rubric::from("Then one or more of the following passages from Holy Scripture is read. Other readings from Scripture suitable for the occasion may be used. If there is to be a Communion, a passage from the Gospel always concludes the Readings.")),

            BCP_WEDDING_READINGS.clone(),

            Document::from(Rubric::from("A homily or other response to the Readings may follow.")).tags([HOMILY]),

            // The Marriage
            Document::from(Heading::from((HeadingLevel::Heading2, "The Marriage"))).tags([THE_MARRIAGE_HEADER]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("Each member of the couple, in turn, takes the right hand of the other and says")),
                Document::from("In the Name of God, I, *N.*, take you, *N.*, to be my *wife/husband/spouse*, \nto have and to hold from this day forward,\nfor better for worse, for richer for poorer,\nin sickness and in health, to love and to cherish, \nuntil we are parted by death.\nThis is my solemn vow."),
            ])).tags([THE_MARRIAGE]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Priest may ask God’s blessing on rings as follows")),
                Document::from(Text::from("Bless, O Lord, these rings to be signs of the vows\nby which *N.* and *N.* have bound themselves to each other; \nthrough Jesus Christ our Lord.").response("Amen.")),
                Document::from(Rubric::from("The giver places the ring on the ring finger of the other’s hand and says")),
                Document::from("*N.*, I give you this ring as a symbol of my vow,\nand with all that I am, and all that I have, I honor you, in the Name of the Father, and of the Son,\nand of the Holy Spirit [*or* in the Name of God]."),
            ])).tags([THE_RINGS]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("Then the Celebrant joins the right hands of the couple and says")),
                Document::from("Now that *N.* and *N.* have given themselves to each other by solemn vows, \nwith the joining of hands and the giving and receiving of rings,\nI pronounce that they are wed to one another,\nin the Name of the Father, and of the Son, and of the Holy Spirit.\n\nThose whom God has joined together let no one put asunder."),
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
                        "Let us pray.\n\nEternal God, creator and preserver of all life, author of salvation, and giver of all grace: Look with favor upon the world you have made, and for which your Son gave his life, and especially upon *N.* and *N.* whom you make one flesh in Holy Matrimony.",
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
            Document::from(Rubric::from("The People remain standing. The couple kneel, and the Priest says one of the following prayers")).tags([BLESSING_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Text::from("Most gracious God, we give you thanks for your tender love in sending Jesus Christ to come among us, to be born of a human mother, and to make the way of the cross to be the way of life. We thank you, also, for consecrating the union of two people in his Name. By the power of your Holy Spirit, pour out the abundance of your blessing upon *N.* and *N.* Defend them from every enemy. Lead them into all peace. Let their love for each other be a seal upon their hearts, a mantle about their shoulders, and a crown upon their foreheads. Bless them in their work and in their companionship; in their sleeping and in their waking; in their joys and in their sorrows; in their life and in their death. Finally, in your mercy, bring them to that table where your saints feast for ever in your heavenly home; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.").response("Amen.")),
                Document::from(Text::from("O God, you have so consecrated the covenant of marriage that in it is represented the spiritual unity between Christ and his Church: Send therefore your blessing upon these your servants, that they may so love, honor, and cherish each other in faithfulness and patience, in wisdom and true godliness, that their home may be a haven of blessing and peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.").response("Amen."))
            ])).tags([BLESSING_PRAYERS]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The couple still kneeling, the Priest adds this blessing")),
                Document::from(Text::from("God the Father, God the Son, God the Holy Spirit, bless, preserve, and keep you; the Lord mercifully with his favor look upon you, and fill you with all spiritual benediction and grace; that you may faithfully live together in this life, and in the age to come have life everlasting.").response("Amen.")),
            ])).tags([BLESSING_PROPER]),

            // Peace and Dismissal
            Document::from(Heading::from((HeadingLevel::Heading3, "The Peace"))).tags([PEACE_HEADER]),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant may say to the people")),
                Document::from(Preces::from([
                    ("", "The peace of the Lord be always with you."),
                    ("People", "And also with you.")
                ]))
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
                Document::from(Text::from("O God, the giver of all that is true and lovely and gracious:\nWe give you thanks for binding us together\nin these holy mysteries of the Body and Blood \nof your Son Jesus Christ.\nGrant that by your Holy Spirit,\n*N.* and *N.*, now joined in Holy Matrimony, \nmay become one in heart and soul,\nlive in fidelity and peace,\nand obtain those eternal joys prepared for all who love you; \nfor the sake of Jesus Christ our Lord.").response("Amen.").display_format(DisplayFormat::Unison)).tags(["Postcommunion Prayer", POSTCOMMUNION_PRAYER]),
                Document::from(Rubric::from("As the wedding party leaves the church, a hymn, psalm, or anthem may be sung; or instrumental music may be played.")).tags([CLOSING_HYMN]),
                Document::from(HymnLink::Tag("Marriage".into())).tags([CLOSING_HYMN]),
            ])).display(Show::TemplateOnly)
    ]))));

    pub static ref BLESSING_OF_A_CIVIL_MARRIAGE: Document = Document::new()
        .label("Blessing of a Civil Marriage")
        .version(Version::Expansive)
        .status(Status::TrialUse)
        .source(Reference {
            source: Source::LiturgicalResources1,
            page: 105
        })
        .alternate_source(Reference {
            source: Source::LiturgicalResources2,
            page: 25
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Blessing\nof a Civil Marriage"))),
            Document::from(Rubric::from("The rite begins as prescribed for celebrations of the Holy Eucharist, using the Collect and Lessons appointed in the Marriage service.")),
            Document::from(Content::DocumentLink {
                label: "Holy Eucharist".into(),
                path: SlugPath::from([Slug::Eucharist]),
                rotate: false,
                link_only: false
            }),
            Document::from(Content::DocumentLink {
                label: "Marriage Service".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing, Slug::Version(Version::Expansive)]),
                rotate: false,
                link_only: false
            }),

            // Consent
            Document::from(Rubric::from("After the Gospel (and homily), the couple stand before the Celebrant, who addresses them in these or similar words")),
            Document::from("*N.* and *N.*, you have come here today to seek the blessing of God and of his Church upon your marriage. I require, therefore, that you promise, with the help of God, to fulfill the obligations which Christian Marriage demands."),

            Document::from(Rubric::from("The Celebrant then addresses one member of the couple, then the other, saying")),
            Document::from("*N.*, you have taken *N.* to be your *wife/husband/spouse*. Do you promise to love *her/him*, comfort *her/him*, honor and keep *her/him*, in sickness and in health, and, forsaking all others, to be faithful to *her/him* as long as you both shall live?"),
            Document::from(Preces::from([
                ("Answer", "I do.")
            ])),

            Document::from(Rubric::from("The Celebrant then addresses the congregation, saying")),
            Document::from("Will you who have witnessed these promises do all in your power to uphold these two persons in their marriage?"),
            Document::from(Preces::from([
                ("", ""),
                ("People", "We will.")
            ])),

            Document::from(Rubric::from("If rings are to be blessed, the members of the couple extend their hands toward the Priest [or Bishop], who says")),
            Document::from(Text::from("Bless, O Lord, these rings to be signs of the vows by which *N.* and *N.* have bound themselves to each other; through Jesus Christ our Lord").response("Amen.")),
            Document::from(Rubric::from("The Celebrant joins the right hands of the couple and says")),
            Document::from("Those whom God has joined together let no one put asunder."),
            Document::from(Preces::from([
                ("People", "Amen.")
            ])),

            Document::from(Rubric::from("The service continues with The Prayers on page 104 [in the marriage service].")),
            Document::from(Content::DocumentLink {
                label: "Marriage Service".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing, Slug::Version(Version::Expansive)]),
                rotate: false,
                link_only: false
            }),
        ])
    )));

    pub static ref AN_ORDER_FOR_MARRIAGE: Document = Document::new()
        .label("An Order for Marriage")
        .version(Version::Expansive)
        .status(Status::TrialUse)
        .source(Reference {
            source: Source::LiturgicalResources1,
            page: 106
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "An Order for Marriage"))),
            Document::from(Rubric::from("If it is desired to celebrate a marriage otherwise than as provided on pages 76-85 of “Liturgical Resources 1: The Witnessing and Blessing of a Lifelong Covenant (revised and expanded),” this Order is used.")),
            Document::from(Content::DocumentLink {
                label: "Marriage Service".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing, Slug::Version(Version::Expansive)]),
                rotate: false,
                link_only: false
            }),
            Document::from(Rubric::from("Normally, the celebrant is a priest or bishop. Where permitted by civil law, and when no priest or bishop is available, a deacon may function as celebrant, but does not pronounce a nuptial blessing.\n\nThe laws of the State having been complied with, the couple, together with their witnesses, families, and friends assemble in the church or in some other convenient place.")),
            Document::from(Rubric::from("1. The teaching of the Church concerning Holy Matrimony, as it is declared in the formularies, is briefly stated.\n\n2. The intention of the two to enter the state of matrimony, and their free consent, is publicly ascertained.\n\n3. One or more Readings, one of which is always from Holy Scripture, may precede the exchange of vows. If there is to be a Communion, a Reading from the Gospel is always included.\n\n4. The vows are exchanged, using the following form").long()),
            Document::from(Choice::from(vec![
                Document::from("In the Name of God,\nI, *N.*, take you, *N.*, to be my *wife/husband/spouse*, \nto have and to hold from this day forward,\nfor better for worse, for richer for poorer,\nin sickness and in health, to love and to cherish, \nuntil we are parted by death.\nThis is my solemn vow."),
                Document::from("I, *N.*, take thee *N.*, to my wedded *wife/husband/spouse*, \nto have and to hold from this day forward,\nfor better for worse, for richer for poorer,\nin sickness and in health, to love and to cherish,\ntill death us do part, according to God’s holy ordinance; \nand thereto I plight [*or* give] thee my troth.")
            ])),
            Document::from(Rubric::from("5. The Celebrant declares the union of the couple, in the Name of the Father, and of the Son, and of the Holy Spirit.\n\n6. Prayers are offered for the couple, for their life together, for the Christian community, and for the world.\n\n7. A priest or bishop pronounces a solemn blessing upon the couple.\n\n8. If there is no Communion, the service concludes with the Peace, the couple first greeting each other. The Peace may be exchanged throughout the assembly.\n\n9. If there is to be a Communion, the service continues with the Peace and the Offertory. The Holy Eucharist may be celebrated either according to Rite One or Rite Two, or according to the Order on page 401 of the Book of Common Prayer 1979.").long()),
            Document::from(Content::DocumentLink {
                label: "Holy Eucharist".into(),
                path: SlugPath::from([Slug::Eucharist]),
                rotate: false,
                link_only: false
            }),
        ]))));
}
