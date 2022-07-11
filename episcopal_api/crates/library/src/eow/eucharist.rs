use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref PRAYER_1: Document = Document::new()
        .label("Eucharistic Prayer 1")
        .version_label("Prayer 1")
        .source(Reference {
            source: Source::EOW1,
            page: 57
        })
        .version(Version::EOW)
        .content(Series::from(vec![
            Document::from(Text::from("")),
            Document::from(Preces::from([
                ("Celebrant", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Lift up your hearts."),
                ("People", "We lift them to the Lord."),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("People", "It is right to give our thanks and praise.\n"),
                ("Celebrant", "It is truly right, and good and joyful, to give you thanks, all-holy God, source of life and fountain of mercy.")
            ])),
            Document::from(Rubric::from("The following Preface may be used at any time.")),
            Document::from(Text::from("You have filled us and all creation with your blessing\nand fed us with your constant love;\nyou have redeemed us in Jesus Christ\nand knit us into one body.\nThrough your Spirit you replenish us\nand call us to fullness of life.")),
            Document::from(Rubric::from("In place of the preceding, a Proper Preface from the Book of Common Prayer may be used.")),
            Document::from(Content::DocumentLink {
                label: "Proper Prefaces".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Version(Version::RiteII)]),
                rotate: true,
                link_only: false
            }),
            Document::from(Text::from("Therefore, joining with Angels and Archangels\nand with the faithful of every generation,\nwe lift our voices with all creation as we sing (say):")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is the one who comes in the name of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("Blessed are you, gracious God,\ncreator of the universe and giver of life.\nYou formed us in your own image\nand called us to dwell in your infinite love.\nYou gave the world into our care\nthat we might be your faithful stewards\nand show forth your bountiful grace.\n\nBut we failed to honor your image\nin one another and in ourselves;\nwe would not see your goodness in the world around us;\nand so we violated your creation,\nabused one another,\nand rejected your love.\nYet you never ceased to care for us,\nand prepared the way of salvation for all people.\n\nThrough Abraham and Sarah\nyou called us into covenant with you.\nYou delivered us from slavery,\nsustained us in the wilderness,\nand raised up prophets\nto renew your promise of salvation.\nThen, in the fullness of time,\nyou sent your eternal Word,\nmade mortal flesh in Jesus.\nBorn into the human family,\nand dwelling among us,\nhe revealed your glory.\nGiving himself freely to death on the cross,\nhe triumphed over evil,\nopening the way of freedom and life.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it, or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated.")),
            Document::from(Text::from("On the night before he died for us,\nOur Savior Jesus Christ took bread,\nand when he had given thanks to you,\nhe broke it, and gave it to his friends, and said:\n“Take, eat:\nThis is my Body which is given for you.\nDo this for the remembrance of me.”\n\nAs supper was ending, Jesus took the cup of wine,\nand when he had given thanks,\nhe gave it to them, and said:\n“Drink this, all of you:\nThis is my Blood of the new Covenant,\nwhich is poured out for you and for all\nfor the forgiveness of sins.\nWhenever you drink it,\ndo this for the remembrance of me.”\n\nTherefore we proclaim the mystery of faith:")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Christ has died.\nChrist is risen.\nChrist will come again.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("\n\nRemembering his death and resurrection,\nwe now present to you from your creation\nthis bread and this wine.\nBy your Holy Spirit may they be for us\nthe Body and Blood of our Savior Jesus Christ.\nGrant that we who share these gifts\nmay be filled with the Holy Spirit\nand live as Christ’s Body in the world.\nBring us into the everlasting heritage\nof your daughters and sons,\nthat with [ ___________ and] all your saints,\npast, present, and yet to come,\nwe may praise your Name for ever.\n\nThrough Christ and with Christ and in Christ,\nin the unity of the Holy Spirit,\nto you be honor, glory, and praise,\nfor ever and ever.").response("AMEN."))
    ]));

    pub static ref PRAYER_2: Document = Document::new()
        .label("Eucharistic Prayer 2")
        .version_label("Prayer 2")
        .source(Reference {
                    source: Source::EOW1,
                    page: 60
                })
        .version(Version::EOW)
        .content(Series::from(vec![
            Document::from(Preces::from([
                ("Celebrant", "The Lord be with you."),
                ("People", "And also with you."),
                ("Celebrant", "Lift up your hearts."),
                ("People", "We lift them to the Lord."),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("People", "It is right to give our thanks and praise.")
            ])),
            Document::from(Rubric::from("Celebrant")),
            Document::from(Text::from("We praise you and we bless you, holy and gracious God,\nsource of life abundant.\nFrom before time you made ready the creation.\nYour Spirit moved over the deep\nand brought all things into being:\nsun, moon, and stars;\nearth, winds, and waters;\nand every living thing.\nYou made us in your image,\nand taught us to walk in your ways.\nBut we rebelled against you, and wandered far away;\nand yet, as a mother cares for her children,\nyou would not forget us.\nTime and again you called us\nto live in the fullness of your love.\n\nAnd so this day we join with Saints and Angels\nin the chorus of praise that rings through eternity,\nlifting our voices to magnify you as we sing (say):")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is the one who comes in the name of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("Glory and honor and praise to you, holy and living God.\nTo deliver us from the power of sin and death\nand to reveal the riches of your grace,\nyou looked with favor upon Mary, your willing servant,\nthat she might conceive and bear a son,\nJesus the holy child of God.\nLiving among us, Jesus loved us.\nHe broke bread with outcasts and sinners,\nhealed the sick, and proclaimed good news to the poor.\nHe yearned to draw all the world to himself\nyet we were heedless of his call to walk in love.\nThen, the time came for him to complete upon the cross\nthe sacrifice of his life,\nand to be glorified by you.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it, or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing the wine to be consecrated.")),
            Document::from(Text::from("On the night before he died for us,\nJesus was at table with his friends.\nHe took bread, gave thanks to you,\nbroke it, and gave it to them, and said:\n“Take, eat:\nThis is my Body, which is given for you.\nDo this for the remembrance of me.”\nAs supper was ending, Jesus took the cup of wine.\nAgain, he gave thanks to you,\ngave it to them, and said:\n“Drink this, all of you:\nThis is my Blood of the new Covenant,\nwhich is poured out for you and for all\nfor the forgiveness of sins.\nWhenever you drink it,\ndo this for the remembrance of me.”\n\nNow gathered at your table, O God of all creation,\nand remembering Christ, crucified and risen,\nwho was and is and is to come,\nwe offer to you our gifts of bread and wine,\nand ourselves, a living sacrifice.\n\nPour out your Spirit upon these gifts\nthat they may be the Body and Blood of Christ.\nBreathe your Spirit over the whole earth\nand make us your new creation,\nthe Body of Christ given for the world you have made.\n\nIn the fullness of time bring us,\nwith [___________ and] all your saints,\nfrom every tribe and language and people and nation,\nto feast at the banquet prepared\nfrom the foundation of the world.\n\nThrough Christ and with Christ and in Christ,\nin the unity of the Holy Spirit,\nto you be honor, glory, and praise,\nfor ever and ever.").response("AMEN."))
    ]));

    pub static ref PRAYER_3: Document = Document::new()
        .label("Eucharistic Prayer 3")
        .version_label("Prayer 3")
        .source(Reference {
            source: Source::EOW1,
            page: 62
        })
        .version(Version::EOW)
        .content(Series::from(vec![
            Document::from(Preces::from([
                ("Presider", "The Lord be with you."),
                ("People", "And also with you."),
                ("Presider", "Lift up your hearts."),
                ("People", "We lift them to the Lord."),
                ("Presider", "Let us give thanks to the Lord our God."),
                ("People", "It is right to give our thanks and praise.")
            ])),
            Document::from(Rubric::from("Presider")),
            Document::from(Text::from("All thanks and praise\nare yours at all times and in all places,\nour true and loving God;\nthrough Jesus Christ, your eternal Word,\nthe Wisdom from on high by whom you created all things.\nYou laid the foundations of the world\nand enclosed the sea when it burst out from the womb;\nYou brought forth all creatures of the earth\nand gave breath to humankind.\n\nWondrous are you, Holy One of Blessing,\nall you create is a sign of hope for our journey;\nAnd so as the morning stars sing your praises\nwe join the heavenly beings and all creation\nas we shout with joy:")),
            Document::from(Rubric::from("Presider and People")),
            Document::from(Choice::from(vec![
                        Document::new()	.version_label("“Blessed is the one”")
                            .content(Text::from("Holy, holy, holy Lord,\nGod of power and might,\nheaven and earth\nare full of your glory.\n\tHosanna in the highest.\nBlessed is the one\nwho comes in the\nname of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison)),
                        Document::new()	.version_label("”Blessed is he”")
                            .content(Text::from("Holy, holy, holy Lord,\nGod of power and might,\nheaven and earth\nare full of your glory.\n\tHosanna in the highest.\nBlessed is he\nwho comes in the\nname of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison))
            ])),
            Document::from(Rubric::from("The Presider continues")),
            Document::from(Text::from("Glory and honor are yours, Creator of all,\nyour Word has never been silent;\nyou called a people to yourself, as a light to the nations,\nyou delivered them from bondage\nand led them to a land of promise.\nOf your grace, you gave Jesus\nto be human, to share our life,\nto proclaim the coming of your holy reign\nand give himself for us, a fragrant offering.\n\nThrough Jesus Christ our Redeemer,\nyou have freed us from sin,\nbrought us into your life,\nreconciled us to you,\nand restored us to the glory you intend for us.\n\nWe thank you that on the night before he died for us\nJesus took bread,\nand when he had given thanks to you, he broke it,\ngave it to his friends and said:\n“Take, eat, this is my Body, broken for you.\nDo this for the remembrance of me.”\n\nAfter supper Jesus took the cup of wine,\nsaid the blessing, gave it to his friends and said:\n“Drink this, all of you:\nthis cup is the new Covenant in my Blood,\npoured out for you and for all\nfor the forgiveness of sin.\nDo this for the remembrance of me.”\n\nAnd so, remembering all that was done for us:\nthe cross, the tomb, the resurrection and ascension,\nlonging for Christ’s coming in glory,\nand presenting to you these gifts\nyour earth has formed and human hands have made,\nwe acclaim you, O Christ:")),
            Document::from(Rubric::from("Presider and People")),
            Document::from(Text::from("Dying, you destroyed our death.\nRising, you restored our life.\nChrist Jesus, come in glory!").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Presider continues")),
            Document::from(Text::from("Send your Holy Spirit upon us\nand upon these gifts of bread and wine\nthat they may be to us\nthe Body and Blood of your Christ.\nGrant that we, burning with your Spirit’s power,\nmay be a people of hope, justice and love.\n\nGiver of Life, draw us together in the Body of Christ,\nand in the fullness of time gather us\nwith [blessed _______, and] all your people\ninto the joy of our true eternal home.\nThrough Christ and with Christ and in Christ,\nby the inspiration of your Holy Spirit,\nwe worship you our God and Creator\nin voices of unending praise.")),
            Document::from(Rubric::from("Presider and People")),
            Document::from(Text::from("Blessed are you now and for ever. AMEN.").display_format(DisplayFormat::Unison))
    ]));
}
