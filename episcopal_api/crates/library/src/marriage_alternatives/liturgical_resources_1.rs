use lazy_static::lazy_static;
use liturgy::*;
use psalter::bcp1979::*;

use crate::conditions::EASTER_SEASON;

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

    pub static ref WITNESSING_AND_BLESSING_OF_A_LIFELONG_COVENANT: Document = Document::new()
        .label("The Witnessing and Blessing of a Lifelong Covenant")
        .version(Version::Expansive)
        .source(Reference {
            source: Source::LiturgicalResources1,
            page: 79
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Witnessing and Blessing\nof a Lifelong Covenant"))),

            // Concerning the Service
            Document::from(Heading::from((HeadingLevel::Heading3, "Concerning the Service"))),
            Document::from(Rubric::from("This rite is appropriately celebrated in the context of the Holy Eucharist and may take place at the principal Sunday Liturgy. This rite then replaces the Ministry of the Word. A bishop or priest normally presides. Parallel texts from Enriching Our Worship 1 are included as options for elements of this rite.\n\nAt least one of the couple must be a baptized Christian.\n\nTwo or more presenters, who may be friends, parents, family members, or drawn from the local assembly, may present the couple to the presider and the assembly.\n\nAs indicated in the opening address, the consent, and the blessing of the rings, the rite may be modified for use with a couple who have previously made a lifelong commitment to one another.\n\n").long()),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Word of God")))
                .source(Reference {
                    source: Source::LiturgicalResources1,
                    page: 80
                }),
            Document::from(Heading::from((HeadingLevel::Heading3, "Gathering"))),
            Document::from(Rubric::from("The couple to be blessed joins the assembly.")),
            Document::from(Rubric::from("A hymn of praise, psalm, or anthem may be sung, or instrumental music may be played.")),
            Document::from(HymnLink::Tag("Marriage".into())),

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
            ])),

            Document::from(Rubric::from("The Presider may address the assembly in these words")),
            Document::from(Choice::from(vec![
                Document::from("Dear friends in Christ, (*or* Dearly beloved), in the name of God and the Church\nwe have come together today with *N. N.* and *N. N.*,\nto witness the vows they make, committing themselves to one another. Forsaking all others,\nthey will bind themselves to one another\nin a covenant of mutual fidelity and steadfast love, remaining true to one another in heart, body, and mind, as long as they both shall live.\nSuch a lifelong commitment\nis not to be entered into lightly or thoughtlessly, but responsibly and with reverence.\nLet us pray, then, that God will give them the strength to remain steadfast in what they vow this day.\nLet us also pray for the generosity to support them in the commitment they undertake,\nand for the wisdom to see God at work in their life together.\n"),
                Document::from("Dear friends in Christ [*or* Dearly beloved], in the name of God and the Church,\nwe have come together with *N. N.* and *N. N.*, to witness the sacred vows they make\nas they solemnize [or reaffirm] their commitment to one another. Today they renew their covenant of mutual fidelity and steadfast love,\nforsaking all others and remaining true to one another in heart, body, and mind, as long as they both shall live.\nLet us pray, then, that God will give them the strength to remain steadfast in what they vow this day.\nLet us also pray for the generosity\nto support them in the commitment they undertake,\nand for the wisdom to see God at work in their life together.\n")
                    .version_label("For those who have previously made a lifelong commitment to one another")
            ])),

            Document::from(Heading::from((HeadingLevel::Heading3, "The Collect of the Day"))),
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
            ])),
            Document::from(Choice::from(vec![
                Document::from(Text::from("God of abundance:\nassist by your grace *N.* and *N.*,\nwhose covenant of love and fidelity we witness this day.\nGrant them your protection, that with firm resolve\nthey may honor and keep the vows they make;\nthrough Jesus Christ our Savior,\nwho lives and reigns with you in the unity of the Holy Spirit,\none God, for ever and ever.").response("Amen")),
                Document::from(Text::from("Almighty and everliving God:\nlook tenderly upon *N.* and *N.*,\nwho stand before you in the company of your Church.\nLet their life together bring them great joy.\nGrant them so to love selflessly and live humbly,\nthat they may be to one another and to the world\na witness and a sign of your never-failing love and care;\nthrough Jesus Christ your Son our Lord,\nwho lives and reigns with you and the Holy Spirit,\none God, to the ages of ages.").response("Amen.")),
                Document::from(Text::from("O God, faithful and true,\nwhose steadfast love endures for ever:\nwe give you thanks for sustaining *N.* and *N.* in the life they share\nand for bringing them to this day.\nNurture them and fill them with joy in their life together,\ncontinuing the good work you have begun in them;\nand grant us, with them, a dwelling place eternal in the heavens\nwhere all your people will share the joy of perfect love,\nand where you, with the Son and the Holy Spirit, live and reign,\none God, now and for ever.").response("Amen.")),
                Document::from(Text::from("Holy Trinity, one God,\nthree Persons perfect in unity and equal in majesty:\nDraw together with bonds of love and affection\n*N.* and *N.*, who with *their families*\nseek to live in harmony and forbearance all their days,\nthat their joining together will be to us\na reflection of that perfect communion\nwhich is your very essence and life,\nO Father, Son, and Holy Spirit,\nwho live and reign in glory everlasting.").response("Amen."))
                    .version_label("For Those Who Bring Children")
            ])),

            // The Readings
            Document::from(Heading::from((HeadingLevel::Heading2, "The Lessons"))),
            Document::from(Rubric::from("The people sit. Then one or more of the following passages of Scripture is read. If the Holy Communion is to be celebrated, a passage from the Gospels always concludes the Readings. When the blessing is celebrated in the context of the Sunday Eucharist, the Readings of the Sunday are used, except with the permission of the Bishop.")),

            // First Lesson
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
            READING_RESPONSE.clone(),

            // Psalm
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

            // Second Lesson
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

            // Gospel
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
            ])),

            // Sermon
            Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon"))),

            // Vows
            Document::from(Heading::from((HeadingLevel::Heading2, "The Witnessing of the Vows and the Blessing of the Covenant"))),
            Document::from(Rubric::from("The couple comes before the assembly. If there is to be a presentation, the presenters stand with the couple, and the Presider says to them")),
            Document::from(Preces::from([
                ("Presider", "Who presents *N.* and *N.*, as they seek the blessing of God and the Church on their love and life together?"),
                ("Presenters", "We do."),
                ("Presider", "Will you love, respect, and pray for *N.* and *N.*, and do all in your power to stand with them in the life they will share?"),
                ("Presenters", "We will.")
            ])),
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
            Document::from(Rubric::from("The assembly stands, the couple faces the People, and the Presider addresses them, saying")),
            Document::from(Preces::from([
                ("Presider", "Will all of you gathered to witness these vows do all in your power to uphold and honor this couple in the covenant they make?"),
                ("Answer", "We will."),
                ("Presider", "Will you pray for them, especially in times of trouble, and celebrate with them in times of joy?"),
                ("Answer", "We will."),
            ])),

            // Prayers
            Document::from(Heading::from((HeadingLevel::Heading2, "The Prayers"))),
            Document::from(Rubric::from("The Presider then introduces the prayers")),
            Document::from(Preces::from([
                ("Presider", "Then let us pray for *N.* and *N.* in their life together and for the concerns of this community."),
            ])),
            Document::from(Rubric::from("A Deacon or another leader bids prayers for the couple.\n\nPrayers for the Church and for the world, for the concerns of the local community, for those who suffer or face trouble, and for the departed are also appropriate. If the rite takes place in the principal Sunday worship of the congregation, the rubric concerning the Prayers of the People on page 359 of the Book of Common Prayer is followed.\n\nAdaptations or insertions may be made to the form that follows.\n\nA bar in the margin indicates a bidding that may be omitted.")),
            Document::from(Choice::from(vec![
                Document::from(Series::from(vec![
                    Document::from(Litany::from((
                        "Hear our prayer.",
                        [
                            "For *N.* and *N.*, seeking your blessing and the blessing of your holy people;\nLoving God,",
                            "For a spirit of loving-kindness to shelter them all their days;\nLoving God,",
                            "For friends to support them and communities to enfold them;\nLoving God,",
                            "For peace in their home and love in their family;\nLoving God,",
                            "| For the grace and wisdom to care for the children you entrust to them [*or* may entrust to them];\nLoving God,",
                            "For the honesty to acknowledge when they hurt each other, and the humility to seek each other’s forgiveness and yours;\nLoving God,",
                            "For the outpouring of your love through their work and witness;\nLoving God,",
                            "For the strength to keep the vows each of us has made;\nLoving God,",
                        ]
                    ))),
                    Document::from(Rubric::from("The leader may add one or more of the following biddings")),
                    Document::from(Litany::from((
                        "Hear our prayer.",
                        [
                            "| For all who have been reborn and made new in the waters of Baptism;\nLoving God,",
                            "| For those who lead and serve in communities of faith;\nLoving God,",
                            "| For those who seek justice, peace, and concord among nations;\nLoving God",
                            "| For those who are sick or suffering, homeless or poor;\nLoving God,",
                            "| For victims of violence and those who inflict it;\nLoving God,",
                            "| For communion with all who have died, [especially those whom we remember this day:	___________];\nLoving God,"
                        ]
                    ))),
                ]))
                .version_label("Loving God"),
                Document::from(Series::from(vec![
                    Document::from(Litany::from((
                        "Hear our prayer.",
                        [
                            "For *N.* and *N.*, seeking your blessing and the blessing of your holy people;\nLord, in your mercy,",
                            "For a spirit of loving-kindness to shelter them all their days;\nLord, in your mercy,",
                            "For friends to support them and communities to enfold them;\nLord, in your mercy,",
                            "For peace in their home and love in their family;\nLord, in your mercy,",
                            "| For the grace and wisdom to care for the children you entrust to them [*or* may entrust to them];\nLord, in your mercy,",
                            "For the honesty to acknowledge when they hurt each other, and the humility to seek each other’s forgiveness and yours;\nLord, in your mercy,",
                            "For the outpouring of your love through their work and witness;\nLord, in your mercy,",
                            "For the strength to keep the vows each of us has made;\nLord, in your mercy,",
                        ]
                    ))),
                    Document::from(Rubric::from("The leader may add one or more of the following biddings")),
                    Document::from(Litany::from((
                        "Hear our prayer.",
                        [
                            "| For all who have been reborn and made new in the waters of Baptism;\nLord, in your mercy,",
                            "| For those who lead and serve in communities of faith;\nLord, in your mercy,",
                            "| For those who seek justice, peace, and concord among nations;\nLoving God",
                            "| For those who are sick or suffering, homeless or poor;\nLord, in your mercy,",
                            "| For victims of violence and those who inflict it;\nLord, in your mercy,",
                            "| For communion with all who have died, [especially those whom we remember this day:	___________];\nLord, in your mercy,"
                        ]
                    ))),
                ]))
                .version_label("Lord, in your mercy")
            ])),
            Document::from(Rubric::from("The Presider concludes the Prayers with the following or another appropriate Collect")),
            Document::from(Text::from("Giver of every gift, source of all goodness,\nhear the prayers we bring before you for N. and N., \nwho seek your blessing this day.\nStrengthen them as they share in the saving work of Jesus, \nand bring about for them and for all you have created\nthe fullness of life he promised,\nwho now lives and reigns for ever and ever.").response("Amen.")),

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

            // The Commitment
            Document::from(Heading::from((HeadingLevel::Heading2, "The Commitment"))),
            Document::from(Rubric::from("The People sit. The couple stands, facing the Presider.")),
            Document::from(Preces::from([
                ("Presider", "*N.* and *N.*, I invite you now, illumined by the Word of God and strengthened by the prayer of this community, to make your covenant before God and the Church.")
            ])),
            Document::from(Rubric::from("Each member of the couple, in turn, takes the right hand of the other and says")),
            Document::from(Choice::from(vec![
                Document::from("In the name of God,\nI, N., give myself to you, N., and take you to myself.\nI will support and care for you by the grace of God:\nin times of sickness, in times of health.\nI will hold and cherish you in the love of Christ:\nin times of plenty, in times of want.\nI will honor and keep you with the Spirit’s help:\nin times of anguish, in times of joy,\nforsaking all others, as long as we both shall live.\nThis is my solemn vow."),
                Document::from("In the name of God,\nI, N., give myself to you, N., and take you to myself.\nI will support and care for you:\nin times of sickness, in times of health.\nI will hold and cherish you:\nin times of plenty, in times of want.\nI will honor and love you:\nin times of anguish, in times of joy,\nforsaking all others, as long as we both shall live.\nThis is my solemn vow.")
            ])),

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
            ])),

            Document::from(Heading::from((HeadingLevel::Heading2, "Pronouncement"))),
            Document::from(Rubric::from("The Presider joins the right hands and says")),
            Document::from(Text::from("Now that N. and N. have exchanged vows of love and fidelity\nin the presence of God and the Church,\nI now pronounce that they are bound to one another\nas long as they both shall live.").response("Amen.")),

            Document::from(Heading::from((HeadingLevel::Heading2, "Blessing of the Couple"))),
            Document::from(Rubric::from("As the couple stands or kneels, the Presider invokes God’s blessing upon them, saying")),
            Document::from("Let us pray.\n\nMost gracious God,\nwe praise you for the tender mercy and unfailing care revealed to us in Jesus the Christ\nand for the great joy and comfort bestowed upon us\nin the gift of human love.\nWe give you thanks for *N.* and *N.*,\nand the covenant of faithfulness they have made.\nPour out the abundance of your Holy Spirit upon them.\nKeep them in your steadfast love;\nprotect them from all danger;\nfill them with your wisdom and peace;\nlead them in holy service to each other and the world."),
            Document::from(Rubric::from("The Presider continues with one of the following")),
            Document::from(Choice::from(vec![
                Document::from(Text::from("God the Father,\nGod the Son,\nGod the Holy Spirit,\nbless, preserve, and keep you,\nand mercifully grant you rich and boundless grace,\nthat you may please God in body and soul.\nGod make you a sign of the loving-kindness and steadfast fidelity\nmanifest in the life, death, and resurrection of our Savior,\nand bring you at last to the delight of the heavenly banquet,\nwhere he lives and reigns for ever and ever.").response("Amen.")),
                Document::from(Text::from("God, the holy and undivided Trinity, \nbless, preserve, and keep you,\nand mercifully grant you rich and boundless grace, \nthat you may please God in body and soul.\nGod make you a sign of the loving-kindness and steadfast fidelity \nmanifest in the life, death, and resurrection of our Savior,\nand bring you at last to the delight of the heavenly banquet, \nwhere he lives and reigns for ever and ever.").response("Amen.")),
            ])),

            Document::from(Heading::from((HeadingLevel::Heading3, "The Peace"))),
            Document::from(Rubric::from("The Presider bids the Peace.")),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("Presider", "The peace of the Lord be always with you."),
                    ("People", "And also with you.")
                ])),
                Document::from(Preces::from([
                    ("Presider", "The peace of Christ be always with you."),
                    ("People", "And also with you.")
                ]))
            ])),
            Document::from(Rubric::from("The liturgy continues with the Holy Communion. When the Eucharist is not celebrated, the Presider blesses the people. The Deacon, or in the absence of a Deacon, the Priest, dismisses them.")),

            // At the Eucharist
            Document::from(Heading::from((HeadingLevel::Heading2, "At the Eucharist"))),
            Document::from(Rubric::from("The liturgy continues with the Offertory, at which the couple may present the offerings of bread and wine.")),
            Document::from(Rubric::from("The following proper preface may be said.")),
            Document::from("Because in the giving of two people to each other in faithful love \nyou reveal the joy and abundant life you share \nwith your Son Jesus Christ and the Holy Spirit."),
            Document::from(Rubric::from("The following postcommunion prayer may be said.")),
            Document::from(Text::from("God our strength and joy,\nwe thank you for the communion of our life together, \nfor the example of holy love that you give us in *N.* and *N.*, \nand for the Sacrament of the Body and Blood\nof our Savior Jesus Christ.\nGrant that it may renew our hope\nand nourish us for the work you set before us\nto witness to the presence of Christ in the world, \nthrough the power of your Spirit,\nand to the glory of your Name.").response("Amen.").display_format(DisplayFormat::Unison))
    ]))));
}
