use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref PRAYER_I: Document = Document::new()
        .label("Eucharistic Prayer I")
        .version_label("Prayer I")
        .page(333)
        .version(Version::RiteI)
        .content(Series::from(vec![
            Document::from(Rubric::from("The people remain standing. The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And with thy spirit."),
                ("Celebrant", "Lift up your hearts."),
                ("People", "We lift them up unto the Lord."),
                ("Celebrant", "Let us give thanks unto our Lord God."),
                ("People", "It is meet and right so to do.")
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(Text::from("It is very meet, right, and our bounden duty, that we should at all times, and in all places, give thanks unto thee, O Lord, holy Father, almighty, everlasting God.")),
            Document::from(Rubric::from("Here a Proper Preface is sung or said on all Sundays, and on other occasions as appointed.")),
            Document::from(Content::DocumentLink {
                label: "Proper Prefaces".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Version(Version::RiteI)]),
                rotate: true,
                link_only: false
            }),
            Document::from(Text::from("Therefore with Angels and Archangels, and with all the company of heaven, we laud and magnify thy glorious Name; evermore praising thee, and saying,")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy, Lord God of Hosts:\nHeaven and earth are full of thy glory.\nGlory be to thee, O Lord Most High.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("Here may be added")),
            Document::from(Text::from("Blessed is he that cometh in the name of the Lord.\nHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The people kneel or stand.\n\nThen the Celebrant continues")),
            Document::from(Text::from("All glory be to thee, Almighty God, our heavenly Father, for that thou, of thy tender mercy, didst give thine only Son Jesus Christ to suffer death upon the cross for our redemption; who made there, by his one oblation of himself once offered, a full, perfect, and sufficient sacrifice, oblation, and satisfaction, for the sins of the whole world; and did institute, and in his holy Gospel command us to continue, a perpetual memory of that his precious death and sacrifice, until his coming again.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it, or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated")),
            Document::from(Text::from("For in the night in which he was betrayed, he took bread; and when he had given thanks, he brake it, and gave it to his disciples, saying, “Take, eat, this is my Body, which is given for you. Do this in remembrance of me.”\n\nLikewise, after supper, he took the cup; and when he had given thanks, he gave it to them, saying, “Drink ye all of this; for this is my Blood of the New Testament, which is shed for you, and for many, for the remission of sins. Do this, as oft as ye shall drink it, in remembrance of me.”\n\nWherefore, O Lord and heavenly Father, according to the institution of thy dearly beloved Son our Savior Jesus Christ, we, thy humble servants, do celebrate and make here before thy divine Majesty, with these thy holy gifts, which we now offer unto thee, the memorial thy Son hath commanded us to make; having in remembrance his blessed passion and precious death, his mighty resurrection and glorious ascension; rendering unto thee most hearty thanks for the innumerable benefits procured unto us by the same.\n\nAnd we most humbly beseech thee, O merciful Father, to hear us; and, of thy almighty goodness, vouchsafe to bless and sanctify, with thy Word and Holy Spirit, these thy gifts and creatures of bread and wine; that we, receiving them according to thy Son our Savior Jesus Christ’s holy institution, in remembrance of his death and passion, may be partakers of his most blessed Body and Blood.\n\nAnd we earnestly desire thy fatherly goodness mercifully to accept this our sacrifice of praise and thanksgiving; most humbly beseeching thee to grant that, by the merits and death of thy Son Jesus Christ, and through faith in his blood, we, and all thy whole Church, may obtain remission of our sins, and all other benefits of his passion.\n\nAnd here we offer and present unto thee, O Lord, our selves, our souls and bodies, to be a reasonable, holy, and living sacrifice unto thee; humbly beseeching thee that we, and all others who shall be partakers of this Holy Communion, may worthily receive the most precious Body and Blood of thy Son Jesus Christ, be filled with thy grace and heavenly benediction, and made one body with him, that he may dwell in us, and we in him.\n\nAnd although we are unworthy, through our manifold sins, to offer unto thee any sacrifice, yet we beseech thee to accept this our bounden duty and service, not weighing our merits, but pardoning our offenses, through Jesus Christ our Lord;\n\nBy whom, and with whom, in the unity of the Holy Ghost, all honor and glory be unto thee, O Father Almighty, world without end.").response("AMEN."))
    ]));

    pub static ref PRAYER_II: Document = Document::new()
        .label("Eucharistic Prayer II")
        .version_label("Prayer II")
        .page(340)
        .version(Version::RiteI)
        .content(Series::from(vec![
            Document::from(Rubric::from("The people remain standing. The Celebrant, whether bishop or priest faces them and sings or says")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And with thy spirit."),
                ("Celebrant", "Lift up your hearts."),
                ("People", "We lift them up unto the Lord."),
                ("Celebrant", "Let us give thanks unto our Lord God."),
                ("People", "It is meet and right so to do.")
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(Text::from("It is very meet, right, and our bounden duty, that we should at all times, and in all places, give thanks unto thee, O Lord, holy Father, almighty, everlasting God.")),
            Document::from(Rubric::from("Here a Proper Preface is sung or said on all Sundays, and on other occasions as appointed.")),
            Document::from(Content::DocumentLink {
                label: "Proper Prefaces".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Version(Version::RiteI)]),
                rotate: true,
                link_only: false
            }),
            Document::from(Text::from("Therefore with Angels and Archangels, and with all the company of heaven, we laud and magnify thy glorious Name; evermore praising thee, and saying,")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy, Lord God of Hosts:\nHeaven and earth are full of thy glory.\nGlory be to thee, O Lord Most High.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("Here may be added")),
            Document::from(Text::from("Blessed is he that cometh in the name of the Lord.\nHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The people kneel or stand.\n\nThen the Celebrant continues")),
            Document::from(Text::from("All glory be to thee, O Lord our God, for that thou didst create heaven and earth, and didst make us in thine own image; and, of thy tender mercy, didst give thine only Son Jesus Christ to take our nature upon him, and to suffer death upon the cross for our redemption. He made there a full and perfect sacrifice for the whole world; and did institute, and in his holy Gospel command us to continue, a perpetual memory of that his precious death and sacrifice, until his coming again.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it, or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated.")),
            Document::from(Text::from("For in the night in which he was betrayed, he took bread; and when he had given thanks to thee, he broke it, and gave it to his disciples, saying, “Take, eat, this is my Body, which is given for you. Do this in remembrance of me.”\n\nLikewise, after supper, he took the cup; and when he had given thanks, he gave it to them, saying, “Drink this, all of you; for this is my Blood of the New Covenant, which is shed for you, and for many, for the remission of sins. Do this, as oft as ye shall drink it, in remembrance of me.”\n\nWherefore, O Lord and heavenly Father, we thy people do celebrate and make, with these thy holy gifts which we now offer unto thee, the memorial thy Son hath commanded us to make; having in remembrance his blessed passion and precious death, his mighty resurrection and glorious ascension; and looking for his coming again with power and great glory.\n\nAnd we most humbly beseech thee, O merciful Father, to hear us, and, with thy Word and Holy Spirit, to bless and sanctify these gifts of bread and wine, that they may be unto us the Body and Blood of thy dearly-beloved Son Jesus Christ.\n\nAnd we earnestly desire thy fatherly goodness to accept this our sacrifice of praise and thanksgiving, whereby we offer and present unto thee, O Lord, our selves, our souls and bodies. Grant, we beseech thee, that all who partake of this Holy Communion may worthily receive the most precious Body and Blood of thy Son Jesus Christ, and be filled with thy grace and heavenly benediction; and also that we and all thy whole Church may be made one body with him, that he may dwell in us, and we in him; through the same Jesus Christ our Lord;\n\nBy whom, and with whom, and in whom, in the unity of the Holy Ghost all honor and glory be unto thee, O Father Almighty, world without end.").response("AMEN."))
    ]));
}
