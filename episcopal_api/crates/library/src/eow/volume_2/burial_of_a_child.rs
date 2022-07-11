use crate::bcp1979::burial::parallels::*;
use crate::rite2::{LORDS_PRAYER_CONTEMPORARY_TEXT, LORDS_PRAYER_TRADITIONAL_TEXT};
use lazy_static::lazy_static;
use liturgy::*;
use psalter::bcp1979::{PSALM_121, PSALM_139, PSALM_142, PSALM_23, PSALM_42};

lazy_static! {
    pub static ref BURIAL_OF_A_CHILD: Document = Document::new()
        .version(Version::EOW)
        .source(Reference {
            source: Source::EOW2,
            page: 131
        })
        .label("Burial of a Child")
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "Burial of a Child")))
                .tags([TITLE]),


            Document::from(Series::from(vec![
                Document::from(Heading::from((
                    HeadingLevel::Heading2,
                    "Concerning the Service"
                ))),
                Document::from(Rubric::from("The death of a member of the Church should be reported as soon as possible to, and arrangements for the funeral should be made in consultation with, the Minister of the Congregation.\n\nBaptized Christians are properly buried from the church. The service should be held at a time when the congregation has opportunity to be present.\n\nThe coffin is to be closed before the service, and it remains closed thereafter. It is appropriate that it be covered with a pall or other suitable covering. If necessary, or if desired, all or part of the service of Committal may be said in the church. If preferred, the Committal service may take place before the service in the church. It may also be used prior to cremation.\n\nA priest normally presides at the service. It is appropriate that the bishop, when present, preside at the Eucharist and pronounce the Commendation. \n\nIt is desirable that the Lesson from the Old Testament, and the Epistle, be read by lay persons.\n\nWhen the services of a priest cannot be obtained, a deacon or lay reader may preside at the service.\n\nIt is customary that the celebrant meet the body and go before it into the church or towards the grave.\n\nThe anthems at the beginning of the service are sung or said as the body is borne into the church, or during the entrance of the ministers, or by the celebrant standing in the accustomed place.\n\nWhen children die, it is usually long before their expected span of life. Often they die very suddenly and sometimes violently, whether as victims of abuse, gunfire, or drunken drivers, adding to the trauma of their survivors. The surprise and horror at the death of a child call for a liturgical framework that addresses these different expectations and circumstances.").long())
            ])),

            Document::from(Heading::from((
                HeadingLevel::Heading2,
                "Gather in the Name of God"
            ))).tags([OPENING_RUBRIC]),
            Document::from(Rubric::from("All stand while one or more of the following is said or sung")).tags([OPENING_RUBRIC]),

            Document::from(Series::from(vec![
                Document::from(Sentence::from("He will feed his flock like a shepherd; he will gather the lambs in his arms, and carry them in his bosom.").citation("Isaiah 40:11")),
                Document::from(Sentence::from("The eternal God is your refuge, and underneath are the everlasting arms.").citation("Deuteronomy 33:27")),
                Document::from(Sentence::from("As a mother comforts her child, so I will comfort you.").citation("Isaiah 66:13a")),
                Document::from(Sentence::from("When Israel was a child, I loved him.... it was I who taught Ephraim to walk, I took them up in my arms.... I led them with...bands of love. I was to them like those who lift infants to their cheeks. I bent down to them and fed them.").citation("Hosea 11:1a, 3, 4")),
                Document::from(Sentence::from("For these things I weep; my eyes flow with tears.... But you, O Lord, reign for ever; your throne endures to all generations.").citation("Lamentations 1: 16a; 5:19")),
                Document::from(Sentence::from("Jesus said, Let the little children come to me, and do not stop them; for it is to such as these that the kingdom of heaven belongs.").citation("Matthew 19:14")),
                Document::from(Sentence::from("For the Lamb at the center of the throne will be their shepherd, and he will guide them to springs of the water of life, and God will wipe away every tear from their eyes.").citation("Revelation 7:17")),
            ]).indivisible()).tags([ANTHEMS]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("When all are in place, the Minister may address the congregation, acknowledging briefly the purpose of their gathering, and bidding their prayers for the deceased and the bereaved.")),
                Document::from(Rubric::from("The Minister says one of the following Collects, first saying")),
                Document::from(Preces::from([
                    ("", "The Lord be with you."),
                    ("People", "And also with you."),
                    ("Minister", "Let us pray.")
                ])),
                Document::from(Rubric::from("Silence")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from("Holy God, your beloved Son took children into his arms and blessed them. Help us to entrust *N.* to your never failing loving- kindness. Comfort us as we bear the pain of her/his death, and reunite us in your good time in your Paradise; through Jesus Christ our Savior who lives and reigns with you and the Holy Spirit, one God, now and forever.").response("Amen.")),
                    Document::from(Text::from("Gracious God, we come before you this day in pain and sorrow. We grieve the loss of *N.*, a precious human life. Give your grace to those who grieve [especially *N.*], that they may find comfort in your presence and be strengthened by your Spirit. Be with this your family as they mourn, and draw them together in your healing love; in the name of the one who suffered, died, and rose for us, Jesus our Savior.").response("Amen."))
                ])),
            ])).tags([OPENING_PRAYERS]),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Lessons"))).tags([LESSONS_HEADING]),
            Document::from(Rubric::from("One or more of the following passages from Holy Scripture is read. If the Eucharist is celebrated, a passage from the Gospel always concludes the Readings")).tags([LESSONS_RUBRIC]),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("2 Samuel 12:16-23").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Second Book of Samuel.")))).label("From the Old Testament").version_label("2 Samuel 12:16-23 (the death of David’s child)"),
                Document::from(BiblicalCitation::from("Isaiah 65:17-20, 23-25").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Isaiah.")))).label("From the Old Testament").version_label("Isaiah 65:17-20, 23-25 (I am about to create new heavens and a new earth)"),
                Document::from(BiblicalCitation::from("Isaiah 66:7-14").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Isaiah.")))).label("From the Old Testament").version_label("Isaiah 66:7-14 (As a mother comforts her child, so will I comfort you)"),
                Document::from(BiblicalCitation::from("Jeremiah 31:15-17").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Jeremiah.")))).label("From the Old Testament").version_label("Jeremiah 31:15-17 (Rachel weeping for her children)"),
            ])).tags([FIRST_LESSON]),

            Document::from(Choice::from(vec![
                Document::from(PSALM_23.clone()),
                Document::from(PSALM_42.clone().citation("Psalm 42:1-7")),
            ])).tags([PSALM]),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Romans 8:31-39").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Romans.")))).label("From the New Testament").version_label("Romans 8: 31-39 (Who will separate us from the love of Christ?)"),
                Document::from(BiblicalCitation::from("1 Thessalonians 4:13-14,18").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter to the Thessalonians.")))).label("From the New Testament").version_label("1 Thessalonians 4:13-14,18 (We do not want you to be uninformed about those who have died)"),
                Document::from(BiblicalCitation::from("1 John 3:1-2").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter of John.")))).label("From the New Testament").version_label("1 John 3:1-2 (See what love the Father has given us)"),
            ])).tags([SECOND_LESSON]),

            Document::from(Choice::from(vec![
                Document::from(PSALM_121.clone()),
                Document::from(PSALM_139.clone().citation("Psalm 139:7-12")),
                Document::from(PSALM_142.clone().citation("Psalm 142:1-6"))
            ])).tags([PSALM_2]),

            Document::from(Choice::from(vec![
                Document::from(
                    BiblicalCitation::from("Matthew 5:1-10")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("Matthew 5: 1-10 (Blessed are those who mourn)"),
                Document::from(
                    BiblicalCitation::from("Matthew 18:1-5, 10-14")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("Matthew 18: 1-5, 10-14 (this child is the greatest in the kingdom)"),
                Document::from(
                    BiblicalCitation::from("Mark 10:13-16")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to Mark."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("Mark 10:13-16 (Let the little children come to me)"),
                Document::from(
                    BiblicalCitation::from("Matthew 19:13-15")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("Matthew 19:13-15 (Let the little children come to me)"),
                Document::from(
                    BiblicalCitation::from("Luke 18:15-17")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to Luke."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("Luke 18:15-17 (Let the little children come to me)"),
                Document::from(
                    BiblicalCitation::from("John 10:11-16")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 10:11-16 (I am the good shepherd)"),
            ])).tags([GOSPEL]),

            Document::from(Heading::from((HeadingLevel::Heading2, "The Sermon"))).tags([HOMILY]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Apostles’ Creed may then be said, all standing. The Celebrant may introduce the Creed with these or similar words")),
                Document::from(Text::from("In the assurance of eternal life given at Baptism, let us proclaim our faith and say,")),
                Document::from(Rubric::from("Celebrant and People")),
                Document::from(Text::from("I believe in God, the Father almighty,\n\tcreator of heaven and earth.\nI believe in Jesus Christ, his only Son, our Lord.\n\tHe was conceived by the power of the Holy Spirit\n\t\tand born of the Virgin Mary.\n\tHe suffered under Pontius Pilate,\n\t\twas crucified, died, and was buried.\n\tHe descended to the dead.\n\tOn the third day he rose again.\n\tHe ascended into heaven,\n\t\tand is seated at the right hand of the Father.\n\tHe will come again to judge the living and the dead.\n\nI believe in the Holy Spirit,\n\tthe holy catholic Church,\n\tthe communion of saints,\n\tthe forgiveness of sins,\n\tthe resurrection of the body,\n\tand the life everlasting. Amen.").display_format(DisplayFormat::Unison))
            ])).tags([CREED]),

            Document::from(Rubric::from("The service continues with the Prayers. If the Eucharist is not celebrated, the Lord’s Prayer concludes the intercessions.")).tags([RUBRIC_BEFORE_PRAYERS]),
            Document::from(Heading::from((HeadingLevel::Heading2, "The Prayers of the People"))).tags([PRAYERS_TITLE]),
            Document::from(Rubric::from("The Deacon or other person appointed says")).tags([PRAYERS_RUBRIC]),

            Document::from(Series::from(vec![
                Document::from(Litany::from((
                    "O God, have mercy",
                    [
                        "In the peace of God, let us pray, responding “O God, have mercy.”\n\nIn the assurance of your mercy, in thanksgiving for the life of your child *N.*, and in confident expectation of the resurrection to eternal life, we pray",
                        "| Remember *N.’s* parents, *N. N.* Help them to hold each other in their hearts, that this sorrow may draw them together and not tear them apart, we pray",
                        "| Remember *N.’s* brother(s) *N., N.* and sister(s) *N., N.,* that *they/he/she* may be enfolded in love, comforted in fear, honored in *their/his/her* grief, and kept safe, we pray",
                        "Remember all the family and friends of *N.*, that they may know the consolation of your love, and may hold *N.* in their love all the days of their lives, we pray",
                        "Support them in their grief, and be present to all who mourn, we pray",
                        "Teach us to be patient and gentle with ourselves and each other as we grieve, we pray",
                        "Help us to know and accept that we will be reunited at your heavenly banquet, we pray",
                        "Finally, our God, help us become co-creators of a world in which children are happy, healthy, loved and do not know want or hunger, we pray",
                    ]
                ))),
                Document::from(Rubric::from("The Minister concludes the prayers with this Collect")),
                Document::from(Text::from("Compassionate God, your ways are beyond our understanding, and your love for those whom you create is greater by far than ours; comfort all who grieve for this child *N.* Give them the faith to endure the wilderness of bereavement and bring them in the fullness of time to share with *N.* the light and joy of your eternal presence; through Jesus Christ our Lord.").response("Amen.")),
            ])).tags([PRAYERS]),

            Document::from(Rubric::from("When the Eucharist is not to be celebrated, the service continues with the Commendation or with the Committal.")).tags([RUBRIC_AFTER_PRAYERS]),

            Document::from(Heading::from((HeadingLevel::Heading2, "At the Eucharist"))).tags([AT_THE_EUCHARIST_TITLE]),
            Document::from(Rubric::from("In place of the usual postcommunion prayer, the following is said")).tags([POSTCOMMUNION_PRAYER]),
            Document::from(Text::from("Almighty God, we thank you that in your great love you have fed us with the spiritual food and drink of the Body and Blood of your Son Jesus Christ, and have given us a foretaste of your heavenly banquet. Grant that this Sacrament may be to us a comfort in affliction, and a pledge of our inheritance in that kingdom where there is no death, neither sorrow nor crying, but the fullness of joy with all your saints; through Jesus Christ our Savior.").response("Amen.").display_format(DisplayFormat::Unison)).tags([POSTCOMMUNION_PRAYER]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Commendation"))),
                Document::from(Rubric::from("The Celebrant and other ministers take their places at the body.\n\nThis anthem, or some other suitable anthem, or a hymn, may be sung or said.")),
                Document::from(ResponsivePrayer::from([
                    "Give rest, O Christ, to your servant(s) with your saints,",
                    "where sorrow and pain are no more,\nneither sighing, but life everlasting.\n",
                    "You only are immortal, the creator and maker of mankind; and we are mortal, formed of the earth, and to earth shall we return. For so did you ordain when you created me, saying, “You are dust, and to dust you shall return.” All of us go down to the dust, yet even at the grave we make our song: Alleluia, alleluia, alleluia.\n\nGive rest, O Christ, to your servant(s) with your saints,",
                    "where sorrow and pain are no more,\nneither sighing, but life everlasting.",
                ])),
                Document::from(Rubric::from("The minister, facing the body, says")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from("We commend *N.* to the mercy of God, our maker, redeemer, and comforter.\n\n*N.*, our companion in faith and fellow child of Christ, we entrust you to God. Go forth from this world in the love of God who created you, in the mercy of Jesus who died for you, in the power of the Holy Spirit who receives and protects you. May you rest in peace and rise in glory, where pain and grief are banished, and life and joy are yours for ever.").response("Amen.")),
                    Document::from(Text::from("Into your hands, O merciful Savior, we commend your servant *N.* Acknowledge, we humbly beseech you, a sheep of your own fold, a lamb of your own flock, a sinner of your own redeeming. Receive him into the arms of your mercy, into the blessed rest of everlasting peace, and into the glorious company of the saints in light.").response("Amen."))
                ])),
                Document::from(Rubric::from("The Blessing and Dismissal follow."))
            ])).tags([COMMENDATION]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Committal"))),
                Document::from(Rubric::from("One or more of the following anthems is sung or said")),
                Document::from(Sentence::from("They are before the throne of God,\nand worship him day and night within his temple,\nand the one who is seated on his throne will shelter them.\nThey will hunger no more and thirst no more;\nthe sun will not strike them, nor any scorching heat;\nfor the Lamb at the center of the throne will be their shepherd,\nand he will guide them to springs of the water of life,\nand God will wipe away every tear from their eyes.").citation("Revelation 7:15-17")),
                Document::from(Sentence::from("See, the home of God is among mortals. He will dwell with them\n\tas their God;\nthey will be his peoples, and God himself will be with them;\nhe will wipe away every tear from their eyes. Death will be no more;\nmourning and crying and pain will be no more, for the first\n\tthings have passed away.\nThose who conquer will inherit these things, and I will be their\n\tGod, and they will be my children.").citation("Revelation 21: 3b-4, 7")),
                Document::from(Rubric::from("Before the following prayer, the coffin may be lowered into the grave.")),
                Document::from(Rubric::from("Then, while earth is cast upon the coffin, the minister says these words")),
                Document::from(Text::from("In sure and certain hope of the resurrection to eternal life through our Lord Jesus Christ, we commend to Almighty God our *brother N.*, and we commit *his* body to the ground;\\* earth to earth, ashes to ashes, dust to dust. The Lord bless *him* and keep *him*, the Lord make his face to shine upon *him* and be gracious to *him*, the Lord lift up his countenance upon *him* and give *him* peace.").response("Amen.")),
                Document::from(Text::from("\\* *Or* the deep, *or* the elements, *or* its resting place.")),
                Document::from(Rubric::from("Then shall be sung or said")),
                Document::from(Sentence::from("Jesus said to his friends, “You have pain now; but I will see you again, and your hearts will rejoice, and no one will take your joy from you.”").citation("John 16:22")),
            ])).tags([COMMITTAL]),
            Document::from(Rubric::from("Then the minister says")).tags([COMMITTAL_PRAYERS]),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And also with you."),
                ("Minister", "Let us pray.")
            ])).tags([COMMITTAL_PRAYERS]),
            Document::from(Choice::from(vec![
                Document::from(Text::from("Loving God, we stand before you in pain and sadness. You gave the gift of new life, and now it has been taken from us. Hear the cry of our hearts for the pain of our loss. Be with us as we struggle to understand the mystery of life and death. Receive *N.* in the arms of your mercy, to live in your gracious and eternal love, and help us to commit ourselves to your tender care. In Jesus’ name we pray.").response("Amen.")),
                Document::from(Text::from("God, you have loved us into being. Hear our cries at our loss of *N.* Move us from the shadow of death into the light of your love and peace in the name of Mary’s child, Jesus the risen one.").response("Amen."))
            ])).tags([COMMITTAL_PRAYERS]),
            Document::from(Rubric::from("Here one or more of the additional prayers may be said. Then the Lord’s Prayer may be said.")).tags([COMMITTAL_PRAYERS]),
            Document::from(Content::DocumentLink { label: "Additional Prayers".into(), path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::EOW)]), rotate: false, link_only: false }).tags([COMMITTAL_PRAYERS]),
            Document::from(Choice::from(vec![
                Document::from(Series::from(vec![
                    Document::from(Text::from("As our Savior Christ\nhas taught us,\nwe now pray,\n")),
                    Document::from(Rubric::from("Officiant and People")),
                    Document::from(Text::from(LORDS_PRAYER_CONTEMPORARY_TEXT).response("Amen.").display_format(DisplayFormat::Unison))
                ])).version_label("Contemporary"),
                Document::from(Series::from(vec![
                    Document::from(Text::from("And now, as our Savior\nChrist has taught us,\nwe are bold to say,")),
                    Document::from(Rubric::from("Officiant and People")),
                    Document::from(Text::from(LORDS_PRAYER_TRADITIONAL_TEXT).response("Amen.").display_format(DisplayFormat::Unison))
                ])).version_label("Traditional"),
            ])).tags([COMMITTAL_LORDS_PRAYER]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Blessing follows.")),
                Document::from(Text::from("The God of peace, who brought again from the dead our Lord Jesus Christ, the great Shepherd of the sheep, through the blood of the everlasting covenant: Make you perfect in every good work to do his will, working in you that which is well-pleasing in his sight; through Jesus Christ, to whom be glory for ever and ever.").response("Amen."))
            ])).tags([BLESSING]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The service concludes with this Dismissal")),
                Document::from(Text::from("Since we believe that Jesus died and rose again, so will it be for those who have died: God will bring them to life with Jesus. Alleluia.\n\nGo in peace in the name of Christ."))
            ])).tags([DISMISSAL]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "Additional Prayers"))),
                Document::from(Content::DocumentLink { label: "Additional Prayers".into(), path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::EOW)]), rotate: false, link_only: false })
            ])).tags([ADDITIONAL_PRAYERS]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "Hymns Appropriate for the Burial of a Child"))),
                Document::from(HymnLink::Tag("Burial of a Child".to_string()))
            ]))
        ]))));

    pub static ref ADDITIONAL_PRAYERS_BURIAL_OF_A_CHILD: Vec<Document> = vec![
        Document::from(Text::from("God our Creator, you called into being this fragile life, which had seemed to us so full of promise: give to *N.*, whom we commit to your care, abundant life in your presence, and to us, who grieve for hopes destroyed, courage to bear our loss; through Jesus Christ our Savior.").response("Amen.")).label("The Death of an Infant"),
        Document::from(Text::from("O God, who gathered Rachel’s tears over her lost children, hear now the sorrow and distress of *N.* [and *N.*] for the death of *their/her/his* expected child; in the darkness of loss, stretch out to *them/her/him* the strength of your arm and renewed assurance of your love; through your own suffering and risen Child Jesus.").response("Amen.")).label("For a Miscarriage"),
        Document::from(Text::from("Heavenly Father, your love for all children is strong and enduring. We were not able to know *N.* as we hoped. Yet you knew *her/him* growing in *her/his* mother’s womb, and *she/he* is not lost to you. In the midst of our sadness, we thank you that *N.* is with you now.").response("Amen.")).label("For a Stillbirth or Child Who Dies Soon after Birth"),
        Document::from(Text::from("Loving God, we thank you that in your mercy you brought your daughter *N.* through childbirth in safety. We pray that *N.* [and *N.*] will know your support in this time of trouble and enjoy your protection always; through Jesus Christ our Savior.").response("Amen.")).label("For a Mother Whose Child has Died Near Birth"),
        Document::from(Text::from("").response("Amen.")).label(""),
        Document::from(Choice::from(vec![
            Document::from(Text::from("Loving God, Jesus gathered your little ones in his arms and blessed them. Have pity on those who mourn for *N.*, an innocent slaughtered by the violence of our fallen world. Be with us as we struggle with the mysteries of life and death; in our pain, bring your comfort, and in our sorrow, bring your hope and your promise of new life, in the name of Jesus our Savior.").response("Amen.")).label("For a Child Who Dies by Violence"),
            Document::from(Text::from("God our deliverer, gather our horror and pity for the death of your child *N.* into the compass of your wisdom and strength, that through the night we may seek and do what is right, and when morning comes trust ourselves to your cleansing justice and new life; through Christ our Savior.").response("Amen.")).label("For a Child Who Dies by Violence"),
            Document::from(Text::from("God, do not hide your face from us in our anger and grief for the death of *N.* Renew us in hope that your justice will roll down like mighty waters and joy spring up from the broken ground in a living stream; through Jesus our Savior.").response("Amen.")).label("For a Child Who Dies by Violence")
        ])),
        Document::from(Text::from("Holy God, we lift into the light of your justice *N.* [the one] who has taken the life of your child *N.* Where our hearts are stone return to us hearts of flesh; that grief may not swallow us up, but new life find us through Jesus the crucified, with whom we are raised by your power.").response("Amen.")).label("For One Who has Killed"),
        Document::from(Choice::from(vec![
            Document::from(Text::from("God of compassion and strength: keep safe the soul of your child *N.*, whose moment of pain and fear is past. Send your healing to *N.* [and *N.*] and all who mourn, that their suffering may find peace and resolution within your love, whose Spirit gives life in Christ our Savior.").response("Amen.")).label("For Those Who Mourn"),
            Document::from(Text::from("Most loving God: the death of your Son has opened to us a new and living way. Give us hope to overcome our despair; help us to surrender *N.* to your keeping, and let our sorrow find comfort in your care; through the name and presence of Jesus our Savior").response("Amen.")).label("For Those Who Mourn"),
            Document::from(Text::from("God, as Mary stood at the foot of the cross, we stand before you with broken hearts and tearful eyes. Keep us mindful that you know our pain, and free us to see your resurrection power already at work in *N.’s* life. In your time, raise us from our grief as you have raised *N.* to eternal life; through Jesus Christ our Savior.").response("Amen.")).label("For Those Who Mourn"),
            Document::from(Text::from("Merciful God, you grant to children an abundant entrance into your kingdom. In your compassion, comfort those who mourn for *N.*, and grant us grace to conform our lives to *her/his* innocence and faith, that at length, united with *her/him*, we may stand in your presence in the fullness of joy; for the sake of Jesus Christ.").response("Amen.")).label("For Those Who Mourn"),
        ])),

        Document::from(Choice::from(vec![
            Document::from(Text::from("Out of the depths we cry to you, merciful God, for your child *N.*, dead by *her/his* own hand. Meet our confusion with your peace, our anger with forgiveness, our guilt with mercy, and our sorrow with consolation. Help us acknowledge the mystery that our lives are hid with Christ in you, whose compassion is over all whom you have made. ").response("Amen.")).label("For a Child Dead by Suicide"),
            Document::from(Text::from("All-knowing and eternal God, come to our help as we mourn for *N.*, dead by *her/his* own hand. We know only in part, we love imperfectly, and we fail to ease one another’s pain as you intend. But you are the God whose property is always to have mercy, and so we put our trust in you and ask the courage to go on; through our Savior Christ, who suffered for us, and whom you raised to new life.").response("Amen.")).label("For a Child Dead by Suicide")
        ]))
    ];
}
