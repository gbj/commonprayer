use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref THE_GREAT_LITANY: Document = Document::new()
        .label("The Great Litany")
        .page(148)
        .content(Liturgy::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Great Litany"))),
            Document::from(Rubric::from("To be said or sung, kneeling, standing, or in procession; before the Eucharist or after the Collects of Morning or Evening Prayer; or separately; especially in Lent and on Rogation days.")),
            Document::from(Litany::from((
                "Have mercy upon us.",
                [
                    "O God the Father, Creator of heaven and earth,",
                    "O God the Son, Redeemer of the world,",
                    "O God the Holy Ghost, Sanctifier of the faithful,",
                    "O holy, blessed, and glorious Trinity, one God,"
                ]
            ))),
            Document::from(ResponsivePrayer::from([
                "Remember not, Lord Christ, our offenses, nor the offenses of our forefathers; neither reward us according to our sins. Spare us, good Lord, spare thy people, whom thou hast redeemed with thy most precious blood, and by thy mercy preserve us for ever.",
                "Spare us, good Lord."
            ])),
            Document::from(Litany::from((
                "Good Lord, deliver us.",
                [
                    "From all evil and wickedness; from sin; from the crafts and assaults of the devil; and from everlasting damnation,",
                    "From all blindness of heart; from pride, vainglory, and hypocrisy; from envy, hatred, and malice; and from all want of charity,",
                    "From all inordinate and sinful affections; and from all the deceits of the world, the flesh, and the devil,",
                    "From all false doctrine, heresy, and schism; from hardness of heart, and contempt of thy Word and commandment,",
                    "From lightning and tempest; from earthquake, fire, and flood; from plague, pestilence, and famine,",
                    "From all oppression, conspiracy, and rebellion; from violence, battle, and murder; and from dying suddenly and unprepared,",
                    "By the mystery of thy holy Incarnation; by thy holy Nativity and submission to the Law; by thy Baptism, Fasting, and Temptation,",
                    "By thine Agony and Bloody Sweat; by thy Cross and Passion; by thy precious Death and Burial; by thy glorious Resurrection and Ascension; and by the Coming of the Holy Ghost,",
                    "In all time of our tribulation; in all time of our prosperity; in the hour of death, and in the day of judgment,"
                ]
            ))),
            Document::from(Litany::from((
                "We beseech thee to hear us, good Lord.",
                [
                    "We sinners do beseech thee to hear us, O Lord God; and that it may please thee to rule and govern thy holy Church Universal in the right way,",
                    "That it may please thee to illumine all bishops, priests, and deacons, with true knowledge and understanding of thy Word; and that both by their preaching and living, they may set it forth, and show it accordingly,",
                    "That it may please thee to bless and keep all thy people,",
                    "That it may please thee to send forth laborers into thy harvest, and to draw all mankind into thy kingdom,",
                    "That it may please thee to give to all people increase of grace to hear and receive thy Word, and to bring forth the fruits of the Spirit,",
                    "That it may please thee to bring into the way of truth all such as have erred, and are deceived,",
                    "That it may please thee to give us a heart to love and fear thee, and diligently to live after thy commandments,",
                    "That it may please thee so to rule the hearts of thy servants, the President of the United States (or of this nation), and all others in authority, that they may do justice, and love mercy, and walk in the ways of truth,",
                    "That it may please thee to make wars to cease in all the world; to give to all nations unity, peace, and concord; and to bestow freedom upon all peoples,",
                    "That it may please thee to show thy pity upon all prisoners and captives, the homeless and the hungry, and all who are desolate and oppressed,",
                    "That it may please thee to give and preserve to our use the bountiful fruits of the earth, so that in due time all may enjoy them,",
                    "That it may please thee to inspire us, in our several callings, to do the work which thou givest us to do with singleness of heart as thy servants, and for the common good,",
                    "That it may please thee to preserve all who are in danger by reason of their labor or their travel,",
                    "That it may please thee to preserve, and provide for, all women in childbirth, young children and orphans, the widowed, and all whose homes are broken or torn by strife,",
                    "That it may please thee to visit the lonely; to strengthen all who suffer in mind, body, and spirit; and to comfort with thy presence those who are failing and infirm,",
                    "That it may please thee to support, help, and comfort all who are in danger, necessity, and tribulation,",
                    "That it may please thee to have mercy upon all mankind,",
                    "That it may please thee to give us true repentance; to forgive us all our sins, negligences, and ignorances; and to endue us with the grace of thy Holy Spirit to amend our lives according to thy holy Word,",
                    "That it may please thee to forgive our enemies, persecutors, and slanderers, and to turn their hearts,",
                    "That it may please thee to strengthen such as do stand; to comfort and help the weak-hearted; to raise up those who fall; and finally to beat down Satan under our feet,",
                    "That it may please thee to grant to all the faithful departed eternal life and peace,",
                    "That it may please thee to grant that, in the fellowship of [__________ and] all the saints, we may attain to thy heavenly kingdom,"
                ]
            ))),
            Document::from(ResponsivePrayer::from([
                "Son of God, we beseech thee to hear us.",
                "Son of God, we beseech thee to hear us."
            ])),
            Document::from(ResponsivePrayer::from([
                "O Lamb of God, that takest away the sins of the world,",
                "Have mercy upon us.",
                "O Lamb of God, that takest away the sins of the world,",
                "Have mercy upon us.",
                "O Lamb of God, that takest away the sins of the world,",
                "Grant us thy peace."
            ])),
            Document::from(ResponsivePrayer::from([
                "O Christ, hear us.",
                "O Christ, hear us."
            ])),
            Document::from(Choice::from(vec![
                        Document::from(ResponsivePrayer::from([
                            "Lord, have mercy upon us.",
                            "Christ, have mercy upon us.",
                            "Lord, have mercy upon us. "
                        ])),
                        Document::from(ResponsivePrayer::from([
                            "Kyrie eleison.",
                            "Christe eleison.",
                            "Kyrie eleison."
                        ]))
            ])),
            Document::from(Rubric::from("When the Litany is sung or said immediately before the Eucharist, the Litany concludes here, and the Eucharist begins with the Salutation and the Collect of the Day.\n\nOn all other occasions, the Officiant and People say together")),
            Document::from(Text::from("Our Father, who art in heaven,\n\thallowed be thy Name,\n\tthy kingdom come,\n\tthy will be done,\n\t\ton earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n\tas we forgive those who trespass against us.\nAnd lead us not into temptation,\n\tbut deliver us from evil.").response("Amen.").display_format(DisplayFormat::Unison)),
            Document::from(Preces::from([
                ("V.", "O Lord, let thy mercy be showed upon us;"),
                ("R.", "As we do put our trust in thee.")
            ])),
            Document::from(Rubric::from("The Officiant concludes with the following or some other Collect")),
            Document::from(Text::from("Let us pray.")),
            Document::from(Text::from("Almighty God, who hast promised to hear the petitions of those who ask in thy Son’s Name: We beseech thee mercifully to incline thine ear to us who have now made our prayers and supplications unto thee; and grant that those things which we have asked faithfully according to thy will, may be obtained effectually, to the relief of our necessity, and to the setting forth of thy glory; through Jesus Christ our Lord.").response("Amen.")),
            Document::from(Rubric::from("The Officiant may add other Prayers, and end the Litany, saying")),
            Document::from(Text::from("The grace of our Lord Jesus Christ, and the love of God, and the fellowship of the Holy Ghost, be with us all evermore.").response("Amen."))
    ]));
}
