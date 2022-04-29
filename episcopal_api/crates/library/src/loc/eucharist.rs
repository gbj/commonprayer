use crate::rite2::eucharist::parallel::*;
use calendar::Season;
use language::*;
use lazy_static::lazy_static;
use lectionary::ReadingType;
use liturgy::*;

lazy_static! {
    pub static ref GLORIA_IN_EXCELSIS: Text = Text::from("Gloria a Dios en el cielo,\n\ty en la tierra paz a quienes ama el Señor.\n\nPor tu inmensa gloria\n\tte alabamos,\n\tte bendecimos,\n\tte adoramos,\n\tte glorificamos,\n\tte damos gracias,\nSeñor Dios, Rey celestial,\nDios Padre todopoderoso.\n\nSeñor, Hijo único Jesucristo,\nSeñor Dios, Cordero de Dios,\nHijo del Padre:\nTú que quitas el pecado del mundo,\n\tten piedad de nosotros;\nTú que quitas el pecado del mundo,\n\tatiende nuestra súplica;\nTú que estás sentado a la derecha del Padre,\n\tten piedad de nosotros:\n\nPorque sólo tú eres Santo,\nsólo tú Señor,\nsólo tú Altísimo, Jesucristo,\n\tcon el Espíritu Santo\n\ten la gloria de Dios Padre.\n\tAmén.");

    pub static ref KYRIE_ELEISON: Choice = Choice::from(vec![
        Document::from(ResponsivePrayer::from([
            "Señor, ten piedad [de nosotros].",
            "Cristo, ten piedad [de nosotros].",
            "Señor, ten piedad [de nosotros]."
        ])).version_label("Señor, ten piedad."),
        Document::from(ResponsivePrayer::from([
            "Kyrie eleison.",
            "Christe eleison.",
            "Kyrie eleison."
        ])).version_label("Kyrie eleison.")
    ]);

    pub static ref TRISAGION: ResponsivePrayer = ResponsivePrayer::from([
        "Santo Dios,\nSanto Poderoso,\nSanto Inmortal,",
        "Ten piedad de nosotros."
    ]);

    pub static ref NICENE_CREED_ES: Document = Document::from(Text::from("Creemos en un solo Dios,\n\tPadre todopoderoso,\n\tCreador de cielo y tierra,\n\tde todo lo visible e invisible.\n\nCreemos en un solo Señor, Jesucristo,\n\tHijo único de Dios,\n\tnacido del Padre antes de todos los siglos:\n\tDios de Dios, Luz de Luz,\n\tDios verdadero de Dios verdadero,\n\tengendrado, no creado,\n\tde la misma naturaleza que el Padre,\n\tpor quien todo fue hecho;\n\tque por nosotros\n\ty por nuestra salvación\n\tbajó del cielo:\n\tpor obra del Espíritu Santo\n\tse encarnó de María, la Virgen,\n\ty se hizo hombre.\n\tPor nuestra causa fue crucificado\n\ten tiempos de Poncio Pilato:\n\tpadeció y fue sepultado.\n\tResucitó al tercer día, según las Escrituras,\n\tsubió al cielo\n\ty está sentado a la derecha del Padre.\n\tDe nuevo vendrá con gloria\n\tpara juzgar a vivos y muertos,\n\ty su reino no tendrá fin.\n\nCreemos en el Espíritu Santo,\n\tSeñor y dador de vida,\n\tque procede del Padre y del Hijo,\n\tque con el Padre y el Hijo\n\trecibe una misma adoración y gloria,\n\ty que habló por los profetas.\n\tCreemos en la Iglesia,\n\tque es una, santa, católica y apostólica.\n\tReconocemos un solo Bautismo\n\tpara el perdón de los pecados.\n\tEsperamos la resurrección de los muertos\n\ty la vida del mundo futuro. Amén.").display_format(DisplayFormat::Unison)).label("El Credo Niceno")
        .language(Language::Es)
        .version(Version::LibroDeOracionComun)
        .source(Reference {
            source: Source::LibroDeOracionComun,
            page: 280
        });

    pub static ref RITE_II_LOC_EUCHARIST_PARALLEL: Document = Document::new()
        .label("Eucharist/Eucaristía")
        .page(355)
        .alternate_source(Reference {
            source: Source::LibroDeOracionComun,
            page: 277
        })
        .content(parallelize(
            &crate::rite2::eucharist::HOLY_EUCHARIST_II,
            &HOLY_EUCHARIST_II
        ).content);

    pub static ref HOLY_EUCHARIST_II: Document = Document::new()
        .label("Eucaristía")
        .source(Reference {
            source: Source::LibroDeOracionComun,
            page: 277
        })
        .language(Language::Es)
        .version(Version::LibroDeOracionComun)
        .content(Liturgy::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "La Santa Eucaristía:\nRito Dos"))).tags([TITLE]),
            Document::from(Heading::InsertDate),
            Document::from(Heading::InsertDay),

            Document::from(Heading::from((HeadingLevel::Heading2, "Palabra de Dios"))).tags([WORD_OF_GOD]),
            Document::from(Rubric::from("Puede cantarse un himno, salmo o antífona.")).tags([OPENING_HYMN]),
            Document::from(HymnLink::Hymnals).tags([OPENING_HYMN]),

            Document::from(Rubric::from("Todos de pie, el Celebrante dice:")).tags([OPENING_ACCLAMATION]),
            Document::from(Preces::from([
                ("", "Bendito sea Dios: Padre, Hijo y Espíritu Santo."),
                ("Pueblo", "Y bendito sea su reino, ahora y por siempre. Amén.")
            ])).condition(Condition::None(vec![
                Condition::Season(Season::Lent),
                Condition::Season(Season::HolyWeek),
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).tags([OPENING_ACCLAMATION]),

            Document::new()
                .display(Show::TemplateOnly)
                .content(Rubric::from("Desde el Día de Pascua hasta el Día de Pentecostés inclusive, en lugar de lo anterior, se dice:"))
                .tags([OPENING_ACCLAMATION_EASTER]),
            Document::from(Preces::from([
                ("Celebrante", "¡Aleluya! Cristo ha resucitado."),
                ("Pueblo", "¡Es verdad! El Señor ha resucitado. ¡Aleluya!")
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).tags([OPENING_ACCLAMATION_EASTER]),
            Document::new().display(Show::TemplateOnly)
                .content(Rubric::from("Durante la Cuaresma y en otras ocasiones penitenciales, se dice:"))
                .tags([OPENING_ACCLAMATION_LENT]),
            Document::from(Preces::from([
                ("Celebrante", "Bendigan al Señor, quien perdona todos nuestros pecados."),
                ("Pueblo", "Para siempre es su misericordia.")
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Lent),
                Condition::Season(Season::HolyWeek)
            ])).tags([OPENING_ACCLAMATION_LENT]),

            Document::from(Rubric::from("El Celebrante puede decir:")).tags([COLLECT_FOR_PURITY]),
            Document::from(Text::from("Dios omnipotente, para quien todos los corazones están manifiestos, todos los deseos son conocidos y ningún secreto se halla encubierto: Purifica los pensamientos de nuestros corazones por la inspiración de tu Santo Espíritu, para que perfectamente te amemos y dignamente proclamemos la grandeza de tu santo Nombre; por Cristo nuestro Señor.").response("Amén.")).tags([COLLECT_FOR_PURITY]),

            Document::from(Choice::from(vec![
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("Cuando se indique, se canta o dice el siguiente himno u otro cántico de alabanza. Todos de pie.")),
                    Document::from(GLORIA_IN_EXCELSIS.clone()),
                ])).version_label("Gloria").tags([GLORIA_IN_EXCELSIS_TAG]),
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("En otras ocasiones se usa lo siguiente:")),
                    Document::from(ResponsivePrayer::from([
                        "Señor, ten piedad [de nosotros].",
                        "Cristo, ten piedad [de nosotros].",
                        "Señor, ten piedad [de nosotros]."
                    ]))
                ])).version_label("Kyrie (“Señor, ten piedad.”)").tags([KYRIE_TAG]),
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("En otras ocasiones se usa lo siguiente:")),
                    Document::from(ResponsivePrayer::from([
                        "Kyrie eleison.",
                        "Christe eleison.",
                        "Kyrie eleison."
                    ]))
                ])).version_label("Kyrie (“Kyrie eleison.”)").tags([KYRIE_TAG]),
                Document::from(Series::from(vec![
                    Document::from(Rubric::from("o bien:")),
                    Document::from(TRISAGION.clone())
                ])).version_label("Trisagion").tags([TRISAGION_TAG])
            ])).tags([GLORIA_SECTION_TAG]),
            Document::from(Heading::from((HeadingLevel::Heading3, "Colecta del Día"))).tags([COLLECT_OF_THE_DAY]),
            Document::from(Rubric::from("El Celebrante dice al pueblo:")).tags([COLLECT_OF_THE_DAY]),
            Document::from(Preces::from([
                ("", "El Señor sea con ustedes."),
                ("Pueblo", "Y con tu espíritu."),
                ("Celebrante", "Oremos.")
            ])).tags([COLLECT_OF_THE_DAY]),
            Document::from(Rubric::from("El Celebrante dice la Colecta.")).tags([COLLECT_OF_THE_DAY]),
            Document::new().version(Version::LibroDeOracionComun)
                .content(Content::CollectOfTheDay { allow_multiple: false })
                .tags([COLLECT_OF_THE_DAY]),
            Document::new().display(Show::TemplateOnly)
                .content(Preces::from([
                ("", ""),
                ("Pueblo", "Amén.")
            ])).tags([COLLECT_OF_THE_DAY]),
            Document::from(Heading::from((HeadingLevel::Heading3, "Lecciones"))).tags([THE_LESSONS]),
            Document::new().display(Show::TemplateOnly)
                .content(Series::from(vec![
                    Document::from(Rubric::from("El pueblo se sienta. Se lee una o dos Lecciones, seg ún se indique. El Lector dice:")),
                    Document::from(Text::from("Lectura (Lección) de ____________.")),
                    Document::from(Rubric::from("Puede añadirse la referencia al capítulo y versículo.")),
                    Document::from(Rubric::from("Después de cada Lectura el Lector puede decir:")),
                    Document::from(Preces::from([
                        ("", "Palabra del Señor."),
                        ("Pueblo", "Demos gracias a Dios.")
                    ])),
                    Document::from(Rubric::from("o el Lector puede decir:")),
                    Document::from(Text::from("Aquí termina la Lectura (Epístola).")),
                    Document::from(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::FirstReading),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Selected(Lectionaries::RCLTrack2),
                        intro: None,
                    }),
                    Document::from(Rubric::from("Puede guardarse un período de silencio.\n\nDespués de cada Lectura puede seguir un Salmo, himno o antífona.")),
                    Document::from(Content::HymnLink(HymnLink::Hymnals)),
                    Document::from(Rubric::from("Entonces, todos de pie, el Diácono o un Presbítero lee el Evangelio, diciendo primero:")),
                    Document::from(Preces::from([
                        ("", "Santo Evangelio de nuestro Señor Jesucristo, según _______________."),
                        ("Pueblo", "¡Gloria a ti, Cristo Señor!")
                    ])),
                    Document::from(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::Gospel),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Selected(Lectionaries::RCLTrack2),
                        intro: None,
                    }),
                    Document::from(Rubric::from("Después del Evangelio el Lector dice:")),
                    Document::from(Preces::from([
                        ("", "El Evangelio del Señor."),
                        ("Pueblo", "Te alabamos, Cristo Señor.")
                    ])),
            ])).tags([LESSONS_RUBRICS]),
            Document::new().display(Show::CompiledOnly)
                .content(Series::from(vec![
                    Document::from(Rubric::from("El pueblo se sienta.")).tags([LESSONS_RUBRICS]),
                    // TODO insert responses for these
                    Document::new().label("Primera Lectura")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Preference(PreferenceKey::Global(GlobalPref::ReadingA)),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Text::from("Lectura de {{long_name}}."))))
                    }).tags([FIRST_LESSON]),
                    Document::from(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::Psalm),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: None
                    }).tags([PSALM]),
                    Document::new().label("Segunda Lectura")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Preference(PreferenceKey::Global(GlobalPref::ReadingB)),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Text::from("Lectura de {{long_name}}."))))
                    }).tags([SECOND_LESSON]),
                    Document::new().label("Evangelio")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::Gospel),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Preces::from([
                        ("", "Santo Evangelio de nuestro Señor Jesucristo, según"),
                        ("Pueblo", "¡Gloria a ti, Cristo Señor!")
                    ]))))
                }).tags([GOSPEL])
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "Sermón"))).tags([SERMON]),
            Document::from(Rubric::from("Los domingos, y en otras Fiestas Mayores, todos de pie, dicen:")).tags([NICENE_CREED]),
            NICENE_CREED_ES.clone().tags([NICENE_CREED]),

            Document::from(Heading::from((HeadingLevel::Heading3, "Oración de los Fieles"))).tags([PRAYERS_OF_THE_PEOPLE]),
            Document::from(Rubric::from("La oración se ofrece con intercesiones por:\n\nLa Iglesia Universal, sus miembros y su misión\nLa Nación y sus autoridades\nEl bienestar del mundo\nLos intereses de la comunidad local\nLos que sufren y los atribulados\nLos difuntos (con la conmemoración de un santo cuando sea apropiado).\n\nVéanse las fórmulas que comienzan en la página 305.")).tags([POP_RUBRIC]),
            Document::from(Content::DocumentLink {
                label: "Oración de los Fieles".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::PrayersOfThePeople, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: false
            }).tags([POP_FORMS]),

            Document::from(Rubric::from("Si no hay celebración de la Comunión, o si no hay sacerdote, el rito concluye como se indica en la página 329.")).tags([NO_COMMUNION_RUBRIC]),
            Document::from(Heading::from((HeadingLevel::Heading3, "Confesión de Pecado"))).tags([CONFESSION]),
            Document::from(Rubric::from("Si no se ha hecho antes la Confesión de Pecado, se hace aquí. En ciertas ocasiones la Confesión puede omitirse.")).tags([CONFESSION]),
            Document::from(Rubric::from("Puede decirse uno de los versículos del Orden Penitencial, en la página 273.")).tags([CONFESSION]),
            Document::from(Content::DocumentLink {
                label: "Versículos del Orden Penitencial".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::PenitentialSentences, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: false
            }).tags([CONFESSION]),
            Document::from(Rubric::from("El Diácono o el Celebrante dice:")).tags([CONFESSION]),
            Document::from(Text::from("Confesemos nuestros pecados contra Dios y contra nuestro prójimo.")).tags([CONFESSION]),
            Document::from(Rubric::from("Puede guardarse un período de silencio.")).tags([CONFESSION]),
            Document::from(Rubric::from("Ministro y Pueblo:")).tags([CONFESSION]),
            Document::from(Text::from("Dios de misericordia,\nconfesamos que hemos pecado contra ti\npor pensamiento, palabra y obra,\npor lo que hemos hecho\ny lo que hemos dejado de hacer.\nNo te hemos amado con todo el corazón;\nno hemos amado a nuestro prójimo como a nosotros mismos.\nSincera y humildemente nos arrepentimos.\nPor amor de tu Hijo Jesucristo,\nten piedad de nosotros y perdónanos;\nasí tu voluntad será nuestra alegría\ny andaremos por tus caminos,\npara gloria de tu Nombre. Amén.").display_format(DisplayFormat::Unison)).tags([CONFESSION]),
            Document::from(Rubric::from("El Obispo, si está presente, o el Sacerdote, puesto de pie, dice:")).tags([CONFESSION]),
            Document::from(Text::from("Dios omnipotente tenga misericordia de ustedes, perdone todos sus pecados por Jesucristo nuestro Señor, les fortalezca en toda bondad y, por el poder del Espíritu Santo, les conserve en la vida eterna.").response("Amén.")).tags([CONFESSION]),
            Document::from(Heading::from((HeadingLevel::Heading3, "La Paz"))).tags([THE_PEACE]),
            Document::from(Rubric::from("Todos de pie, el Celebrante dice:")).tags([THE_PEACE]),
            Document::from(Preces::from([
                ("", "La paz del Señor sea siempre con ustedes."),
                ("Pueblo", "Y con tu espíritu.")
            ])).tags([THE_PEACE]),
            Document::from(Rubric::from("Los Ministros y el Pueblo pueden saludarse mutuamente en el nombre del Señor.")).tags([THE_PEACE]),
            Document::from(Heading::from((HeadingLevel::Heading2, "Santa Comunión"))).tags([HOLY_COMMUNION]),
            Document::from(Rubric::from("El Celebrante puede comenzar el Ofertorio con Uno de los versículos en la página 299, o con otro versículo de las Escrituras.")).tags([OFFERTORY]),

            Document::from(Content::DocumentLink {
                label: "Versículos".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::OffertorySentences, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: false
            }).version(Version::RiteII).tags([OFFERTORY]),

            Document::from(Rubric::from("Durante el Ofertorio puede cantarse un himno, salmo o antífona.")).tags([OFFERTORY_HYMN]),
            Document::from(Content::HymnLink(HymnLink::Hymnals)).tags([OFFERTORY_HYMN]),
            Document::from(Rubric::from("Representantes de la congregación traen al diácono o al celebrante las ofrendas del pueblo de pan y vino, y de dinero u otros dones. El pueblo se pone de pie mientras se presentan las ofrendas y se colocan sobre el Altar.")).tags([OFFERTORY_RUBRIC_2]),

            Document::from(Heading::from((HeadingLevel::Heading3, "La Gran Plegaria Eucarística"))).tags([GREAT_THANKSGIVING_HEADER]),
            Document::from(Content::DocumentLink {
                label: "Plegarias Eucarísticas".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::GreatThanksgiving]),
                rotate: false
            }).tags([EUCHARISTIC_PRAYERS]).display(Show::TemplateOnly),
            Document::from(Choice::from(vec![
                PRAYER_A.clone(),
                PRAYER_B.clone(),
                PRAYER_C.clone(),
                PRAYER_D.clone()
            ])).tags([EUCHARISTIC_PRAYERS]),

            Document::from(Series::from(vec![
                Document::from("Oremos como nuestro Salvador Cristo nos enseñó."),
                Document::from(Rubric::from("Pueblo y Celebrante:")),
                Document::from(Text::from("Padre nuestro que estás en el cielo,\n\tsantificado sea tu Nombre,\n\tvenga tu reino,\n\thágase tu voluntad,\n\t\ten la tierra como en el cielo.\nDanos hoy nuestro pan de cada día.\nPerdona nuestras ofensas,\n\tcomo también nosotros perdonamos\n\t\ta los que nos ofenden.\nNo nos dejes caer en tentación\n\ty líbranos del mal.\nPorque tuyo es el reino,\n\ttuyo es el poder,\n\ty tuya es la gloria,\n\tahora y por siempre. Amén.").display_format(DisplayFormat::Unison))
            ])).tags([LORDS_PRAYER_TAG]),

            Document::from(Heading::from((HeadingLevel::Heading3, "Fracción del Pan"))).tags([FRACTION]),
            Document::from(Rubric::from("El Celebrante parte el Pan consagrado.\n\nSe guarda un período de silencio.\n\nLuego puede cantarse o decirse:")).tags([FRACTION]),
            Document::from(ResponsivePrayer::from([
                "[¡Aleluya!] Cristo, nuestra Pascua, se ha sacrificado por nosotros.",
                "¡Celebremos la fiesta! [¡Aleluya!]"
            ]))
                .tags([FRACTION_ANTHEM])
                .condition(Condition::None(vec![
                    Condition::Season(Season::Lent),
                    Condition::Season(Season::HolyWeek),
                    Condition::Season(Season::Easter),
                    Condition::Season(Season::Ascension),
                    Condition::Season(Season::Pentecost),
                ])),
            Document::from(ResponsivePrayer::from([
                "¡Aleluya! Cristo, nuestra Pascua, se ha sacrificado por nosotros.",
                "¡Celebremos la fiesta! ¡Aleluya!"
            ]))
                .display(Show::CompiledOnly)
                .tags([FRACTION_ANTHEM])
                .condition(Condition::Any(vec![
                    Condition::Season(Season::Easter),
                    Condition::Season(Season::Ascension),
                    Condition::Season(Season::Pentecost),
                ])),
            Document::from(ResponsivePrayer::from([
                "Cristo, nuestra Pascua, se ha sacrificado por nosotros.",
                "¡Celebremos la fiesta!"
            ]))
                .display(Show::CompiledOnly)
                .tags([FRACTION_ANTHEM])
                .condition(Condition::Any(vec![
                    Condition::Season(Season::Lent),
                    Condition::Season(Season::HolyWeek),
                ])),
            Document::from(Rubric::from("Durante la Cuaresma se omite el ¡Aleluya! y también puede omitirse en otras ocasiones, excepto durante la Estación de Pascua.")).display(Show::TemplateOnly).tags([FRACTION_ANTHEM_RUBRIC]),
            Document::from(Rubric::from("En lugar de, o además de, lo precedente puede usarse cualquier otra antífona apropiada.")).tags([FRACTION_ANTHEM_RUBRIC]),
            Document::from(HymnLink::TagWithLabel("Antífonas".into(), "Fraction Anthems".into())).tags([FRACTION_ANTHEM_RUBRIC]),
            Document::from(Rubric::from("De cara al pueblo, el Celebrante hace la siguiente Invitación:")).tags([COMMUNION_INVITATION]),
            Document::from(Text::from("Los Dones de Dios para el Pueblo de Dios.")).tags([COMMUNION_INVITATION]),
            Document::from(Rubric::from("y puede añadir:")).tags([COMMUNION_INVITATION]),
            Document::from(Text::from("Tómenlos en memoria de que Cristo murió por ustedes, y aliméntense de él en sus corazones, por fe y con agradecimiento.")).tags([COMMUNION_INVITATION]),
            Document::from(Rubric::from("Los ministros reciben el Sacramento en ambas especies e inmediatamente después lo dan al pueblo.")).tags([DISTRIBUTION_RUBRIC]),
            Document::from(Rubric::from("Se da a los comulgantes el Pan y el Cáliz con estas palabras:")).tags([DISTRIBUTION_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Text::from("El Cuerpo (la Sangre) de nuestro Señor Jesucristo te guarde en la vida eterna.").response("[Amén.]")),
                Document::from(Series::from(vec![
                    Document::from(Text::from("El Cuerpo de Cristo, pan del cielo.").response("[Amén.]")),
                    Document::from(Text::from("La Sangre de Cristo, cáliz de salvación.").response("[Amén.]"))
                ]))
            ])).tags([DISTRIBUTION_RUBRIC]),
            Document::from(Rubric::from("Durante la administración de la Comunión pueden cantarse himnos, salmos o antífonas.")).tags([COMMUNION_HYMN]),
            Document::from(Content::HymnLink(HymnLink::Hymnals)).tags([COMMUNION_HYMN]),
            Document::from(Rubric::from("Cuando sea necesario, el Celebrante consagra más pan y vino, utilizando la fórmula de la página 331.")).tags([ADDITIONAL_CONSECRATION_TAG]),
            Document::from(Content::DocumentLink {
                label: "La Fórmula para Consagrar Más Pan y Vino".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ConsecratingAdditional, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: false
            }).tags([ADDITIONAL_CONSECRATION_TAG]),
            Document::from(Rubric::from("Después de la Comunión, el Celebrante dice:")).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Text::from("Oremos.")).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Rubric::from("Celebrante y Pueblo:")).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Choice::from(vec![
                Document::new()
                    .source(Reference {
                        source: Source::LibroDeOracionComun,
                        page: 288
                    })
                    .content(Text::from("Eterno Dios, Padre celestial,\nen tu bondad nos has aceptado como miembros vivos\nde tu Hijo, nuestro Salvador Jesucristo;\nnos has nutrido con alimento espiritual\nen el Sacramento de su Cuerpo y de su Sangre.\nEnvíanos ahora en paz al mundo;\nrevístenos de fuerza y de valor\npara amarte y servirte\ncon alegría y sencillez de corazón;\npor Cristo nuestro Señor. Amén.").display_format(DisplayFormat::Unison)),
                Document::new()
                    .source(Reference {
                        source: Source::LibroDeOracionComun,
                        page: 288
                    })
                    .content(Text::from("Omnipotente y sempiterno Dios, te damos gracias\nporque nos has nutrido con el alimento espiritual\ndel preciosísimo Cuerpo y Sangre\nde tu Hijo, nuestro Salvador Jesucristo;\ny porque nos aseguras, en estos santos misterios,\nque somos miembros vivos del Cuerpo de tu Hijo\ny herederos de tu reino eterno.\nY ahora, Padre, envíanos al mundo para cumplir la misión\nque tú nos has encomendado,\npara amarte y servirte\ncomo fieles testigos de Cristo nuestro Señor.\nA él, a ti y al Espíritu Santo,\nsea todo honor y gloria, ahora y por siempre. Amén.").display_format(DisplayFormat::Unison))
            ])).tags([POSTCOMMUNION_PRAYER_TAG]),
            Document::from(Rubric::from("El Obispo, si está presente, o el Sacerdote, puede bendecir al pueblo.")).tags([BLESSING]),
            Document::from(Rubric::from("El Diácono, o el Celebrante, despide al pueblo con estas palabras:s")).tags([DISMISSAL_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("", "Salgamos en nombre de Cristo."),
                    ("Pueblo", "Demos gracias a Dios.")
                ])),
                Document::from(Preces::from([
                    ("Diácono", "Vayan en paz para amar y servir al Señor."),
                    ("Pueblo", "Demos gracias a Dios.")
                ])),
                Document::from(Preces::from([
                    ("Diácono", "Salgamos con gozo al mundo, en el poder del Espíritu."),
                    ("Pueblo", "Demos gracias a Dios.")
                ])),
                Document::from(Preces::from([
                    ("Diácono", "Bendigamos al Señor."),
                    ("Pueblo", "Demos gracias a Dios.")
                ]))
            ])).condition(Condition::None(vec![
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).tags([DISMISSAL]),
            Document::from(Choice::from(vec![
                Document::from(Preces::from([
                    ("", "Salgamos en nombre de Cristo. ¡Aleluya, aleluya!"),
                    ("Pueblo", "Demos gracias a Dios. ¡Aleluya, aleluya!")
                ])),
                Document::from(Preces::from([
                    ("Diácono", "Vayan en paz para amar y servir al Señor. ¡Aleluya, aleluya!"),
                    ("Pueblo", "Demos gracias a Dios. ¡Aleluya, aleluya!")
                ])),
                Document::from(Preces::from([
                    ("Diácono", "Salgamos con gozo al mundo, en el poder del Espíritu. ¡Aleluya, aleluya!"),
                    ("Pueblo", "Demos gracias a Dios. ¡Aleluya, aleluya!")
                ])),
                Document::from(Preces::from([
                    ("Diácono", "Bendigamos al Señor. ¡Aleluya, aleluya!"),
                    ("Pueblo", "Demos gracias a Dios. ¡Aleluya, aleluya!")
                ]))
            ])).condition(Condition::Any(vec![
                Condition::Season(Season::Easter),
                Condition::Season(Season::Ascension),
                Condition::Season(Season::Pentecost)
            ])).display(Show::CompiledOnly).tags([DISMISSAL]),
            Document::new().display(Show::TemplateOnly)
                .content(Rubric::from("Desde la Vigilia Pascual basta el Día de Pentecostés inclusive, puede añadirse ¡Aleluya, aleluya! a cualquiera de las despedidas."))
                .tags([DISMISSAL_RUBRIC_2]),
            Document::new().display(Show::TemplateOnly)
                .content(Preces::from([
                ("", ""),
                ("El Pueblo responde:", "Demos gracias a Dios. ¡Aleluya, aleluya!")
            ])).tags([DISMISSAL_EASTER])
        ]));

    pub static ref OFFERTORY_SENTENCES_LOC: Vec<Document> = vec![
        Document::from(Sentence::from("Sacrifica a Dios alabanza, y paga tus votos al Altísimo.").citation("Salmo 50:14")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Rindan al Señor la gloria debida a su Nombre; traigan ofrendas, y entren en sus atrios.").citation("Salmo 96:8")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Anden en amor, como también Cristo nos amó, y se entregó a sí mismo por nosotros, ofrenda y sacrificio a Dios.").citation("Efesios 5:2")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Hermanos, les ruego por las misericordias de Dios, que presenten sus cuerpos en sacrificio vivo, santo, agradable a Dios, que es su culto racional.").citation("Romanos 12:1")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Si traes tu ofrenda al altar, y allí te acuerdas de que tu hermano tiene algo contra ti, deja allí tu ofrenda delante del altar, y anda, reconcíliate primero con tu h ermano, y entonces ven y presenta tu ofrenda.").citation("San Mateo 5:23, 24")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Ofrezcamos siempre a Dios, por medio de Cristo, sacrificio de alabanza, es decir, fruto de labios que confiesan su Nombre. Y de hacer bien y de la ayuda mutua no se olviden; porque de tales sacrificios se agrada Dios.").citation("Hebreos 13:15, 16")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Señor, digno eres de recibir la gloria y la honra y el poder; porque tú creaste todas las cosas, y por tu voluntad existen y fueron creadas.").citation("Apocalipsis 4:11")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Tuya es, oh Señor, la magnificencia y el p oder, la gloria, la victoria y el honor; porque todas las cosas que están en los cielos y en la tierra son tuyas. Tuyo, oh Señor, es el reino, y tú eres excelso sobre todos.").citation("1 Crónicas 29:11")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from("Presentemos al Señor con alegría las ofrendas y oblaciones de nuestra vida y de nuestro trabajo.").language(Language::Es).version(Version::LibroDeOracionComun)
    ];

    pub static ref PENITENTIAL_SENTENCES_LOC: Vec<Document> = vec![
        Document::from(Sentence::from("Jesús dijo: “El primer mandamiento es éste: Escucha, Israel: El Señor nuestro Dios es el único Señor. Amarás al Señor tu Dios con todo tu corazón, con toda tu alma, con toda tu mente y con todas tus fuerzas. El segundo es éste: Amarás a tu prójimo como a ti mismo. No hay otro mandamiento mayor que éstos.”").citation("San Marcos 12:29-31")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Si decimos: “No tenemos pecado,” nos engañamos y la verdad no está en nosotros. Si reconocemos nuestros pecados, fiel y justo es él para perdonarnos los pecados y purificarnos de toda injusticia.").citation("1 San Juan 1:8,9")).language(Language::Es).version(Version::LibroDeOracionComun),
        Document::from(Sentence::from("Por tanto, teniendo tal Sumo Sacerdote que penetró los cielos, Jesús, el Hijo de Dios, acerquémonos confiadamente al trono de gracia, a fin de alcanzar misericordia y hallar gracia para una ayuda oportuna.").citation("Hebreos 4:14, 16")).language(Language::Es).version(Version::LibroDeOracionComun),
    ];

    pub static ref PRAYER_A: Document = Document::new()
        .label("Plegaria Eucarística A")
        .version_label("Plegaria A")
        .language(Language::Es)
        .source(Reference {
                    source: Source::LibroDeOracionComun,
                    page: 282
                })
        .version(Version::LibroDeOracionComun)
        .content(Series::from(vec![
            Document::from(Rubric::from("El pueblo permanece de pie. El Celebrante, sea obispo o sacerdote, de cara al pueblo, canta o dice:")),
            Document::from(Preces::from([
                ("", "El Señor sea con ustedes."),
                ("Pueblo", "Y con tu espíritu."),
                ("Celebrante", "Elevemos los corazones."),
                ("Pueblo", "Los elevamos al Señor."),
                ("Celebrante", "Demos gracias a Dios nuestro Señor."),
                ("Pueblo", "Es justo darle gracias y alabanza.")
            ])),
            Document::from(Rubric::from("El Celebrante continúa:")),
            Document::from(Text::from("En verdad es digno, justo y saludable, darte gracias, en todo tiempo y lugar, Padre omnipotente, Creador de cielo y tierra.")),
            Document::from(Rubric::from("Aquí, todos los domingos y en las ocasiones que se indique, se canta o dice el Prefacio Propio.")),
            Document::from(Content::DocumentLink {
                label: "Prefacios Propios".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: true
            }),
            Document::from(Text::from("Por tanto te alabamos, uniendo nuestras voces con los Angeles y Arcángeles, y con todos los coros celestiales que, proclamando la gloria de tu Nombre, por siempre cantan este himno:")),
            Document::from(Rubric::from("Celebrante y Pueblo:")),
            Document::from(Text::from("Santo, santo, santo es el Señor, Dios del universo.\nLlenos están el cielo y la tierra de tu gloria.\n\tHosanna en el cielo.\nBendito el que viene en nombre del Señor.\n\tHosanna en el cielo.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("El pueblo permanece de pie o se arrodilla.\n\nEl Celebrante continúa:")),
            Document::from(Text::from("Padre Santo y bondadoso: En tu amor infinito nos hiciste para ti, y cuando caímos en pecado y quedamos esclavos del mal y de la muerte, tú, en tu misericordia, enviaste a Jesucristo, tu Hijo único y eterno, p ara compartir nuestra naturaleza humana, para vivir y morir como uno de nosotros, y así reconciliarnos contigo, el Dios y Padre de todos.\n\nExtendió sus brazos sobre la cruz y se ofreció en obediencia a tu voluntad, un sacrificio perfecto por todo el mundo.")),
            Document::from(Rubric::from("Al decir las palabras relativas al pan, el Celebrante lo toma en sus manos o impone una mano sobre él; y al decir la palabras relativas al cáliz, lo toma en sus manos o impone una mano sobre él y sobre cualquier otro recipiente con vino que hubiere de consagrarse.")),
            Document::from(Text::from("En la noche en que fue entregado al sufrimiento y a la muerte, nuestro Señor Jesucristo tomó pan; y dándote gracias, lo partió y lo dio a sus discípulos, y dijo: “Tomen y coman. Este es mi Cuerpo, entregado por ustedes. Hagan esto como memoria l mío.”\n\nDespués de la cena tomó el cáliz; y dándote gracias, se lo entregó, y dijo: “Beban todos de él. Esta es mi Sangre del nuevo Pacto, sangre derramada por ustedes y por muchos para el perdón de los pecados. Siempre que lo beban, háganlo como memorial mío.”\n\nPor tanto, proclamamos el misterio de fe:")),
            Document::from(Rubric::from("Celebrante y Pueblo:")),
            Document::from(Text::from("Cristo ha muerto.\nCristo ha resucitado.\nCristo volverá.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("El Celebrante continúa:")),
            Document::from(Text::from("Padre, en este sacrificio de alabanza y acción de gracias, celebramos el memorial de nuestra redenció n. Recordando su muerte, resurrección y ascención, te ofrecemos estos dones.\n\nSantifícalos con tu Espíritu Santo, y así serán para tu pueblo el Cuerpo y la Sangre de tu Hijo, la santa comida y la santa bebida de la vida nueva en él que no tiene fin. Santifícanos también, para que recibamos fielmente este Santo Sacramento y seamos perseverantes en tu servicio en paz y unidad. Y en el día postrero, llévanos con todos tus santos al gozo de tu reino eterno.\n\nTodo esto te pedimos por tu Hijo Jesucristo. Por él, y con él y en él, en la unidad del Espíritu Santo, tuyos son el honor y la gloria, Padre omnipotente, ahora y por siempre.").response("AMEN."))
    ]));

    pub static ref PRAYER_B: Document = Document::new()	.label("Eucharistic Prayer B")
        .version_label("Prayer B")
        .page(367)
        .version(Version::RiteII)
        .content(Series::from(vec![
            Document::from(Rubric::from("The people remain standing. The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("Pueblo", "And also with you."),
                ("Celebrant", "Lift up your hearts."),
                ("Pueblo", "We lift them to the Lord."),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("Pueblo", "It is right to give him thanks and praise.")
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(Text::from("It is right, and a good and joyful thing, always and everywhere to give thanks to you, Father Almighty, Creator of heaven and earth.")),
            Document::from(Rubric::from("Here a Proper Preface is sung or said on all Sundays, and on other occasions as appointed.")),
            Document::from(Content::DocumentLink {
                label: "Proper Prefaces".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Version(Version::RiteII)]),
                rotate: true
            }),
            Document::from(Text::from("Therefore we praise you, joining our voices with Angels and Archangels and with all the company of heaven, who for ever sing this hymn to proclaim the glory of your Name:")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is he who comes in the name of the Lord.\n\tHosanna in the highest. ").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The people stand or kneel.\n\nThen the Celebrant continues")),
            Document::from(Text::from("We give thanks to you, O God, for the goodness and love which you have made known to us in creation; in the calling of Israel to be your people; in your Word spoken through the prophets; and above all in the Word made flesh, Jesus, your Son. For in these last days you sent him to be incarnate from the Virgin Mary, to be the Savior and Redeemer of the world. In him, you have delivered us from evil, and made us worthy to stand before you. In him, you have brought us out of error into truth, out of sin into righteousness, out of death into life.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated.")),
            Document::from(Text::from("On the night before he died for us, our Lord Jesus Christ took bread; and when he had given thanks to you, he broke it, and gave it to his disciples, and said, “Take, eat: This is my Body, which is given for you. Do this for the remembrance of me.”\n\nAfter supper he took the cup of wine; and when he had given thanks, he gave it to them, and said, “Drink this, all of you: This is my Blood of the new Covenant, which is shed for you and for many for the forgiveness of sins. Whenever you drink it, do this for the remembrance of me.”\n\nTherefore, according to his command, O Father,")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("We remember his death,\nWe proclaim his resurrection,\nWe await his coming in glory.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("And we offer our sacrifice of praise and thanksgiving to you, O Lord of all; presenting to you, from your creation, this bread and this wine.\n\nWe pray you, gracious God, to send your Holy Spirit upon these gifts that they may be the Sacrament of the Body of Christ and his Blood of the new Covenant. Unite us to your Son in his sacrifice, that we may be acceptable through him, being sanctified by the Holy Spirit. In the fullness of time, put all things in subjection under your Christ, and bring us to that heavenly country where, with [ ___________ and] all your saints, we may enter the everlasting heritage of your sons and daughters; through Jesus Christ our Lord, the firstborn of all creation, the head of the Church, and the author of our salvation.\n\nBy him, and with him, and in him, in the unity of the Holy Spirit all honor and glory is yours, Almighty Father, now and for ever.").response("AMEN."))
    ]));

    pub static ref PRAYER_C: Document = Document::new()
        .version(Version::RiteII)
        .label("Eucharistic Prayer C")
        .version_label("Prayer C")
        .page(369)
        .content(Series::from(vec![
            Document::from(Rubric::from("In this prayer, the lines in italics are spoken by the People.")),
            Document::from(Rubric::from("The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(ResponsivePrayer::from([
                "The Lord be with you.",
                "And also with you.\n",
                "Lift up your hearts.",
                "We lift them to the Lord.\n",
                "Let us give thanks to the Lord our God.",
                "It is right to give him thanks and praise.\n"
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(ResponsivePrayer::from([
                "God of all power, Ruler of the Universe, you are worthy of glory and praise.",
                "Glory to you for ever and ever.\n",
                "At your command all things came to be: the vast expanse of interstellar space, galaxies, suns, the planets in their courses, and this fragile earth, our island home.",
                "By your will they were created and have their being.\n",
                "From the primal elements you brought forth the human race, and blessed us with memory, reason, and skill. You made us the rulers of creation. But we turned against you, and betrayed your trust; and we turned against one another.",
                "Have mercy, Lord, for we are sinners in your sight.\n",
                "Again and again, you called us to return. Through prophets and sages you revealed your righteous Law. And in the fullness of time you sent your only Son, born of a woman, to fulfill your Law, to open for us the way of freedom and peace.",
                "By his blood, he reconciled us.\nBy his wounds, we are healed.\n",
                "And therefore we praise you, joining with the heavenly chorus, with prophets, apostles, and martyrs, and with all those in every generation who have looked to you in hope, to proclaim with them your glory, in their unending hymn:"
            ])),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is he who comes in the name of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("And so, Father, we who have been redeemed by him, and\nmade a new people by water and the Spirit, now bring before\nyou these gifts. Sanctify them by your Holy Spirit to be the\nBody and Blood of Jesus Christ our Lord.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it,\nor lay a hand upon it; and at the words concerning the cup, to hold or\nplace a hand upon the cup and any other vessel containing wine to be\nconsecrated.")),
            Document::from(Text::from("On the night he was betrayed he took bread, said the\nblessing, broke the bread, and gave it to his friends, and\nsaid, “Take, eat: This is my Body, which is given for you. Do\nthis for the remembrance of me.”\n\nAfter supper, he took the cup of wine, gave thanks, and\nsaid, “Drink this, all of you: This is my Blood of the new\nCovenant, which is shed for you and for many for the\nforgiveness of sins. Whenever you drink it, do this for the\nremembrance of me.”")),
            Document::from(ResponsivePrayer::from([
                "Remembering now his work of redemption, and offering to\nyou this sacrifice of thank",
                "We celebrate his death and resurrection,\nas we await the day of his coming."
            ])),
            Document::from(Text::from("Lord God of our Fathers; God of Abraham, Isaac, and\nJacob; God and Father of our Lord Jesus Christ: Open our\neyes to see your hand at work in the world about us. Deliver\nus from the presumption of coming to this Table for solace\nonly, and not for strength; for pardon only, and not for\nrenewal. Let the grace of this Holy Communion make us one\nbody, one spirit in Christ, that we may worthily serve the\nworld in his name.\n\nRisen Lord, be known to us in the breaking of the Bread.\nAccept these prayers and praises, Father, through Jesus\nChrist our great High Priest, to whom, with you and the\nHoly Spirit, your Church gives honor, glory, and worship,\nfrom generation to generation.").response("AMEN."))
    ]));
    pub static ref PRAYER_D: Document = Document::new()
        .label("Eucharistic Prayer D")
        .version_label("Prayer D")
        .page(372)
        .version(Version::RiteII)
        .content(Series::from(vec![
            Document::from(Rubric::from("The people remain standing. The Celebrant, whether bishop or priest, faces them and sings or says")),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("Pueblo", "And also with you."),
                ("Celebrant", "Lift up your hearts."),
                ("Pueblo", "We lift them to the Lord."),
                ("Celebrant", "Let us give thanks to the Lord our God."),
                ("Pueblo", "It is right to give him thanks and praise.")
            ])),
            Document::from(Rubric::from("Then, facing the Holy Table, the Celebrant proceeds")),
            Document::from(Text::from("It is truly right to glorify you, Father, and to give you thanks; for you alone are God, living and true, dwelling in light inaccessible from before time and for ever.\n\nFountain of life and source of all goodness, you made all things and fill them with your blessing; you created them to rejoice in the splendor of your radiance.\n\nCountless throngs of angels stand before you to serve you night and day; and, beholding the glory of your presence, they offer you unceasing praise. Joining with them, and giving voice to every creature under heaven, we acclaim you, and glorify your Name, as we sing (say),")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("Holy, holy, holy Lord, God of power and might,\nheaven and earth are full of your glory.\n\tHosanna in the highest.\nBlessed is he who comes in the name of the Lord.\n\tHosanna in the highest.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The people stand or kneel.\n\nThen the Celebrant continues")),
            Document::from(Text::from("We acclaim you, holy Lord, glorious in power. Your mighty works reveal your wisdom and love. You formed us in your own image, giving the whole world into our care, so that, in obedience to you, our Creator, we might rule and serve all your creatures. When our disobedience took us far from you, you did not abandon us to the power of death. In your mercy you came to our help, so that in seeking you we might find you. Again and again you called us into covenant with you, and through the prophets you taught us to hope for salvation.\n\nFather, you loved the world so much that in the fullness of time you sent your only Son to be our Savior. Incarnate by the Holy Spirit, born of the Virgin Mary, he lived as one of us, yet without sin. To the poor he proclaimed the good news of salvation; to prisoners, freedom; to the sorrowful, joy. To fulfill your purpose he gave himself up to death; and, rising from the grave, destroyed death, and made the whole creation new.\n\nAnd, that we might live no longer for ourselves, but for him who died and rose for us, he sent the Holy Spirit, his own first gift for those who believe, to complete his work in the world, and to bring to fulfillment the sanctification of all.")),
            Document::from(Rubric::from("At the following words concerning the bread, the Celebrant is to hold it or lay a hand upon it; and at the words concerning the cup, to hold or place a hand upon the cup and any other vessel containing wine to be consecrated.")),
            Document::from(Text::from("When the hour had come for him to be glorified by you, his heavenly Father, having loved his own who were in the world, he loved them to the end; at supper with them he took bread, and when he had given thanks to you, he broke it, and gave it to his disciples, and said, “Take, eat: This is my Body, which is given for you. Do this for the remembrance of me.”\n\nAfter supper he took the cup of wine; and when he had given thanks, he gave it to them, and said, “Drink this, all of you. This is my Blood of the new Covenant, which is shed for you and for many for the forgiveness of sins. Whenever you drink it, do this for the remembrance of me.”\n\nFather, we now celebrate this memorial of our redemption. Recalling Christ’s death and his descent among the dead, proclaiming his resurrection and ascension to your right hand, awaiting his coming in glory; and offering to you, from the gifts you have given us, this bread and this cup, we praise you and we bless you.")),
            Document::from(Rubric::from("Celebrant and People")),
            Document::from(Text::from("We praise you, we bless you,\nwe give thanks to you,\nand we pray to you, Lord our God.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Celebrant continues")),
            Document::from(Text::from("Lord, we pray that in your goodness and mercy your Holy Spirit may descend upon us, and upon these gifts, sanctifying them and showing them to be holy gifts for your holy people, the bread of life and the cup of salvation, the Body and Blood of your Son Jesus Christ.\n\nGrant that all who share this bread and cup may become one body and one spirit, a living sacrifice in Christ, to the praise of your Name.\n\nRemember, Lord, your one holy catholic and apostolic Church, redeemed by the blood of your Christ. Reveal its unity, guard its faith, and preserve it in peace.\n\n[Remember (*NN.* and) all who minister in your Church.]\n[Remember all your people, and those who seek your truth.]\n[Remember ___________.]\n[Remember all who have died in the peace of Christ, and\nthose whose faith is known to you alone; bring them into\nthe place of eternal joy and light.]\n\nAnd grant that we may find our inheritance with [the Blessed Virgin Mary, with patriarchs, prophets, apostles, and martyrs, (with _________) and] all the saints who have found favor with you in ages past. We praise you in union with them and give you glory through your Son Jesus Christ our Lord.\n\nThrough Christ, and with Christ, and in Christ, all honor and glory are yours, Almighty God and Father, in the unity of the Holy Spirit, for ever and ever.").response("AMEN."))
    ]));
}
