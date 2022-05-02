use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref PRAYERS_OF_THE_PEOPLE: Vec<Document> = vec![
        pop::FORM_I.clone(),
        pop::FORM_II.clone(),
        pop::FORM_III.clone(),
        pop::FORM_IV.clone(),
        pop::FORM_V.clone(),
        pop::FORM_V_KYRIE.clone(),
        pop::FORM_VI.clone()
    ];

    pub static ref CONSECRATING_ADDITIONAL: Document = Document::from(Series::from(vec![
        Document::from(Rubric::from("If the consecrated Bread or Wine does not suffice for the number of communicants, the celebrant is to return to the Holy Table, and consecrate more of either or both, by saying").long()),
        Document::from(Text::from("Hear us, O heavenly Father, and with thy (your) Word and Holy Spirit bless and sanctify this bread (wine) that it, also, may be the Sacrament of the precious Body (Blood) of thy (your) Son Jesus Christ our Lord, who took bread (the cup) and said, “This is my Body (Blood).”").response("Amen.")),
        Document::from(Rubric::from("or else the celebrant may consecrate more of both kinds, saying again the words of the Eucharistic Prayer, beginning with the words which follow the Sanctus, and ending with the Invocation (in the case of Eucharistic Prayer C, ending with the narrative of the Institution).").long())
    ])).label("Form for Consecrating Additional Bread and Wine");
}

pub mod pop {
    use liturgy::*;

    lazy_static! {
        pub static ref FORM_I: Document = Document::new()
            .label("Form I")
            .content(Content::Series(Series::from(vec![
            Document::from(Rubric::from("Deacon or other leader")),
            Document::from("With all our heart and with all our mind, let us pray to the Lord, saying, “Lord, have mercy.”"),
            Document::from(Litany::from((
                "Lord, have mercy.",
                [
                    "| For the peace from above, for the loving kindness of God, and for the salvation of our souls, let us pray to the Lord.",
                    "For the peace of the world, for the welfare of the holy Church of God, and for the unity of all peoples, let us pray to the Lord.",
                    "For our Bishop, and for all the clergy and people, let us pray to the Lord.",
                    "For our President, for the leaders of the nations, and for all in authority, let us pray to the Lord.",
                    "For this city (town, village, ___________), for every city and community, and for those who live in them, let us pray to the Lord.",
                    "| For seasonable weather, and for an abundance of the fruits of the earth, let us pray to the Lord.",
                    "For the good earth which God has given us, and for the wisdom and will to conserve it, let us pray to the Lord.",
                    "For those who travel on land, on water, or in the air [or through outer space], let us pray to the Lord.",
                    "For the aged and infirm, for the widowed and orphans, and for the sick and the suffering, let us pray to the Lord.",
                    "For ___________, let us pray to the Lord.",
                    "For the poor and the oppressed, for the unemployed and the destitute, for prisoners and captives, and for all who remember and care for them, let us pray to the Lord.",
                    "For all who have died in the hope of the resurrection, and for all the departed, let us pray to the Lord.",
                    "For deliverance from all danger, violence, oppression, and degradation, let us pray to the Lord.",
                    "| For the absolution and remission of our sins and offenses, let us pray to the Lord.",
                    "That we may end our lives in faith and hope, without suffering and without reproach, let us pray to the Lord.",
                    "| Defend us, deliver us, and in thy compassion protect us, O Lord, by thy grace.",
                ]
            ))),
            Document::from(Litany::from((
                "To thee, O Lord our God.",
                ["In the communion of [___________ and of all the] saints, let us commend ourselves, and one another, and all our life, to Christ our God."]
            ))),
            Document::from(Rubric::from("Silence")),
            Document::from(Rubric::from("The Celebrant adds a concluding Collect."))
        ])))
        .page(383);

        pub static ref FORM_II: Document = Document::new()
            .label("Form II")
            .content(Content::Series(Series::from(vec![
            Document::from(Rubric::from("In the course of the silence after each bidding, the People offer their own prayers, either silently or aloud.")),
            Document::from("I ask your prayers for God’s people throughout the world; for our Bishop(s) ___________ ; for this gathering; and for all ministers and people.\nPray for the Church."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for peace; for goodwill among nations; and for the well-being of all people.\nPray for justice and peace."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for the poor, the sick, the hungry, the oppressed, and those in prison.\nPray for those in any need or trouble."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for all who seek God, or a deeper knowledge of him.\nPray that they may find and be found by him."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for the departed [especially __________ ].\nPray for those who have died."),
            Document::from(Rubric::from("Silence")),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("Members of the congregation may ask the prayers or the thanksgivings of those present")),
                Document::from("I ask your prayers for ____________.\n\nI ask your thanksgiving for _____________."),
                Document::from(Rubric::from("Silence."))
            ])).optional(),
            Document::from("Praise God for those in every generation in whom Christ has been honored [especially ____________whom we remember today].\nPray that we may have grace to glorify Christ in our own day."),
            Document::from(Rubric::from("Silence")),
            Document::from(Rubric::from("The Celebrant adds a concluding Collect."))
        ]))).page(385);

        pub static ref FORM_III: Document = Document::new()
            .label("Form III")
            .content(Content::Series(Series::from(vec![
            Document::from(Rubric::from("The Leader and People pray responsively")),
            Document::from(ResponsivePrayer::from([
                "Father, we pray for your holy Catholic Church;",
                "That we all may be one.\n",
                "Grant that every member of the Church may truly and humbly serve you;",
                "That your Name may be glorified by all people.\n",
                "We pray for all bishops, priests, and deacons;",
                "That they may be faithful ministers of your Word and Sacraments.\n",
                "We pray for all who govern and hold authority in the nations of the world;",
                "That there may be justice and peace on the earth.\n",
                "Give us grace to do your will in all that we undertake;",
                "That our works may find favor in your sight.\n",
                "Have compassion on those who suffer from any grief or trouble;",
                "That they may be delivered from their distress.\n",
                "Give to the departed eternal rest;",
                "Let light perpetual shine upon them.\n",
                "We praise you for your saints who have entered into joy;",
                "May we also come to share in your heavenly kingdom."
            ])),
            Document::from(Text::from("Let us pray for our own needs and those of others.")),
            Document::from(Rubric::from("Silence\n\nThe People may add their own petitions.\n\nThe Celebrant adds a concluding Collect")),
        ]))).page(387);

        pub static ref FORM_IV: Document = Document::new()
            .label("Form IV")
            .content(Content::Series(Series::from(vec![
            Document::from(Rubric::from("Deacon or other leader")),
            Document::from(Text::from("Let us pray for the Church and for the world.")),
            Document::from(Text::from("Grant, Almighty God, that all who confess your Name may be united in your truth, live together in your love, and reveal your glory in the world.")),
            Document::from(Rubric::from("Silence")),
            Document::from(ResponsivePrayer::from([
                "Lord, in your mercy",
                "Hear our prayer."
            ])),
            Document::from(Text::from("Guide the people of this land, and of all the nations, in the ways of justice and peace; that we may honor one another and serve the common good.")),
            Document::from(Rubric::from("Silence")),
            Document::from(ResponsivePrayer::from([
                "Lord, in your mercy",
                "Hear our prayer."
            ])),
            Document::from(Text::from("Give us all a reverence for the earth as your own creation, that we may use its resources rightly in the service of others and to your honor and glory.")),
            Document::from(Rubric::from("Silence")),
            Document::from(ResponsivePrayer::from([
                "Lord, in your mercy",
                "Hear our prayer."
            ])),
            Document::from(Text::from("Bless all whose lives are closely linked with ours, and grant that we may serve Christ in them, and love one another as he loves us.")),
            Document::from(Rubric::from("Silence")),
            Document::from(ResponsivePrayer::from([
                "Lord, in your mercy",
                "Hear our prayer."
            ])),
            Document::from(Text::from("Comfort and heal all those who suffer in body, mind, or spirit; give them courage and hope in their troubles, and bring them the joy of your salvation.")),
            Document::from(Rubric::from("Silence")),
            Document::from(ResponsivePrayer::from([
                "Lord, in your mercy",
                "Hear our prayer."
            ])),
            Document::from(Text::from("We commend to your mercy all who have died, that your will for them may be fulfilled; and we pray that we may share with all your saints in your eternal kingdom.")),
            Document::from(Rubric::from("Silence")),
            Document::from(ResponsivePrayer::from([
                "Lord, in your mercy",
                "Hear our prayer."
            ])),
            Document::from(Rubric::from("The Celebrant adds a concluding Collect."))
        ]))).page(388);

        pub static ref FORM_V: Document = Document::new()
            .label("Form V")
            .content(Content::Series(Series::from(vec![
            Document::from(Rubric::from("Deacon or other leader")),
            Document::from(Text::from("In peace, let us pray to the Lord, saying, “Lord, have mercy.”")),
            Document::from(Litany::from((
                "Lord, have mercy.",
                [
                    "For the holy Church of God, that it may be filled with truth and love, and be found without fault at the day of your coming, we pray to you, O Lord.",
                    "For *N.* our Presiding Bishop, for *N.* (*N.*) our own Bishop(s), for all bishops and other ministers, and for all the holy people of God, we pray to you, O Lord.",
                    "For all who fear God and believe in you, Lord Christ, that our divisions may cease, and that all may be one as you and the Father are one, we pray to you, O Lord.",
                    "For the mission of the Church, that in faithful witness it may preach the Gospel to the ends of the earth, we pray to you, O Lord.",
                    "| For those who do not yet believe, and for those who have lost their faith, that they may receive the light of the Gospel, we pray to you, O Lord.",
                    "For the peace of the world, that a spirit of respect and forbearance may grow among nations and peoples, we pray to you, O Lord.",
                    "For those in positions of public trust [especially _________ ], that they may serve justice, and promote the dignity and freedom of every person, we pray to you, O Lord.",
                    "| For all who live and work in this community [especially _____________ ], we pray to you, O Lord.",
                    "| For a blessing upon all human labor, and for the right use of the riches of creation, that the world may be freed from poverty, famine, and disaster, we pray to you, O Lord.",
                    "For the poor, the persecuted, the sick, and all who suffer; for refugees, prisoners, and all who are in danger; that they may be relieved and protected, we pray to you, O Lord.",
                    "For this *congregation* [for those who are present, and for those who are absent], that we may be delivered from hardness of heart, and show forth your glory in all that we do, we pray to you, O Lord.",
                    "| For our enemies and those who wish us harm; and for all whom we have injured or offended, we pray to you, O Lord.",
                    "| For ourselves; for the forgiveness of our sins, and for the grace of the Holy Spirit to amend our lives, we pray to you, O Lord.",
                    "For all who have commended themselves to our prayers; for our families, friends, and neighbors; that being freed from anxiety, they may live in joy, peace, and health, we pray to you, O Lord.",
                    "| For ___________, we pray to you, O Lord.",
                    "For all who have died in the communion of your Church, and those whose faith is known to you alone, that, with all the saints, they may have rest in that place where there is no pain or grief, but life eternal, we pray to you, O Lord."
                ]
            ))),
            Document::from(ResponsivePrayer::from([
                "Rejoicing in the fellowship of [the ever-blessed Virgin Mary, (*blessed N.*) and] all the saints, let us commend ourselves, and one another, and all our life to Christ our God.",
                "To you, O Lord our God."
            ])),
            Document::from(Rubric::from("Silence")),
            Document::from(Rubric::from("The Celebrant adds a concluding Collect, or the following Doxology")),
            Document::from(Text::from("For yours is the majesty, O Father, Son, and Holy Spirit; yours is the kingdom and the power and the glory, now and for ever.").response("Amen."))
        ]))).version_label("Form V (Lord, have mercy.)");

        pub static ref FORM_V_KYRIE: Document = Document::new()
            .label("Form V")
            .content(Content::Series(Series::from(vec![
            Document::from(Rubric::from("Deacon or other leader")),
            Document::from(Text::from("In peace, let us pray to the Lord, saying, Kyrie, eleison.”")),
            Document::from(Litany::from((
                "Kyrie, eleison.",
                [
                    "For the holy Church of God, that it may be filled with truth and love, and be found without fault at the day of your coming, we pray to you, O Lord.",
                    "For *N.* our Presiding Bishop, for *N.* (*N.*) our own Bishop(s), for all bishops and other ministers, and for all the holy people of God, we pray to you, O Lord.",
                    "For all who fear God and believe in you, Lord Christ, that our divisions may cease, and that all may be one as you and the Father are one, we pray to you, O Lord.",
                    "For the mission of the Church, that in faithful witness it may preach the Gospel to the ends of the earth, we pray to you, O Lord.",
                    "| For those who do not yet believe, and for those who have lost their faith, that they may receive the light of the Gospel, we pray to you, O Lord.",
                    "For the peace of the world, that a spirit of respect and forbearance may grow among nations and peoples, we pray to you, O Lord.",
                    "For those in positions of public trust [especially _________ ], that they may serve justice, and promote the dignity and freedom of every person, we pray to you, O Lord.",
                    "| For all who live and work in this community [especially _____________ ], we pray to you, O Lord.",
                    "| For a blessing upon all human labor, and for the right use of the riches of creation, that the world may be freed from poverty, famine, and disaster, we pray to you, O Lord.",
                    "For the poor, the persecuted, the sick, and all who suffer; for refugees, prisoners, and all who are in danger; that they may be relieved and protected, we pray to you, O Lord.",
                    "For this *congregation* [for those who are present, and for those who are absent], that we may be delivered from hardness of heart, and show forth your glory in all that we do, we pray to you, O Lord.",
                    "| For our enemies and those who wish us harm; and for all whom we have injured or offended, we pray to you, O Lord.",
                    "| For ourselves; for the forgiveness of our sins, and for the grace of the Holy Spirit to amend our lives, we pray to you, O Lord.",
                    "For all who have commended themselves to our prayers; for our families, friends, and neighbors; that being freed from anxiety, they may live in joy, peace, and health, we pray to you, O Lord.",
                    "| For ___________, we pray to you, O Lord.",
                    "For all who have died in the communion of your Church, and those whose faith is known to you alone, that, with all the saints, they may have rest in that place where there is no pain or grief, but life eternal, we pray to you, O Lord."
                ]
            ))),
            Document::from(ResponsivePrayer::from([
                "Rejoicing in the fellowship of [the ever-blessed Virgin Mary, (*blessed N.*) and] all the saints, let us commend ourselves, and one another, and all our life to Christ our God.",
                "To you, O Lord our God."
            ])),
            Document::from(Rubric::from("Silence")),
            Document::from(Rubric::from("The Celebrant adds a concluding Collect, or the following Doxology")),
            Document::from(Text::from("For yours is the majesty, O Father, Son, and Holy Spirit; yours is the kingdom and the power and the glory, now and for ever.").response("Amen."))
        ]))).version_label("Form V (Kyrie, eleison.)");

        pub static ref FORM_VI: Document = Document::new()
            .label("Form VI")
            .content(Content::Series(Series::from(vec![
            Document::from(Rubric::from("The Leader and People pray responsively")),
            Document::from(Text::from("In peace, we pray to you, Lord God.")),
            Document::from(Rubric::from("Silence")),
            Document::from(ResponsivePrayer::from([
                "For all people in their daily life and work;",
                "For our families, friends, and neighbors, and for those who are alone.\n ",
                "For this community, the nation, and the world;",
                "For all who work for justice, freedom, and peace.\n ",
                "For the just and proper use of your creation;",
                "For the victims of hunger, fear, injustice, and oppression.\n ",
                "For all who are in danger, sorrow, or any kind of trouble;",
                "For those who minister to the sick, the friendless, and the needy.\n ",
                "For the peace and unity of the Church of God;",
                "For all who proclaim the Gospel, and all who seek the Truth.\n ",
                "For [N. our Presiding Bishop, and N. (N.) our Bishop(s); and for] all bishops and other ministers;",
                "For all who serve God in his Church.\n "
            ])),
            Document::from(Text::from("For the special needs and concerns of this congregation.")),
            Document::from(Rubric::from("Silence\n\nThe People may add their own petitions")),
            Document::from(ResponsivePrayer::from([
                "Hear us, Lord;",
                "For your mercy is great."
            ])),
            Document::from(Text::from("We thank you, Lord, for all the blessings of this life.")),
            Document::from(Rubric::from("Silence\n\nThe People may add their own thanksgivings")),
            Document::from(ResponsivePrayer::from([
                "We will exalt you, O God our King;",
                "And praise your Name for ever and ever."
            ])),
            Document::from(Text::from("We pray for all who have died, that they may have a place in your eternal kingdom.")),
            Document::from(Rubric::from("Silence\n\nThe People may add their own petitions")),
            Document::from(ResponsivePrayer::from([
                "Lord, let your loving-kindness be upon them;",
                "Who put their trust in you."
            ])),
            Document::from(Series::from(vec![
                Document::from(Text::from("We pray to you also for the forgiveness of our sins.")),
                Document::from(Rubric::from("Silence may be kept.\n\nLeader and People")),
                Document::from(Text::from("Have mercy upon us, most merciful Father;\nin your compassion forgive us our sins,\nknown and unknown,\nthings done and left undone;\nand so uphold us by your Spirit\nthat we may live and serve you in newness of life,\nto the honor and glory of your Name;\nthrough Jesus Christ our Lord.").response("Amen.").display_format(DisplayFormat::Unison))
            ])).optional(),
            Document::from(Rubric::from("The Celebrant concludes with an absolution or a suitable Collect."))
        ])));
    }
}
