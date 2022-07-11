use crate::{
    bcp1979::burial::parallels::*,
    rite2::eucharist::{GLORIA_IN_EXCELSIS, KYRIE_ELEISON, TRISAGION},
};
use calendar::Season;
use hymnal::{HymnNumber, Hymnals};
use lazy_static::lazy_static;
use liturgy::*;
use psalter::bcp1979::*;

lazy_static! {
    pub static ref OUR_LADY_OF_GUADALUPE: Document = Document::new()
        .label("Our Lady of Guadalupe")
        .version(Version::BOS)
        .source(Reference {
            source: Source::BookOfOccasionalServices2018,
            page: 27
        })
        .content(Liturgy::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading2, "Concerning the Service"))),
            Document::from(Rubric::from("The Feast of La Virgen de Guadalupe is celebrated on Decemeber 12 throughout Mexico and increasingly in the Episcopal Church.\n\nAccording to tradition, an indigenous man named Juan Diego Cuauhtlatoatzin saw la Virgen on two separate occasions, on December 9 and December 12, 1531. In his vision, she told Juan Diego to ask the archbishop to build a church on the hill of Tepeyac, located in today’s Mexico City. Unconvinced by an uneducated, indigenous person, the bishop asked for proof of La Virgen’s appearance. When Juan Diego returned to Tepeyac, he found roses growing. He gathered the roses in his tilma (cloak) and took them to show to the bishop. Roses spilled out when he unfolded the tilma, and it revealed an image of La Virgen, a dark- skinned indigenous woman, head bowed in prayer. The bishop, convinced by the miracle, built a church.\n\nThe image of La Virgen de Guadelupe permeates Latino cultures. She serves as a rallying point for people’s hopes of liberation and justice revealed in Jesus Christ. The popularity of the feast continues to grow and is attached to many cultural observances. On the day of the feast, people process through towns and cities, offering their songs of love and joy. The procession is followed by a celebration of the Holy Eucharist that may feature dancers, drummers, banners, and mariachis.")),
            Document::from(Heading::from((HeadingLevel::Heading1, "December 12 – Our Lady of Guadalupe"))),
            Document::from(Preces::from([
                ("Presider", "A great sign appeared in the sky, a woman clothed with the sun,"),
                ("People", "With the moon under her feet, and on her head a crown of twelve stars."),
                ("Presider", "Let us pray.")
            ])),
            Document::from(Text::from("Be present with us, God our Deliverer, as you were present with La Virgen de Guadalupe (the Virgin of Guadelupe) who called Juan Diego her beloved son. May her example of tender care for the poor and oppressed guide us in the way of justice and mercy; through Jesus Christ our Lord.").response("Amen.")),
            Document::from(Preces::from([
                ("Deacon", "Let us go forth in peace."),
                ("People", "In the Name of Christ. Amen.")
            ])),
            Document::from(Rubric::from("During the procession, the people may carry flowers, and appropriate hymns, psalms, or anthems are sung, such as the hymn “La Guadalupana.”")),
            Document::from(Rubric::from("Upon arrival in the sanctuary, the Eucharist begins with the Collect of the Day.")),
            Document::from(Rubric::from("The people standing, the Presider says")),
            Document::from(Preces::from([
                ("Presider", "The Lord be with you."),
                ("People", "And also with you"),
                ("Presider", "Let us pray.")
            ])),
            Document::new()	.label("The Collect of the Day")
                .content(Text::from("O God of love, you blessed your people at El Tepeyac with the presence of La Virgen de Guadelupe: grant that her example of love to the poor and forsaken may stir our faith to recognize all people as members of one family. Teach us to follow in the way you have prepared for us, that we may honor one another in word and action, sharing with her your commonwealth of peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.").response("Amen.")),
            Document::from(Rubric::from("The Eucharist continues in the usual manner, using the following Psalm and\nLessons")),
            Document::from(Choice::from(vec![
                        Document::new()
                            .version_label("Zechariah 2:10-13")
                            .content(Series::from(vec![
                                Document::new()
                                    .label("The First Reading")
                                    .content(BiblicalCitation::from("Zechariah 2:10-13")
                                        .intro(BiblicalReadingIntro::from(Document::from(Text::from("A Reading from the Book of Zechariah."))))
                                    ),
                                Document::from(Preces::from([
                                    ("", "The Word of the Lord."),
                                    ("People", "Thanks be to God.")
                                ]))
                        ])),
                        Document::new()
                            .version_label("The Nican Mopohua")
                            .content(Series::from(vec![
                                            Document::new()	.version_label("The Nican Mopohua")
                                                .content(Text::from("A Reading from *The Nican Mopohua*.\n\nJuan Diego returned and right at the top of the mountain he saw the Lady from heaven, at the spot where he saw her the first time. Seeing her, he fell down before her and said, “Lady, the smallest of my daughters, my Child, I went where you sent me to fulfill your mandate; though with difficulty, to the seat of the bishop; I saw him and exposed your message; he received me graciously and listened attentively; but I understood by his response that he believes that it is I who makes the request of building your temple, and that perhaps is not your command. Istrongly beg, Lady, my Child that you send instead of me someone who is known, respected and esteemed. If we entrust your message to someone like that he is sure to be believed. The Bishop does not believe me because I am a lesser man, I'm small, I'm a no one, and you, my Child, the least of my daughters, Lady, you send me to a place where I do not belong. Forgive me if I cause you great sorrow.” Our Lady replied: “Listen, my son, I understand but need you to know thatthat I have many servants and messengers whom I can send to do my bidding yet I choose you. You are the one that I strictly command, to go again tomorrow to see the Bishop. Tell him once more who sends you and that it is my will for a temple to be built in my honor. Tell him, that the ever-Blessed Virgin Mary, mother of God, is the one that sends you.” Juan Diego replied: “My Lady, My Child, I will do what you ask. I'll do your will; but perhaps I will not be heard with pleasure. I know that it will be difficult for them to believe me.”")),
                                            Document::from(Text::from("Here ends the Reading."))
                        ]))
            ])),
            Document::from(Choice::from(vec![
                Document::from(PSALM_131.clone()),
                Document::from(PSALM_116.clone()),
            ])),
            Document::from(Series::from(vec![
                Document::new()
                    .label("The Second Reading")
                    .content(BiblicalCitation::from("Revelation 11:19a; 12:1-6a")
                        .intro(BiblicalReadingIntro::from(Document::from(Text::from("A Reading from the Book of Revelation."))))
                    ),
                Document::from(Preces::from([
                    ("", "The Word of the Lord."),
                    ("People", "Thanks be to God.")
                ]))
            ])),
            Document::from(Series::from(vec![
                            Document::new()
                                .label("The Gospel")
                                .content(BiblicalCitation::from("Luke 6:20-23")
                                    .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                                        ("Presider", "The Holy Gospel of our Lord Jesus Christ according to Luke."),
                                        ("People", "Glory to you, Lord Christ.")
                                    ]))))
                                ),
                            Document::from(Preces::from([
                                ("", "The Gospel of the Lord."),
                                ("People", "Praise to you, Lord Christ.")
                            ]))
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "A Homily"))),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Prayers of the People"))),
            Document::from(Litany::from((
                "Hear our prayer.",
                [
                    "As we honor La Virgen de Guadelupe, may we strive for unity among all people;\nWe pray to you, God of mercy.",
                    "May the vision of Juan Diego inspire our community to stand against prejudice, discrimination, hatred, and violence;\nWe pray to you, God of mercy.",
                    "May our care for youth and children, the elderly and sick, the weak, helpless and poor (especially ______), reflect the love of La Virgen, revealed at Tepeyac;\nWe pray to you, God of mercy.",
                    "May the vision of La Virgen strengthen us to stand with all immigrants and refugees in their struggle in every nation and people, (especially the people of ______);\nWe pray to you, God of mercy.",
                    "May those who have died (especially ) rest in your peace, and those who mourn (especially ) find comfort in fellowship with La Virgen de Guadelupe and all the saints;\nWe pray to you, God of mercy."
                ]
            ))),
            Document::from(Rubric::from("Intercessions may be offered by the people.")),
            Document::from(Rubric::from("The Presider adds this concluding Collect.")),
            Document::from(Text::from("O God of power and mercy, who gives us roses in a season of darkness; fill us with your unexpected truth and vision of peace among all people. By the example of La Virgen de Guadelupe, may we seek the justice which unites all people everywhere; through Jesus Christ our Lord.").response("Amen.")),
            Document::from(Heading::from((HeadingLevel::Heading3, "The Peace"))),
            Document::from(Rubric::from("The Presider says to the people")),
            Document::from(Preces::from([
                ("", "The peace of Christ be always with you."),
                ("People", "And also with you.")
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "At the Eucharist"))),
            Document::from(Rubric::from("The liturgy continues with the Offertory.")),
            Document::new()	.label("Proper Preface")
                .content(Text::from("Because in revealing La Virgen de Guadelupe at El Tepeyac, you have shown us your way of justice and peace, lifted up the small and lowly, and assured us of your great love for the poor and weak.")),
            Document::from(Rubric::from("In place of the usual postcommunion prayer, the following is said")),
            Document::from(Rubric::from("Presider and People")),
            Document::from(Text::from("We give you thanks, O God, for gathering your children together in the Sacrament of Christ’s Body and Blood. We rejoice that you have remembered us; for we were scattered and have now been drawn together from the four corners of the earth. May we who celebrate La Virgen de Guadelupe ever strive for peace, and serve you as witnesses of Jesus Christ, until the Lord dawns in glory.").response("Amen.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("Following the postcommunion prayer, the Congregation may place flowers at the image of La Virgen de Guadalupe. Appropriate hymns, anthems, instrumental music, or songs such as “Buenos Días Paloma Blanca” may accompany this action.")),
            Document::from(Rubric::from("The Bishop, when present, or the Priest, may bless the people.")),
            Document::from(Rubric::from("The Deacon, or the Presider, dismisses the people.")),
            Document::from(Rubric::from("Songs, hymns, and readings traditionally used on the Feast of Our Lady of\nGuadalupe include the following.")),
            Document::from(Heading::from((HeadingLevel::Heading2, "Songs and Hymns:"))),
            Document::from(Text::from("La Guadalupana\nBuenos Días Paloma Blanca\nMi Virgen Ranchera\nO Virgen la mas Hermosa")),
            Document::from(Heading::from((HeadingLevel::Heading3, "From Hymnal 1982"))),
            Document::from(Content::HymnLink(HymnLink::Hymn(Hymnals::Hymnal1982, HymnNumber::H(277)))),
            Document::from(Content::HymnLink(HymnLink::Hymn(Hymnals::Hymnal1982, HymnNumber::H(278)))),
            Document::from(Heading::from((HeadingLevel::Heading3, "From El Himnario"))),
            Document::from(Content::HymnLink(HymnLink::Hymn(Hymnals::ElHimnario, HymnNumber::H(483)))),
            Document::from(Content::HymnLink(HymnLink::Hymn(Hymnals::ElHimnario, HymnNumber::H(62))))
        ]));

    pub static ref A_SERVICE_OF_RENAMING: Document = Document::new()
        .version(Version::BOS)
        .label("A Service of Renaming")
        .source(Reference { source: Source::BookOfOccasionalServices2018, page: 120 })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "A Service of Renaming"))),
            Document::from(Rubric::from("When an event or experience leads a baptized person to take or to be given a new name, the following may be used to mark this transition in the parish community. It is expected that the presider or someone appointed by the presider has prepared the candidate for this rite through pastoral conversation and theological reflection.\n\nThis new beginning is distinct from the new life begun in Holy Baptism, which conveys regeneration and the responsibilities of Christian discipleship. The rite can be used on its own or in place of the Word of God during a celebration of the Holy Eucharist. It is particularly commended for use on a major feast day or any of the following occasions: Advent 3 (Gaudete); Holy Name (Jan. 1); Presentation in the Temple (Feb. 2); The Last Sunday After the Epiphany (Transfiguration Sunday); The Feast of the Transfiguration (Aug. 6).\n\nThroughout the rite, the pronouns “they,” “their,” and “them” are used, with corresponding verb forms. These pronouns should be adapted to the preference of the person receiving or claiming the new name, with appropriate adjustment to the accompanying verbs.")),
            Document::from(Heading::from((HeadingLevel::Heading2, "Opening Acclamation"))),
            Document::from(Preces::from([
                ("Presider", "Blessed be the God of Sarai revealed as Sarah, Jacob who became Israel, and Simon called Peter."),
                ("People", "Blessed be the God who comes among us, reconciles us, and sets us free.")
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "Song of Praise"))),
            Document::from(Rubric::from("A song of praise or the Gloria in excelsis is sung; in Advent and Lent, the Kyrie or Trisagion is used instead")),
            Document::from(Choice::from(vec![
                Document::from(GLORIA_IN_EXCELSIS.clone().display_format(DisplayFormat::Unison))
                    .version_label("Gloria in excelsis")
                    .condition(Condition::All(vec![
                        Condition::Not(Box::new(Condition::Season(Season::Advent))),
                        Condition::Not(Box::new(Condition::Season(Season::Lent))),
                        Condition::Not(Box::new(Condition::Season(Season::HolyWeek))),
                    ])),
                Document::from(KYRIE_ELEISON.clone())
                    .version_label("Kyrie")
                    .condition(Condition::Any(vec![
                        Condition::Season(Season::Advent),
                        Condition::Season(Season::Lent),
                        Condition::Season(Season::HolyWeek),
                    ])),
                Document::from(TRISAGION.clone())
                    .version_label("Trisagion")
                    .condition(Condition::Any(vec![
                        Condition::Season(Season::Advent),
                        Condition::Season(Season::Lent),
                        Condition::Season(Season::HolyWeek),
                    ]))
            ])),
            Document::from(Heading::from((HeadingLevel::Heading2, "Collect"))),
            Document::from(Choice::from(vec![
                Document::from(Text::from("Blessed are you, God of growth and discovery; yours is the inspiration that has altered and changed our lives; yours is the power that has brought us to new dangers and opportunities. Set us, your new creation, to walk through this new world, watching and learning, loving and trusting, until your kingdom comes.").response("Amen.")).version_label("Collect for the Service of Renaming"),
                Document::from(Content::CollectOfTheDay { allow_multiple: false }).version_label("The Collect of the Day"),
            ])),

            Document::from(Heading::from((HeadingLevel::Heading2, "Readings"))),
            Document::from(Rubric::from("The service continues with the readings appointed for the day, readings from the list below, or other scriptural passages suitable to the occasion. If the rite takes place in the context of the Eucharist, a reading from the gospel is always included.")),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Genesis 17:1-7, 15-17").intro(BiblicalReadingIntro::from("A Reading from the Book of Genesis."))).version_label("Genesis 17:1-7, 15-17 - God changes the name of Sarai to Sarah"),
                Document::from(BiblicalCitation::from("Genesis 32:22-31 ").intro(BiblicalReadingIntro::from("A Reading from the Book of Genesis."))).version_label("Genesis 32: 22-31 - Jacob wrestles at Peniel, becomes Israel"),
                Document::from(BiblicalCitation::from("Exodus 3:1-15").intro(BiblicalReadingIntro::from("A Reading from the Book of Exodus."))).version_label("Exodus 3:1-15 - Moses is called to serve the God named I AM"),
                Document::from(BiblicalCitation::from("Isaiah 42:1-9").intro(BiblicalReadingIntro::from("A Reading from the Book of Isaiah."))).version_label("Isaiah 42:1-9 - “Here is my servant whom I uphold”"),
                Document::from(BiblicalCitation::from("Isaiah 43:1-7").intro(BiblicalReadingIntro::from("A Reading from the Book of Isaiah."))).version_label("Isaiah 43:1-7 - “I have called you by name, you are mine”"),
                Document::from(BiblicalCitation::from("Isaiah 56:1-8").intro(BiblicalReadingIntro::from("A Reading from the Book of Isaiah."))).version_label("Isaiah 56: 1-8 - “I will give them an everlasting name”"),
            ])).label("The First Lesson"),

            Document::from(Choice::from(vec![
                Document::from(PSALM_8.clone()).version_label("Psalm 8 - How majestic is God’s name"),
                Document::from(PSALM_23.clone()).version_label("Psalm 23 - The Lord is my Shepherd"),
                Document::from(PSALM_40.clone()).version_label("Psalm 40 - “[God] put a new song in my mouth”"),
                Document::from(PSALM_96.clone()).version_label("Psalm 96 - “Sing to the Lord a new song”"),
            ])),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("2 Corinthians 3:17-18").intro(BiblicalReadingIntro::from("A Reading from the Second Letter to the Corinthians."))).version_label("2 Corinthians 3:17-18 - We are transformed from glory to glory"),
                Document::from(BiblicalCitation::from("2 Corinthians 5:14-21").intro(BiblicalReadingIntro::from("A Reading from the Second Letter to the Corinthians."))).version_label("2 Corinthians 5:14-21 - In Christ, there is a new creation"),
                Document::from(BiblicalCitation::from("Galatians 3:27-28").intro(BiblicalReadingIntro::from("A Reading from the Letter to the Galatians."))).version_label("Galatians 3:27-28 - In Christ there is no longer Jew or Greek..."),
                Document::from(BiblicalCitation::from("Philippians 2:9-13").intro(BiblicalReadingIntro::from("A Reading from the Letter to the Philippians."))).version_label("Philippians 2:9-13 - Christ’s name above all names"),
                Document::from(BiblicalCitation::from("1 John 3:1-2").intro(BiblicalReadingIntro::from("A Reading from the First Letter of John."))).version_label("1 John 3:1-2 - What we will be has not yet been revealed"),
                Document::from(BiblicalCitation::from("Revelation 21:1-6").intro(BiblicalReadingIntro::from("A Reading from the Book of Revelation."))).version_label("Revelation 21:1-6 - ‘See, I am making all things new’"),
            ])).label("The Second Lesson"),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Matthew 16:13-19").intro(BiblicalReadingIntro::from(Document::from(Preces::from([("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."), ("People", "Glory to you, Lord Christ.")]))))).version_label("Matthew 16:13-19 - “You are Peter”"),
                Document::from(BiblicalCitation::from("Matthew 17:1-9").intro(BiblicalReadingIntro::from(Document::from(Preces::from([("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."), ("People", "Glory to you, Lord Christ.")]))))).version_label("Matthew 17:1-9 - The Transfiguration"),
                Document::from(BiblicalCitation::from("Luke 2:15-21").intro(BiblicalReadingIntro::from(Document::from(Preces::from([("", "The Holy Gospel of our Lord Jesus Christ according to Luke."), ("People", "Glory to you, Lord Christ.")]))))).version_label("Luke 2:15-21 - The naming of Jesus"),
                Document::from(BiblicalCitation::from("John 20:11-18").intro(BiblicalReadingIntro::from(Document::from(Preces::from([("", "The Holy Gospel of our Lord Jesus Christ according to John."), ("People", "Glory to you, Lord Christ.")]))))).version_label("John 20:11-18 - The risen Lord calls Mary by name"),
                Document::from(BiblicalCitation::from("John 20: 19-29").intro(BiblicalReadingIntro::from(Document::from(Preces::from([("", "The Holy Gospel of our Lord Jesus Christ according to John."), ("People", "Glory to you, Lord Christ.")]))))).version_label("John 20: 19-29 - Blessed are those who have not seen but believe"),
            ])).label("The Gospel"),

            Document::from(Heading::from((HeadingLevel::Heading2, "Reflections or Sermon"))),
            Document::from(Rubric::from("The candidate may be prepared to offer brief reflections here in place of a Sermon. Depending on the occasion, it may be appropriate to invite others to speak.")),
            Document::from(Rubric::from("On Sundays and other Major Feasts, the Nicene Creed follows the sermon, all standing.")),
            Document::from(Content::DocumentLink {
                label: "Nicene Creed".into(),
                path: SlugPath::from([Slug::NiceneCreed]),
                rotate: false,
                link_only: false
            }),

            Document::from(Heading::from((HeadingLevel::Heading2, "Prayers of the People"))),
            Document::from(Rubric::from("Prayers of the People or a Litany created for the occasion are offered, according to the directions at page 359 in the Book of Common Prayer.")),

            Document::from(Heading::from((HeadingLevel::Heading2, "Rite of Renaming"))),
            Document::from(Preces::from([("Presider", "Hear the invitation of God:")])),
            Document::from("From now on, therefore, we regard no one according to the flesh; even though we once knew Christ according to the flesh, we know him no longer in that way. So if anyone is in Christ, there is a new creation: everything old has passed away; see, everything has become new! All this is from God, who reconciled us to himself through Christ, and has given us the ministry of reconciliation; that is, in Christ God was reconciling the world to himself, not counting their trespasses against them, and entrusting the message of reconciliation to us. So we are ambassadors for Christ, since God is making his appeal through us; we entreat you on behalf of Christ, be reconciled to God. *(2 Cor 5:16-20)*"),
            Document::from(Rubric::from("The Presider turns to the candidate, asking,")),
            Document::from("How do you respond to God’s invitation?"),
            Document::from(Preces::from([
                ("Candidate", "I am a new creation, grateful to embody Christ’s image."),
            ])),
            Document::from("We are here to affirm the name of *N.* This name expresses who *N.* is and who they are becoming, through the grace of God. We honor the name given to them by their parents and acknowledge that the time has come to declare a new name. We honor the name they have chosen, and acknowledge those loved ones who have made holy space for a new name to be spoken. This new name is the culmination of a journey of discovery and, at the same time, a new beginning."),
            Document::from(Rubric::from("Turning to the gathered community, the Presider asks,")),
            Document::from(Preces::from([
                ("", "Will you do all in your power to assist *N.* to embody Christ’s message of reconciliation?"),
                ("People", "We will."),
                ("Presider", "Will you honor N. in name and in spirit as they continue on their path?"),
                ("People", "We will.")
            ])),
            Document::from("Dynamic and holy God, we remember how you changed the names of Abraham and Sarah, as they set out to follow you. We marvel that you changed the name of Jacob, after a long night of wrestling with you. We recall our ancestors in the faith who were given new names as their vocations to serve you were revealed. We now declare publicly and affirm the name you have bestowed upon *N.*"),
            Document::from(Rubric::from("All lay hands upon the head of the candidate, or upon the shoulders of those around the candidate.")),
            Document::from(Text::from("*N.*, receive the blessing of God, the Holy and Undivided Trinity. Walk in the Spirit, this day and always, knowing that God has made an everlasting covenant with you that shall never be cut off.").response("Amen.")),
            Document::from(Preces::from([
                ("Presider", "Jesus said, ‘Rejoice that your names are written in heaven.” *(Luke 10:20)*"),
                ("People", "Amen. Alleluia.")
            ])),
            Document::from(Rubric::from("The Presider concludes with one of the following collects:")),
            Document::from(Choice::from(vec![
                Document::from(Text::from("Holy One of blessing, you created us in your image and pronounced us good. We give you thanks for the gift of life itself. We thank you for our individual names, which connect us to the One who spoke all creation into being. We rejoice in our shared calling to the ministry of reconciliation.").response("Amen.")),
                Document::from(Text::from("God of transformations, you set us free to change and grow, even while you hold us close in love and grace. Send us forth to love and serve you, in Christ’s holy Name.").response("Amen."))
            ])),

            Document::from(Rubric::from("*Sources Consulted and/or Adapted*\n\nJustin Tanis, *Trans-gendered: Theology, Ministry and Communities of Faith* (Cleveland, OH: Pilgrim Press, 2003),189-193.\n\n*Changes: Prayers and Services Honoring Rites of Passage* (Church Publishing, 2007), 47-49.\n\n*A New Zealand Prayer Book* (Harper One, 4th edition, 1997), 475.").long())
        ]))));

    pub static ref BURIAL_OF_A_NON_CHRISTIAN: Document = Document::new()
        .version(Version::BOS)
        .label("Burial of a Non-Christian")
        .source(Reference {
            source: Source::BookOfOccasionalServices2018,
            page: 188
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "Burial of One Who Does Not Profess the Christian Faith"))).tags([TITLE]),
            Document::from(Rubric::from("This anthem; and any of the following Psalms, Lessons, and Prayers; and the form of Committal given below may be used with the Order for Burial on page 506 of the Prayer Book.")).tags([OPENING_RUBRIC]),
            Document::from(Content::DocumentLink {
                label: "An Order for Burial".into(),
                path: SlugPath::from([Slug::Burial, Slug::Order]),
                rotate: false,
                link_only: false
            }),
            Document::from(Heading::from((HeadingLevel::Heading2, "Opening Anthem"))).tags([OPENING_RUBRIC]),
            Document::from(Text::from("The steadfast love of the Lord never ceases, \nhis mercies never come to an end;\nthey are new every morning; \ngreat is his faithfulness.\nThe Lord will not cast off forever.\nThough he cause grief, he will have compassion \naccording to the abundance of his steadfast love;\nThe Lord does not willingly afflict or grieve his children.")).tags([ANTHEMS]),
            Document::from(Heading::from((HeadingLevel::Heading2, "Lessons and Psalms"))).tags([LESSONS_HEADING]),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Ecclesiastes 3:1-11").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Ecclesiastes.")))).label("The First Lesson").version_label("Ecclesiastes 3:1-11 (For everything there is a season)"),
                Document::from(BiblicalCitation::from("Ecclesiastes 12:1-7").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Ecclesiastes.")))).label("The First Lesson").version_label("Ecclesiastes 12:1-7 (Remember your Creator in the days of your youth)"),
            ])).tags([FIRST_LESSON]),
            Document::from(Choice::from(vec![
                Document::from(PSALM_23.clone()).version_label("Psalm 23 (The Lord is my shepherd)"),
                Document::from(PSALM_90.clone()).version_label("Psalm 90 (Lord, you have been our refuge)"),
                Document::from(PSALM_121.clone()).version_label("Psalm 121 (I lift up my eyes to the hills)"),
                Document::from(PSALM_130.clone()).version_label("Psalm 130 (Out of the depths have I called to you, O Lord) ")
            ])).tags([PSALM]),
            Document::from(BiblicalCitation::from("Romans 8:35-39").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Romans.")))).label("The Second Lesson").version_label("Romans 8:35-39 (Who shall separate us from the love of Christ?)").tags([SECOND_LESSON]),
            Document::from(BiblicalCitation::from("John 10:11-16")
                .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                    ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                    ("People", "Glory to you, Lord Christ.")
                ]))))
            ).label("The Gospel").version_label("John 10:11-16 (I am the good shepherd)").tags([GOSPEL]),

            Document::from(Heading::from((HeadingLevel::Heading2, "Prayers"))).tags([PRAYERS_TITLE]),
            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading3, "For the Deceased"))),
                Document::from(Text::from("Almighty God, we entrust all who are dear to us to your never-failing care and love, for this life and the life to come, knowing that you are doing for them better things than we can desire or pray for; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Into your hands, O God, we commend our *brother, N.*, as into the hands of a faithful Creator and most loving Savior. In your infinite goodness, wisdom, and power, work in *him* the merciful purpose of your perfect will, through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Heading::from((HeadingLevel::Heading3, "For those who mourn"))),
                Document::from(Text::from("O God of grace and glory, we remember before you this day our brother (sister), *N.* We thank you for giving *him* to us, *his* family and friends, to know and to love as a companion on our earthly pilgrimage. In your boundless compassion, console us who mourn. Give us quiet confidence that we may continue our course in faith; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("O merciful Father, you have taught us in your holy Word that you do not willingly afflict or grieve your children: Look with pity upon the sorrows of your servants for whom our prayers are offered. Remember them, O Lord, in mercy, nourish their souls with patience, comfort them with a sense of your goodness, lift up your countenance upon them, and give them peace; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Almighty God, Father of mercies and giver of comfort: Deal graciously, we pray, with all who mourn; that, casting all their care on you, they may know the consolation of your love; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Most merciful God, whose wisdom is beyond our understanding, deal graciously with *N.N.* in *their* grief. Surround *them* with your love, that *they* may not be overwhelmed by *their* loss, but have confidence in your goodness, and strength to meet the days to come; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Heading::from((HeadingLevel::Heading3, "For the Christian community"))),
                Document::from(Text::from("Most loving Father, whose will it is for us to give thanks for all things, to fear nothing but the loss of you, and to cast all our care on you who care for us: Preserve us from faithless fears and worldly anxieties, that no clouds of this mortal life may hide from us the light of that love which is immortal, and which you have manifested to us in your Son Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Almighty God, give us grace to cast away the works of darkness, and put on the armor of light, now in the time of this mortal life in which your Son Jesus Christ came to visit us in great humility; that in the last day, when he shall come again in his glorious majesty to judge both the living and the dead, we may rise to the life immortal; through him who lives and reigns for ever and ever.").response("Amen.")),
            ])).tags([PRAYERS]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Committal"))),
                Document::from(Antiphon::from("Holy God, Holy and Mighty, Holy Immortal One, have mercy upon us.")),
                Document::from(Text::from("You only are immortal, the creator and maker of mankind; and we are mortal, formed of the earth, and to earth shall we return. For so did you ordain when you created me, saying, “You are dust, and to dust you shall return.” All of us go down to the dust; yet even at the grave we make our song: Alleluia, alleluia, alleluia.")),
                Document::from(Antiphon::from("Holy God, Holy and Mighty, Holy Immortal One, have mercy upon us."))
            ])).tags([COMMITTAL])
        ]))));
}
