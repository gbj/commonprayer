use calendar::Season;
use liturgy::{
    Choice, Condition, Content, Document, Heading, HeadingLevel, HymnLink, Liturgy, Preces, Rubric,
    Sentence, Series, Show, Text, Version,
};

use crate::{
    conditions::{EASTER_SEASON, LENT},
    rite2::evening_prayer::PHOS_HILARON,
};

lazy_static! {
    pub static ref AN_ORDER_OF_WORSHIP_FOR_EVENING: Document = Document::from(Liturgy::from(Series::from([
        Document::from(Heading::from((HeadingLevel::Heading1, "An Order of Worship\nfor the Evening"))),
        Document::from(Rubric::from("The church is dark, or partially so, when the service is to begin.\n\nAll stand, and the Officiant greets the people with these words")).page(109),

        // Default opening
        Document::from(Preces::from([
            ("", "Light and peace, in Jesus Christ our Lord."),
            ("People", "Thanks be to God.")
        ])).condition(Condition::All(vec![
            Condition::Not(Box::new(EASTER_SEASON.clone())),
            Condition::Not(Box::new(LENT.clone())),
        ])),

        // Easter opening
        Document::from(Rubric::from("In place of the above, from Easter Day through the Day of Pentecost")).display(Show::TemplateOnly),
        Document::from(Preces::from([
            ("Officiant", "Alleluia. Christ is risen."),
            ("People", "The Lord is risen indeed. Alleluia.")
        ])).condition(EASTER_SEASON.clone()),

        // Lent opening
        Document::from(Rubric::from("In Lent and on other penitential occasions")).display(Show::TemplateOnly),
        Document::from(Preces::from([
            ("Officiant", "Bless the Lord who forgives all our sins;"),
            ("People", "His mercy endures for ever.")
        ])).condition(LENT.clone()),

        // Opening Sentence
        Document::from(Rubric::from("One of the following, or some other Short Lesson of Scripture appropriate to the occasion or to the season, may then be read")),
        Document::from(Choice::from(vec![
            Document::from(Sentence::from("Jesus said, “You are the light of the world. A city built on a hill cannot be hid. No one lights a lamp to put it under a bucket, but on a lamp-stand where it gives light for everyone in the house. And you, like the lamp, must shed light among your fellow men, so that they may see the good you do, and give glory to your Father in heaven.”").citation("Matthew 5:14-16")),
            Document::from(Sentence::from("It is not ourselves that we proclaim; we proclaim Christ Jesus as Lord, and ourselves as your servants, for Jesus’ sake. For the same God who said, “Out of darkness let light shine,” has caused his light to shine within us, to give the light of revelation––the revelation of the glory of God in the face of Jesus Christ.").citation("2 Corinthians 4:5-6")),
            Document::from(Sentence::from("If I say, “Surely the darkness will cover me, and the light around me turn to night,” darkness is not dark to you, O Lord; the night is as bright as the day; darkness and light to you are both alike.").citation("Psalm 139:10-11"))
        ])),

        // Prayer for Light
        Document::from(Rubric::from("The Officiant then says the Prayer for Light, using any one of the following or some other suitable prayer, first saying")).page(110),
        Document::from(Text::from("Let us pray.")),
        Document::from(Choice::from(vec![
            // Saints
            Document::from(Text::from("Lord Christ, your saints have been the lights of the world in every generation: Grant that we who follow in their footsteps may be made worthy to enter with them into that heavenly country where you live and reign for ever and ever.").response("Amen."))
                .label("Festivals of Saints")
                .condition(Condition::Season(Season::Saints)),

            // Seasonal options
            Document::from(
                Text::from("Almighty God, give us grace to cast away the works of darkness, and put on the armor of light, now in the time of this mortal life in which your Son Jesus Christ came to visit us in great humility; that in the last day, when he shall come again in his glorious majesty to judge both the living and the dead, we may rise to the life immortal; through him who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
            )
            .label("Collect for the First Sunday of Advent")
            .page(125)
            .condition(Condition::Season(Season::Advent))
            .display(Show::CompiledOnly),

            Document::from(
                Text::from("Almighty God, you have poured upon us the new light of your incarnate Word: Grant that this light, enkindled in our hearts, may shine forth in our lives; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
            )
            .label("Collect for the First Sunday after Christmas Day")
            .page(213)
            .condition(Condition::Any(vec![
                Condition::Season(Season::Christmas),
                Condition::Season(Season::Epiphany),
                Condition::Season(Season::Incarnation)
            ]))
            .display(Show::CompiledOnly),

            Document::from(Text::from("Almighty and most merciful God, kindle within us the fire of love, that by its cleansing flame we may be purged of all our sins and made worthy to worship you in spirit and in truth; through Jesus Christ our Lord.").response("Amen."))
                .label("Lent and other times of penitence")
                .condition(LENT.clone()),

            Document::from(Text::from("Eternal God, who led your ancient people into freedom by a pillar of cloud by day and a pillar of fire by night: Grant that we who walk in the light of your presence may rejoice in the liberty of the children of God; through Jesus Christ our Lord.").response("Amen."))
                .label("Easter Season")
                .condition(EASTER_SEASON.clone()),

            // Generic options
            Document::from(Text::from("Almighty God, we give you thanks for surrounding us, as daylight fades, with the brightness of the vesper light; and we implore you of your great mercy that, as you enfold us with the radiance of this light, so you would shine into our hearts the brightness of your Holy Spirit; through Jesus Christ our Lord.").response("Amen.")),
            Document::from(Text::from("Grant us, Lord, the lamp of charity which never fails, that it may burn in us and shed its light on those around us, and that by its brightness we may have a vision of that holy City, where dwells the true and never-failing Light, Jesus Christ our Lord.").response("Amen.")),
            Document::from(Text::from("O Lord God Almighty, as you have taught us to call the evening, the morning, and the noonday one day; and have made the sun to know its going down: Dispel the darkness of our hearts, that by your brightness we may know you to be the true God and eternal light, living and reigning for ever and ever.").response("Amen.")),
            Document::from(Text::from("Lighten our darkness, we beseech thee, O Lord; and by thy great mercy defend us from all perils and dangers of this night; for the love of thy only Son, our Savior, Jesus Christ.").response("Amen."))
        ])),

        // Template versions of seasonal options
        Document::from(Rubric::from("Advent")).display(Show::TemplateOnly),
        Document::from(Text::from("Collect for the First Sunday of Advent")).display(Show::TemplateOnly),

        Document::from(Rubric::from("Christmas, Epiphany, and other Feasts of the Incarnation")).display(Show::TemplateOnly),
        Document::from(Text::from("Collect for the First Sunday after Christmas")).display(Show::TemplateOnly),

        Document::from(Rubric::from("Lent and other times of penitence")).display(Show::TemplateOnly),
        Document::from(Text::from("Almighty and most merciful God, kindle within us the fire of love, that by its cleansing flame we may be purged of all our sins and made worthy to worship you in spirit and in truth; through Jesus Christ our Lord.").response("Amen.")).display(Show::TemplateOnly),

        Document::from(Rubric::from("Easter Season")).display(Show::TemplateOnly),
        Document::from(Text::from("Eternal God, who led your ancient people into freedom by a pillar of cloud by day and a pillar of fire by night: Grant that we who walk in the light of your presence may rejoice in the liberty of the children of God; through Jesus Christ our Lord.").response("Amen.")).display(Show::TemplateOnly),

        Document::from(Rubric::from("Festivals of Saints")).display(Show::TemplateOnly),
        Document::from(Text::from("Lord Christ, your saints have been the lights of the world in every generation: Grant that we who follow in their footsteps may be made worthy to enter with them into that heavenly country where you live and reign for ever and ever.").response("Amen.")).display(Show::TemplateOnly),

        // Candle-Lighting
        Document::from(Rubric::from("The candles at the Altar are now lighted, as are other candles and lamps as may be convenient.\n\nDuring the candle-lighting, an appropriate anthem or psalm may be sung, or silence kept.")),
        Document::from(HymnLink::Tag("Evening".into())),

        Document::from(Rubric::from("The following hymn, or a metrical version of it, or some other hymn, is then sung")),
        PHOS_HILARON.clone().page(112),

        Document::from(Series::from(vec![
            Document::from(Rubric::from("The service may then continue in any of the following ways:\n\nWith Evening Prayer, beginning with the Psalms; or with some other Office or Devotion;\n\nWith the celebration of the Holy Eucharist, beginning with the Salutation and Collect of the Day;\n\nOr, it may be followed by a meal or other activity, in which case Phos hilaron may be followed by the Lord’s Prayer and a grace or blessing;\n\nOr, it may continue as a complete evening Office with the following elements:"))
                .page(112),
            Document::from(Text::from("**Selection from the Psalter.** Silence, or a suitable Collect, or both, may follow the Psalmody.\n\n**Bible Reading.** A sermon or homily, a passage from Christian literature, or a brief silence, may follow the Reading.\n\n**Canticle.** The Magnificat or other canticle, or some other hymn of praise. \n\n**Prayers.** A litany, or other suitable devotions, including the Lord’s Prayer.\n\n**Blessing or Dismissal,** or both. The Peace may then be exchanged.")),
            Document::from(Rubric::from("On feasts or other days of special significance, the Collect of the Day, or one proper to the season, may precede the Blessing or Dismissal. On other days, either of the following, or one of the Collects from Evening Prayer or from Compline, may be so used"))
                .page(113),
            Document::from(Content::CollectOfTheDay { allow_multiple: true }),
            Document::from(Choice::from(vec![
                Document::from(Text::from("Blessed are you, O Lord, the God of our fathers, creator of the changes of day and night, giving rest to the weary, renewing the strength of those who are spent, bestowing upon us occasions of song in the evening. As you have protected us in the day that is past, so be with us in the coming night; keep us from every sin, every evil, and every fear; for you are our light and salvation, and the strength of our life. To you be glory for endless ages.").response("Amen.")),
                Document::from(Text::from("Almighty, everlasting God, let our prayer in your sight be as incense, the lifting up of our hands as the evening sacrifice. Give us grace to behold you, present in your Word and Sacraments, and to recognize you in the lives of those around us. Stir up in us the flame of that love which burned in the heart of your Son as he bore his passion, and let it burn in us to eternal life and to the ages of ages.").response("Amen."))
            ]))
                .page(113),

            Document::from(Rubric::from("A bishop or priest may use the following or some other blessing or grace"))
                .page(114),
            Document::from(Series::from(vec![
                Document::from(Text::from("The Lord bless you and keep you.").response("Amen.")),
                Document::from(Text::from("The Lord make his face to shine upon you\n\tand be gracious to you.").response("Amen.")),
                Document::from(Text::from("The Lord lift up his countenance upon you\n\tand give you peace.").response("Amen."))
            ])),
            Document::from(Rubric::from("A deacon or lay person using the preceding blessing substitutes “us” for “you.”")),
            Document::from(Rubric::from("A Dismissal may be used (adding “Alleluia, alleluia” in Easter Season)")),
            Document::from(Rubric::from("The People respond")),
            Document::from(Text::from("").response("Thanks be to God.")),

            Document::from(Rubric::from("In Easter Season the People respond")),
            Document::from(Text::from("").response("Thanks be to God. Alleluia, alleluia."))
        ]))
        .display(Show::TemplateOnly)
    ])))
    .label("An Order of Worship for the Evening")
    .page(109)
    .version(Version::BCP1979);
}
