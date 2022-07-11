use crate::{
    loc,
    rite2::eucharist::parallel::{self, *},
};
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
            Document::new().language(Language::Es).version(Version::LibroDeOracionComun)
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
                    Document::new().label("Primera Lectura")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Preference(PreferenceKey::Global(GlobalPref::ReadingA)),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Text::from("Lectura de {{long_name}}."))))
                    }).tags([FIRST_LESSON]),
                    Document::from(Preces::from([
                        ("", "Palabra del Señor."),
                        ("Pueblo", "Demos gracias a Dios.")
                    ])),
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
                    Document::from(Preces::from([
                        ("", "Palabra del Señor."),
                        ("Pueblo", "Demos gracias a Dios.")
                    ])).condition(Condition::Not(Box::new(Condition::Preference(PreferenceKey::from(GlobalPref::ReadingB), PreferenceValue::from(ReadingType::Empty))))),
                    Document::new().label("Evangelio")
                        .content(LectionaryReading {
                        reading_type: ReadingTypeTable::Selected(ReadingType::Gospel),
                        reading_type_overridden_by: None,
                        lectionary: LectionaryTableChoice::Preference(PreferenceKey::Global(GlobalPref::Lectionary)),
                        intro: Some(BiblicalReadingIntroTemplate::from(Document::from(Preces::from([
                        ("", "Santo Evangelio de nuestro Señor Jesucristo, según"),
                        ("Pueblo", "¡Gloria a ti, Cristo Señor!")
                    ])))),
                }).tags([GOSPEL]),
                Document::from(Preces::from([
                    ("", "El Evangelio del Señor."),
                    ("Pueblo", "Te alabamos, Cristo Señor.")
                ])),
            ])),
            Document::from(Heading::from((HeadingLevel::Heading3, "Sermón"))).tags([SERMON]),
            Document::from(Rubric::from("Los domingos, y en otras Fiestas Mayores, todos de pie, dicen:")).tags([NICENE_CREED]),
            NICENE_CREED_ES.clone().tags([NICENE_CREED]),

            Document::from(Heading::from((HeadingLevel::Heading3, "Oración de los Fieles"))).tags([parallel::PRAYERS_OF_THE_PEOPLE]),
            Document::from(Rubric::from("La oración se ofrece con intercesiones por:\n\nLa Iglesia Universal, sus miembros y su misión\nLa Nación y sus autoridades\nEl bienestar del mundo\nLos intereses de la comunidad local\nLos que sufren y los atribulados\nLos difuntos (con la conmemoración de un santo cuando sea apropiado).\n\nVéanse las fórmulas que comienzan en la página 305.")).tags([POP_RUBRIC]),
            Document::from(Content::DocumentLink {
                label: "Oración de los Fieles".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::PrayersOfThePeople, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: false,
                link_only: false
            }).tags([POP_FORMS]),
            Document::from(Choice::from(loc::eucharist::PRAYERS_OF_THE_PEOPLE.clone())),

            Document::from(Rubric::from("Si no hay celebración de la Comunión, o si no hay sacerdote, el rito concluye como se indica en la página 329.")).tags([NO_COMMUNION_RUBRIC]),
            Document::from(Heading::from((HeadingLevel::Heading3, "Confesión de Pecado"))).tags([CONFESSION]),
            Document::from(Rubric::from("Si no se ha hecho antes la Confesión de Pecado, se hace aquí. En ciertas ocasiones la Confesión puede omitirse.")).tags([CONFESSION]),
            Document::from(Rubric::from("Puede decirse uno de los versículos del Orden Penitencial, en la página 273.")).tags([CONFESSION]),
            Document::from(Content::DocumentLink {
                label: "Versículos del Orden Penitencial".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::PenitentialSentences, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: false,
                link_only: false
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
                rotate: false,
                link_only: false
            }).version(Version::RiteII).tags([OFFERTORY]),

            Document::from(Rubric::from("Durante el Ofertorio puede cantarse un himno, salmo o antífona.")).tags([OFFERTORY_HYMN]),
            Document::from(Content::HymnLink(HymnLink::Hymnals)).tags([OFFERTORY_HYMN]),
            Document::from(Rubric::from("Representantes de la congregación traen al diácono o al celebrante las ofrendas del pueblo de pan y vino, y de dinero u otros dones. El pueblo se pone de pie mientras se presentan las ofrendas y se colocan sobre el Altar.")).tags([OFFERTORY_RUBRIC_2]),

            Document::from(Heading::from((HeadingLevel::Heading3, "La Gran Plegaria Eucarística"))).tags([GREAT_THANKSGIVING_HEADER]),
            Document::from(Content::DocumentLink {
                label: "Plegarias Eucarísticas".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::GreatThanksgiving, Slug::Version(Version::LibroDeOracionComun)]),
                rotate: false,
                link_only: false
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
            Document::from(HymnLink::TagWithLabel("Fraction".into(), "Antífonas".into())).tags([FRACTION_ANTHEM_RUBRIC]),
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
                rotate: false,
                link_only: false
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
        ]).preferences([
            // Translations
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::BibleVersion),
                "Bible Version",
                [
                    LiturgyPreferenceOption::from(Version::RV09),
                    LiturgyPreferenceOption::from(Version::NRSV),
                    LiturgyPreferenceOption::from(Version::CEB),
                    LiturgyPreferenceOption::from(Version::ESV),
                    LiturgyPreferenceOption::from(Version::KJV)
                ]
            )).category("Translations"),

            // Lectionary
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::Lectionary),
                "Lectionary",
                [
                    LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
                    LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
                ]
            )).category("Lectionary"),

            // Readings
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::ReadingA),
                "First Lesson",
                [
                    LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
                    LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
                ]
            )).default_value(PreferenceValue::from(ReadingType::FirstReading)).category("Lessons"),
            LiturgyPreference::from((
                PreferenceKey::from(GlobalPref::ReadingB),
                "Second Lesson",
                [
                    LiturgyPreferenceOption::from(("—", PreferenceValue::from(ReadingType::Empty))),
                    LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
                ]
            )).default_value(PreferenceValue::from(ReadingType::SecondReading)).category("Lessons"),
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
                rotate: true,
                link_only: false
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

    pub static ref PRAYER_B: Document = Document::new()
        .label("Plegaria Eucarística B")
        .version_label("Plegaria B")
        .language(Language::Es)
        .source(Reference {
                    source: Source::LibroDeOracionComun,
                    page: 289
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
                rotate: true,
                link_only: false
            }),
            Document::from(Text::from("Por tanto te alabamos, uniendo nuestras voces con los Angeles y Arcángeles, y con todos los coros c elestiales que, proclamando la gloria de tu Nombre, por siempre cantan este himno:")),
            Document::from(Rubric::from("Celebrante y Pueblo:")),
            Document::from(Text::from("Santo, santo, santo es el Señor, Dios del universo.\nLlenos están el cielo y la tierra de tu gloria.\n\tHosanna en el cielo.\nBendito el que viene en nombre del Señor.\n\tHosanna en el cielo.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("El pueblo permanece de pie o se arrodilla.\n\nEl Celebrante continúa:")),
            Document::from(Text::from("Te damos gracias, oh Dios, por la bondad y el amor que tú nos has manifestado en la creación; en el llamado a Israel para ser tu pueblo; en tu Verbo r evelado a través de los profetas; y, sobre todo, en el Verbo hecho carne, Jesús, tu Hijo. Pues en la plenitud de los tiempos le has enviado para que se encarnara de María la Virgen a fin de ser el Salvador y Redentor del mundo. En él, nos has librado del mal, y nos has hecho dignos de estar en tu presencia. En él, nos has sacado del error a la verdad, del pecado a la rectitud, y de la muerte a la vida.")),
            Document::from(Rubric::from("Al decir las palabras relativas al pan, el Celebrante lo toma en sus manos o impone una mano sobre él; y al decir las palabras relativas al cáliz, lo toma en sus manos o impone una mano sobre él y sobre cualquier otro recipiente con vino que hubiere de consagrarse.")),
            Document::from(Text::from("En la víspera de su muerte por nosotros, nuestro Señor Jesucristo tomó pan; y dándote gracias, lo partió y lo dio a sus discípulos, y dijo: “Tomen y coman. Este es mi Cuerpo, entregado por ustedes. Hagan esto como memorial mío.”\n\nDespués de la cena tomó el cáliz; y dándote gracias, se lo entregó, y dijo: “Beban todos de él. Esta es mi San gre del nuevo Pacto, sangre derramada por ustedes y por muchos para el perdón de los pecados. Siempre que lo beban, háganlo como memorial mío.”\n\nPor tanto, oh Padre, según su mandato,")),
            Document::from(Rubric::from("Celebrante y Pueblo:")),
            Document::from(Text::from("Recordamos su muerte,\nProclamamos su resurrección,\nEsperamos su venida en gloria;").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("El Celebrante continúa:")),
            Document::from(Text::from("Y te ofrecemos nuestro sacrificio de alabanza y acción de gracias, Señor de todos; ofreciéndote, de tu creación, este pan y este vino.\n\nTe suplicamos, Dios bondadoso, que envíes tu Espíritu Santo sobre estos dones, para que sean el Sacramento del Cuerpo de Cristo y su Sangre del nuevo Pacto. Unenos a tu Hijo en su sacrificio, a fin de que, por medio de él, seamos aceptables, siendo santificados por el Espíritu Santo. En la plenitud de los tiempos, sujeta todas las cosas a tu Cristo y llévanos a la patria celestial donde, con [_______ y] todos tus santos, entremos en la herencia eterna de tus hijos; por Jesucristo nuestro Señor, el primogénito de toda la creación, la cabeza de la Iglesia, y el autor de nuestra salvación.\n\nPor él, y con él y en él, en la unidad del Espíritu Santo, tuyos son el honor y la gloria, Padre omnipotente, ahora y por siempre.").response("AMEN."))
    ]));

    pub static ref PRAYER_C: Document = Document::new()
        .label("Plegaria Eucarística C")
        .version_label("Plegaria C")
        .language(Language::Es)
        .source(Reference {
            source: Source::LibroDeOracionComun,
            page: 292
        })
        .content(Series::from(vec![
            Document::from(Text::from("")),
            Document::from(Rubric::from("En esta plegaria, las líneas en cursiva son dichas por el Pueblo.")),
            Document::from(Rubric::from("El Celebrante, sea obispo o sacerdote, de cara al pueblo, canta o dice:")),
            Document::from(ResponsivePrayer::from([
                "El Señor sea con ustedes.",
                "Y con tu espíritu.\n",
                "Elevemos los corazones.",
                "Los elevamos al Señor.\n",
                "Demos gracias a Dios nuestro Señor.",
                "Es justo darle gracias y alabanza.\n"
            ])),
            Document::from(Rubric::from("El Celebrante continúa:")),
            Document::from(ResponsivePrayer::from([
                "Dios de todo poder, Soberano del universo, tú eres digno de gloria y alabanza.",
                "Gloria a ti, ahora y por siempre.\n",
                "A tu mandato, todas las cosas lleg aron a ser: la vasta extensión del espacio interestelar, las galaxias, los soles, los planetas en su trayectoria, y esta frágil tierra, nuestro hogar insular.",
                "Por tu voluntad fueron creadas y tienen su ser.\n",
                "De los elementos primarios formaste la raza huma na y nos bendijiste con la memoria, la razón y la destreza. Nos hiciste soberanos de la creación. Mas nos volvimos contra ti, traicionando tu confianza, y también nos volvimos unos contra otros.",
                "Ten misericordia, Señor, porque somos pecadores delante de ti.\n",
                "Una y otra vez, nos llamaste a regresar. Por los profetas y los sabios, nos revelaste tu justa Ley. Y en la plenitud de los tiempos enviaste a tu único Hijo, nacido de mujer, para cumplir tu Ley, y abrirnos el camino de libertad y paz.",
                "Por su sangre nos ha reconciliado.\nPor sus heridas somos sanados.\n",
                "Por tanto te alabamos, uniéndonos a los coros celestiales, con los profetas, apóstoles y mártires, y con aquéllos de todas las generaciones que te han buscado con esperanza, para proclamar con ellos el incesante himno de tu gloria:"
            ])),
            Document::from(Rubric::from("Celebrante y Pueblo:")),
            Document::from(Text::from("Santo, santo, santo es el Señor, Dios del universo.\nLlenos están el cielo y la tierra de tu gloria.\n\tHosanna en el cielo.\nBendito el que viene en nombre del Señor.\n\tHosanna en el cielo.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("El Celebrante continúa:")),
            Document::from(Text::from("Y así, Padre, los que hemos sido redimidos por él y hechos un pueblo nuevo por medio del agua y del Espíritu, traemos ahora ante ti estos dones. Santifícalos por tu Espíritu Santo para que sean el Cuerpo y la Sangre de nuestro Señor Jesucristo.")),
            Document::from(Rubric::from("Al decir las palabras relativas al pan el Celebrante lo toma en sus manos o impone una mano sobre el; y al decir las palabras relativas al cáliz lo toma en sus manos o impone una mano sobre él y sobre cualquier otro recipiente con vino que hubiere de consagrarse.")),
            Document::from(Text::from("En la noche en que fue traicionado, tomó pan, dijo la bendición, partió el pan y lo dio a sus amigos, y dijo: “Tomen y coman. Este es mi Cuerpo, entregado por ustedes. Hagan esto como memorial mío.”\n\nDespués de la cena tomó el cáliz, dio gracias, y dijo: “Beban todos de él. Esta es mi Sangre del nuevo Pacto, sangre derramada por ustedes y por muchos para el perdón de los pecados. Siempre que lo beban, háganlo como memorial mío.”")),
            Document::from(ResponsivePrayer::from([
                "Recordando ahora su obra de redención, y ofreciéndote este sacrificio de ac ción de gracias,",
                "Celebramos su muerte y resurrección,\nmientras esperamos el día de su venida."
            ])),
            Document::from(ResponsivePrayer::from([
                "Señor Dios de nuestros Padres; Dios de Abrahán, Isaac y Jacob; Dios y Padre de nuestro Señor Jesucristo: Abre nuestros ojos para ver tu mano en el mundo que nos rodea. Líbranos de la presunción de acercarnos a esta Mesa buscando sólo consuelo y no fortaleza; buscando sólo perdón y no renovación. Que la gracia de esta Santa Comunión nos haga un solo cuerpo, un solo espíritu en Cristo, a fin de que dignamente sirvam os al mundo en su nombre.",
                "Señor resucitado, muéstrate a nosotros en la fracción del Pan."
            ])),
            Document::from(Text::from("Padre, acepta estas plegarias y alabanzas, por Jesucristo,\nnuestro gran Sumo Sacerdote, a quien contigo y el\nEspíritu Santo, tu Iglesia rinde honor, gloria y adoración\nde generación en generación.").response("AMEN."))
    ]));

    pub static ref PRAYER_D: Document = Document::new()
        .label("Plegaria Eucarística D")
        .version_label("Plegaria D")
        .language(Language::Es)
        .source(Reference {
            source: Source::LibroDeOracionComun,
            page: 292
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
            Document::from(Text::from("En verdad, oh Padre, es justo glorificarte y darte gracias; porque sólo tú eres Dios, vivo y verdadero, morando en luz inaccesible desde siempre y para siempre.\n\nFuente de vida y toda bondad, hiciste todas las cosas y las colmaste de tu bendición; tú las creaste para que se regocijen en el esplendor de tu gloria.\n\nInnumerables ángeles están delante de ti para servirte noche y día; y contemplando la gloria de tu presencia, te ofrecen alabanza sin cesar. Y con ellos, también nosotros, y por nuestra voz las demás criaturas bajo el cielo, te aclamamos y glorificamos tu Nombre, cantando (diciendo):")),
            Document::from(Rubric::from("Celebrante y Pueblo:")),
            Document::from(Text::from("Santo, santo, santo es el Señor, Dios del universo.\nLlenos están el cielo y la tierra de tu gloria.\n\tHosanna en el cielo.\nBendito el que viene en nombre del Señor.\n\tHosanna en el cielo.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("El pueblo permanece de pie o se arrodilla.\n\nEl Celebrante continúa:")),
            Document::from(Text::from("Te aclamamos, santo Señor, glorioso en poder. Tus grandes obras revelan tu sabiduría y amor. Nos formaste a tu propia Imagen, encomendándonos el mundo entero, para que, en obediencia a ti, nuestro Creador, pudiéramos regir y servir a todas tus criaturas. Cuando por desobediencia nos alejamos de ti, no nos abandonaste al poder de la muerte. En tu misericordia, viniste en nuestra ayuda, para que buscándote, te encontráramos. Una y otra vez nos has llamado al pacto contigo, y por los profetas nos enseñaste la esperanza de salvación.\n\nTanto amaste al mundo, Padre, que en la plenitud del tiempo nos enviaste como Salvador a tu único Hijo. Encarnado por obra del Espíritu Santo y nacido de María, la Virgen, vivió como uno de nosotros, empero sin pecado. A los pobres proclamó las buenas nuevas de salvación; a los prisioneros, libertad; a los afligidos, gozo. Para cumplir tus designios, se entreg ó a la muerte y, resucitando de la tumba, destruyó la muerte e hizo nueva la creación entera.\n\nY a fin de que no viviésemos más para nosotros mismos, sino para él, que por nosotros murió y resucitó, envió al Espíritu Santo como su primicia a los que creen, para completar su obra en el mundo y llevar a plenitud la santificación de todos.")),
            Document::from(Rubric::from("Al decir las palabras relativas al pan, el Celebrante lo toma en sus manos o impone una mano sobre él; y al decir las palabras relativas al cáliz, lo toma en sus manos o impone una mano sobre él y sobre cualquier otro recipiente con vino que hubiere de consagrarse.")),
            Document::from(Text::from("Llegada la hora en que había de ser glorificado por ti, su Padre celestial, habiendo amado a los suyos que estaban en el mundo, los amó hasta el final; y mie ntras cenaba con ellos, tomó pan, y dándote gracias, lo partió y se lo dio a sus discípulos, y dijo: “Tomen y coman. Este es mi Cuerpo, entregado por ustedes. Hagan esto como memorial mío.”\n\nDespués de la cena tomó el cáliz; y dándote gracias, se lo entregó, y dijo: “Beban todos de él. Esta es mi Sangre del nuevo Pacto, sangre derramada por ustedes y por muchos para el perdón de los pecados. Siempre que lo beban, háganlo como memorial mío.”\n\nPadre, celebramos ahora este memorial de nuestra redención. Recordando la muerte de Cristo y su descenso entre los muertos, proclamando su resurrección y ascensión a tu derecha, esperando su venida en gloria; y ofreciéndote, de las dádivas que tú nos has dado, este pan y este cáliz, te alabamos y te bendecimos.")),
            Document::from(Rubric::from("Celebrante y Pueblo:")),
            Document::from(Text::from("Te alabamos, te bendecimos,\nte damos gracias,\ny oramos a ti, Señor nuestro Dios.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("El Celebrante continúa:")),
            Document::from(Text::from("Señor, te rogamos que en tu bondad y misericordia, tu Espíritu Santo descienda sobre nosotros y sobre estos dones, santificándolos y mostrando que son dones santos para tu pueblo santo, el pan de vida y el cáliz de salvación, el Cuerpo y la Sangre de tu Hijo Jesucristo.\n\nConcede que todos los que compartan este pan y este cáliz sean un solo cuerpo y un solo espíritu, un sacrificio vivo en Cristo, para alabanza de tu Nombre.\n\nRecuerda, Señor, a tu Iglesia, una, santa, católica y apostólica, redimida por la sangre de tu Cristo. Manifiesta su unidad, guarda su fe y presérvala en paz.\n\n[Recuerda a (*NN.* y) todos los que ministran en tu Iglesia.]\n[Recuerda a todo tu pueblo y a aquéllos que buscan tu verdad.]\n[Recuerda a ______.]\n[Recuerda a todos los que han muerto en la paz de Cristo y a aquéllos cuya fe sólo tu conoces; llévalos al lugar de eterno gozo y luz.]\n\nY concede que alcancemos nuestra herencia con [la Bendita Virgen María, con los patriarcas, profetas, apóstoles y mártires, (con _______) y] todos los santos que han encontrado favor contigo en tiempos pasados. Junto con ellos te alabamos y te damos gloria, por tu Hijo Jesucristo nuestro Señor.\n\nPor Cristo, y con Cristo y en Cristo, tuyos son el honor y la gloria, omnipotente Dios y Padre, en la unidad del Espíritu Santo, por los siglos de los siglos.").response("AMEN."))
    ]));

    pub static ref PRAYERS_OF_THE_PEOPLE: Vec<Document> = vec![
        pop::FORM_I.clone(),
        pop::FORM_II.clone(),
        pop::FORM_III.clone(),
        pop::FORM_IV.clone(),
        pop::FORM_V.clone(),
        pop::FORM_V_KYRIE.clone(),
        pop::FORM_VI.clone()
    ];
}

mod pop {
    use language::Language;
    use liturgy::*;

    lazy_static! {
        pub static ref FORM_I: Document = Document::new()
            .label("Fórmula I")
            .language(Language::Es)
            .source(Reference {
                source: Source::LibroDeOracionComun,
                page: 305
            })
            .version(Version::LibroDeOracionComun)
            .content(Series::from(vec![
                Document::from(Rubric::from("El Diácono u otra persona:")),
                Document::from(Text::from("Con todo el corazón y con toda la mente, oremos al Señor, diciendo: “Señor, ten piedad.”")),
                Document::from(Litany::from((
                    "Señor, ten piedad.",
                    [
                        "| Por la paz de lo alto, por la misericordia de Dios y por la salvación de nuestras almas, oremos al Señor.",
                        "Por la paz del mundo, por el bienestar de la santa Iglesia de Dios y por la unidad de todos los pue blos, oremos al Señor.",
                        "Por nuestro Obispo, y por todos los clérigos y laicos, oremos al Señor.",
                        "Por nuestro Presidente, por los gobernantes de las naciones y por todas las autoridades, oremos al Señor.",
                        "Por esta ciudad (pueblo, aldea, _______), por todas las ciudades y comunidades, y por los que viven en ellas, oremos al Señor.",
                        "| Por un clima apacible y por la abundancia de los frutos de la tierra, oremos al Señor.",
                        "Por la buena tierra que Dios nos ha dado, y por la sabiduría y el deseo de conservarla, oremos al Señor.",
                        "| Por todos los que viajan por tierra, mar o aire [o el espacio], oremos al Señor.",
                        "Por los ancianos e inváli dos, los viudos y huérfanos, por los enfermos y los que yacen en el lecho del dolor, oremos al Señor.",
                        "| Por ______, oremos al Señor.",
                        "Por los pobres y oprimidos, por los desempleados e indigentes, por los encarcelados y cautivos, y por todos los que se acuerdan y cuidan de ellos, oremos al Señor.",
                        "Por todos los que han muerto en la esperanza de la resurrección y por todos los difuntos, oremos al Señor.",
                        "Por la liberación de todo peligro, violencia, opresión y degradación, oremos al Señor.",
                        "| Por la absolución y remisión de nuestros pecados y ofensas, oremos al Señor.",
                        "Para que terminemos nuestra vida en fe y esperanza, sin sufrimiento ni reproche, oremos al Señor.",
                        "| Defiéndenos, líbranos, y en tu compasión protégenos, oh Señor, por medio de tu gracia."
                    ]
                ))),
                Document::from(ResponsivePrayer::from([
                    "En la comunión de [____ y de todos] los santos, encomendémonos los unos a los otros, y toda nuestra vida a Cristo nuestro Dios.",
                    "A ti, Señor nuestro Dios."
                ])),
                Document::from(Rubric::from("Silencio")),
                Document::from(Rubric::from("El Celebrante añade una Colecta final."))
        ]));

        pub static ref FORM_II: Document = Document::new()
            .label("Fórmula II")
            .language(Language::Es)
            .source(Reference {
                source: Source::LibroDeOracionComun,
                page: 307
            })
            .version(Version::LibroDeOracionComun)
            .content(Series::from(vec![
                Document::from(Rubric::from("Durante la pausa que sigue a cada invitación, el Pueblo ofrece sus propias peticiones en silencio o en voz alta.")),
                Document::from(Text::from("Pido sus oraciones por el pueblo de Dios esparcido por todo el mundo; por ______, nuestro(s) Obispo(s); por esta asamblea; y por todos los ministros y fieles.\nOren por la Iglesia.")),
                Document::from(Rubric::from("Pausa")),
                Document::from(Text::from("Pido sus oraciones por la paz; por la concordia entre las naciones y por el bienestar de todos los pueblos.\nOren por la justicia y la paz.")),
                Document::from(Rubric::from("Pausa")),
                Document::from(Text::from("Pido sus oraciones por los pobres, los enfermos, los hambrientos, los oprimidos y los prisioneros.\nOren por los que se hallan en necesidad o tribulación.")),
                Document::from(Rubric::from("Pausa")),
                Document::from(Text::from("Pido sus oraciones por cuantos buscan a Dios o un conocimiento más profundo de él.\nOren para que le encuentren y sean encontrados por él.")),
                Document::from(Rubric::from("Pausa")),
                Document::from(Text::from("Pido sus oraciones por los que han partido de esta vid. [especialmente por _________].\nOren por los difuntos.")),
                Document::from(Rubric::from("Pausa")),
                Document::new()
                    .optional()
                    .content(Series::from(vec![
                        Document::from(Rubric::from("Los miembros de la congregación pueden pedir a los presentes oraciones o acciones de gracias.")),
                        Document::from(Text::from("Pido sus oraciones por _________.\nPido que den gracias por ________.")),
                        Document::from(Rubric::from("Pausa"))
                    ])),
                Document::from(Text::from("Alaben a Dios por aquéllos de todas las generaciones en quienes Cristo ha sido glorificado [especialmente _____________, a quien recordamos hoy].\nOren para que también nosotros recibamos la gracia de glorificar a Cristo en nuestro tiempo.")),
                Document::from(Rubric::from("Pausa")),
                Document::from(Rubric::from("El Celebrante añade una Colecta final.")),
        ]));

        pub static ref FORM_III: Document = Document::new()
            .label("Fórmula III")
            .language(Language::Es)
            .source(Reference {
                source: Source::LibroDeOracionComun,
                page: 309
            })
            .version(Version::LibroDeOracionComun)
            .content(Series::from(vec![
                Document::from(Rubric::from("El que dirige y el Pueblo oran en forma dialogada.")),
                Document::from(ResponsivePrayer::from([
                    "Padre, te suplicamos por tu santa Iglesia Católica.",
                    "Que todos seamos uno.\n",
                    "Concede que todos los miembros de la Iglesia te sirvan en verdad y humildad.",
                    "Que tu Nombre sea glorificado por todo el género humano.\n",
                    "Te pedimos por todos los obispos, presbíteros y diáconos.",
                    "Que sean fieles ministros de tu Palabra y Sacramentos.\n",
                    "Te pedimos por cuantos gobiernan y ejercen autoridad en todas las naciones del mundo.",
                    "Que haya justicia y paz en la tierra.\n",
                    "Danos gracia para hacer tu voluntad en todo cuanto emprendamos.",
                    "Que nuestras obras sean agradables a tus ojos.\n",
                    "Ten compasión de los que sufren de dolor o angustia.",
                    "Que sean librados de sus aflicciones.\n",
                    "Otorga descanso eterno a los difuntos.",
                    "Que sobre ellos resplandezca la luz perpetua.\n",
                    "Te alabamos por tus santos que han entrado en el gozo del Señor.",
                    "Que también nosotros tengamos parte en tu reino celestial."
                ])),
                Document::from(Text::from("Oremos por nuestras necesidades y las necesidades de los demás.")),
                Document::from(Rubric::from("Pausa\n\nEl Pueblo puede añadir sus propias peticiones.\n\nEl Celebrante añade una Colecta final."))
        ]));

        pub static ref FORM_IV: Document = Document::new()
            .label("Fórmula IV")
            .language(Language::Es)
            .source(Reference {
                source: Source::LibroDeOracionComun,
                page: 310
            })
            .version(Version::LibroDeOracionComun)
            .content(Series::from(vec![
                Document::from(Rubric::from("El Diácono u otra persona:")),
                Document::from(Text::from("Oremos por la Iglesia y por el mundo.")),
                Document::from(Text::from("Omnipotente Dios, concede que cuantos confesamos tu Nombre estemos unidos en tu verdad, vivamos unánimes en tu amor y manifestemos tu gloria en el mundo.")),
                Document::from(Rubric::from("Silencio")),
                Document::from(ResponsivePrayer::from([
                    "Señor, en tu misericordia",
                    "atiende nuestra súplica."
                ])),
                Document::from(Text::from("Dirige al pueblo de este país y de todas las naciones por caminos de justicia y paz, para que nos respetemos unos a otros y procuremos el bien común.")),
                Document::from(Rubric::from("Silencio")),
                Document::from(ResponsivePrayer::from([
                    "Señor, en tu misericordia",
                    "atiende nuestra súplica."
                ])),
                Document::from(Text::from("Danos reverencia por la tierra, que es creación tuya, para que utilicemos debidamente sus recursos en servicio de los demás y para tu honra y gloria.")),
                Document::from(Rubric::from("Silencio")),
                Document::from(ResponsivePrayer::from([
                    "Señor, en tu misericordia",
                    "Atiende nuestra súplica."
                ])),
                Document::from(Text::from("Bendice a aquéllos cuyas vidas están unidas a las nuestras, y concede que sirvamos a Cristo en ellos y nos amemos unos a otros, así como él nos ama.")),
                Document::from(Rubric::from("Silencio")),
                Document::from(ResponsivePrayer::from([
                    "Señor, en tu misericordia",
                    "Atiende nuestra súplica."
                ])),
                Document::from(Text::from("Consuela y sana a todos aquéllos que sufren en cuerpo, mente o espíritu; en sus tribulaciones dales valor y esperanza, y llévalos al gozo de tu salvación.")),
                Document::from(Rubric::from("Silencio")),
                Document::from(ResponsivePrayer::from([
                    "Señor, en tu misericordia",
                    "Atiende nuestra súplica."
                ])),
                Document::from(Text::from("Encomendamos a tu misericordia a todos los difuntos, para que tu voluntad se cumpla en ellos; y te pedimos que nos hagas partícipes con todos tus santos de tu reino eterno.")),
                Document::from(Rubric::from("Silencio")),
                Document::from(ResponsivePrayer::from([
                    "Señor, en tu misericordia",
                    "Atiende nuestra súplica."
                ])),
                Document::from(Rubric::from("El Celebrante añade una Colecta final."))
        ]));

        pub static ref FORM_V: Document = Document::new()
            .label("Fórmula V")
            .language(Language::Es)
            .source(Reference {
                source: Source::LibroDeOracionComun,
                page: 312
            })
            .version(Version::LibroDeOracionComun)
            .content(Series::from(vec![
                Document::from(Rubric::from("El Diácono u otra persona:")),
                Document::from(Text::from("En paz oremos al Señor, diciendo: “Señor, ten piedad.”")),
                Document::from(Litany::from((
                    "Señor, ten piedad.",
                    [
                        "Por la santa Iglesia de Dios, para que esté llena de verdad y amor y se halle sin mancha en el día de tu venida, te suplicamos Señor. ",
                        "Por N. nuestro Primado, por N.(N.) nuestro (s) Obispo (s), por todos los obispos y demás ministros, y por todo el pueblo santo de Dios, te suplicamos Señor.",
                        "Por cuantos temen a Dios y creen en ti, Cristo Señor, para que cesen nuestras divisiones y todos seamos uno, como tú y el Padre son uno, te suplicamos Señor.",
                        "Por la misión de la Iglesia, para que en testimonio fiel proclame el Evangelio hasta los confines de la tierra, te suplicamos Señor.",
                        "| Por los que aún no creen y por los que han perdido la fe, para que reciban la luz del Evangelio, te suplicamos Señor.",
                        "Por la paz del mundo, para que entre las naciones y los pueblos crezca un espíritu de respeto y comprensión, te suplicamos Señor.",
                        "Por los que tienen cargos de responsabilidad pública [especialmente ______], para que sirvan a la justicia y promuevan la dignidad y la libertad de toda persona, te suplicamos Señor.",
                        "| Por todos los que viven y trabajan en esta comunidad [especialmente ______ ], te suplicamos Señor.",
                        "| Por tu bendición sobre todo trabajo humano y por el uso debido de las riquezas de la creación, para que el mundo sea librado de la pobreza, el hambre y el desastre, te suplicamos Señor.",
                        "Por los pobres, los perseguidos, los enfermos y todos cuantos sufren; por los refugiados, los prisioneros y por todos los que están en peligr o, para que hallen alivio y protección, te suplicamos Señor.",
                        "Por esta congregación [por los presentes y los ausentes], para que nos libres de dureza de corazón y manifestemos tu gloria en todo lo que hagamos, te suplicamos Señor.",
                        "| Por nuestros enemigos y por cuantos nos desean el mal; y por aquéllos a quienes hemos agraviado u ofendido, te suplicamos Señor.",
                        "| Por nosotros, por el perdón de nuestros pecados y por la gracia del Espíritu Santo para enmendar nuestras vidas, te suplicamos Señor.",
                        "Por todos los que se han encomendado a nuestras oraciones; por nuestras familias, amigos y vecinos, para que, libres de ansiedad, vivan en gozo, paz y salud, te suplicamos Señor.",
                        "| Por ______, te suplicamos Señor.",
                        "Por cuantos han muerto en la comunión de tu Iglesia, y por aquéllos cuya fe sólo tú conoces, para que con todos tus santos tengan descanso en ese lugar donde no hay dolor ni tristeza, sino vida eterna, te suplicamos Señor."
                    ]
                ))),
                Document::from(ResponsivePrayer::from([
                    "Gozándonos en la comunión de [la siempre Bendita Virgen María, (*del bienaventurado N.*) y] todos los santos, encomendémonos los unos a los otros, y toda nuestra vida, a Cristo nuestro Dios.",
                    "A ti, Señor nuestro Dios."
                ])),
                Document::from(Rubric::from("Silencio")),
                Document::from(Rubric::from("El Celebrante añade una Colecta final o la siguiente Doxología:")),
                Document::from(Text::from("Porque tuya es la majestad, Padre, Hijo y Esp íritu Santo; tuyo es el reino y el poder y la gloria, ahora y por siempre.").response("Amén."))
        ]));

        pub static ref FORM_V_KYRIE: Document = Document::new()
            .label("Fórmula V")
            .language(Language::Es)
            .source(Reference {
                source: Source::LibroDeOracionComun,
                page: 312
            })
            .version(Version::LibroDeOracionComun)
            .version_label("Fórmula V (“Kyrie eleison”)")
            .content(Series::from(vec![
                Document::from(Rubric::from("El Diácono u otra persona:")),
                Document::from(Text::from("En paz oremos al Señor, diciendo: “Señor, ten piedad.”")),
                Document::from(Litany::from((
                    "Kyrie eleison.",
                    [
                        "Por la santa Iglesia de Dios, para que esté llena de verdad y amor y se halle sin mancha en el día de tu venida, te suplicamos Señor. ",
                        "Por N. nuestro Primado, por N.(N.) nuestro (s) Obispo (s), por todos los obispos y demás ministros, y por todo el pueblo santo de Dios, te suplicamos Señor.",
                        "Por cuantos temen a Dios y creen en ti, Cristo Señor, para que cesen nuestras divisiones y todos seamos uno, como tú y el Padre son uno, te suplicamos Señor.",
                        "Por la misión de la Iglesia, para que en testimonio fiel proclame el Evangelio hasta los confines de la tierra, te suplicamos Señor.",
                        "| Por los que aún no creen y por los que han perdido la fe, para que reciban la luz del Evangelio, te suplicamos Señor.",
                        "Por la paz del mundo, para que entre las naciones y los pueblos crezca un espíritu de respeto y comprensión, te suplicamos Señor.",
                        "Por los que tienen cargos de responsabilidad pública [especialmente ______], para que sirvan a la justicia y promuevan la dignidad y la libertad de toda persona, te suplicamos Señor.",
                        "| Por todos los que viven y trabajan en esta comunidad [especialmente ______ ], te suplicamos Señor.",
                        "| Por tu bendición sobre todo trabajo humano y por el uso debido de las riquezas de la creación, para que el mundo sea librado de la pobreza, el hambre y el desastre, te suplicamos Señor.",
                        "Por los pobres, los perseguidos, los enfermos y todos cuantos sufren; por los refugiados, los prisioneros y por todos los que están en peligr o, para que hallen alivio y protección, te suplicamos Señor.",
                        "Por esta congregación [por los presentes y los ausentes], para que nos libres de dureza de corazón y manifestemos tu gloria en todo lo que hagamos, te suplicamos Señor.",
                        "| Por nuestros enemigos y por cuantos nos desean el mal; y por aquéllos a quienes hemos agraviado u ofendido, te suplicamos Señor.",
                        "| Por nosotros, por el perdón de nuestros pecados y por la gracia del Espíritu Santo para enmendar nuestras vidas, te suplicamos Señor.",
                        "Por todos los que se han encomendado a nuestras oraciones; por nuestras familias, amigos y vecinos, para que, libres de ansiedad, vivan en gozo, paz y salud, te suplicamos Señor.",
                        "| Por ______, te suplicamos Señor.",
                        "Por cuantos han muerto en la comunión de tu Iglesia, y por aquéllos cuya fe sólo tú conoces, para que con todos tus santos tengan descanso en ese lugar donde no hay dolor ni tristeza, sino vida eterna, te suplicamos Señor."
                    ]
                ))),
                Document::from(ResponsivePrayer::from([
                    "Gozándonos en la comunión de [la siempre Bendita Virgen María, (*del bienaventurado N.*) y] todos los santos, encomendémonos los unos a los otros, y toda nuestra vida, a Cristo nuestro Dios.",
                    "A ti, Señor nuestro Dios."
                ])),
                Document::from(Rubric::from("Silencio")),
                Document::from(Rubric::from("El Celebrante añade una Colecta final o la siguiente Doxología:")),
                Document::from(Text::from("Porque tuya es la majestad, Padre, Hijo y Esp íritu Santo; tuyo es el reino y el poder y la gloria, ahora y por siempre.").response("Amén."))
        ]));

        pub static ref FORM_VI: Document = Document::new()
            .label("Fórmula VI")
            .source(Reference {
                source: Source::LibroDeOracionComun,
                page: 314
            })
            .version(Version::LibroDeOracionComun)
            .content(Series::from(vec![
                Document::from(Rubric::from("El que dirige y el Pueblo oran en forma dialogada.")),
                Document::from(Text::from("En paz oramos a ti, Señor Dios.")),
                Document::from(Rubric::from("Silencio")),
                Document::from(ResponsivePrayer::from([
                    "Por todos los seres humanos en su vida y trabajo diarios;",
                    "Por nuestras familias, amigos y vecinos, y por los que están solos.\n",
                    "Por esta comunidad, por esta nación, y por el mundo entero;",
                    "Por cuantos trabajan por la justicia, la libertad y la paz.\n",
                    "Por el uso justo y adecuado de tu creación;",
                    "Por las víctimas del hambre, el temor, la injusticia y la opresión.\n",
                    "Por cuantos se hallan en peligro, tristeza, o cualquier otra adversidad;",
                    "Por los que ministran a los enfermos, a los desamparados y a los necesitados.\n",
                    "Por la paz y unidad de la Iglesia de Dios;",
                    "Por todos los que proclaman el Evangelio, y cuantos buscan la Verdad.\n",
                    "Por [N. nuestro Primado, y por N. (N.) nuestro(s) obispo(s), y por] todos los obispos y demás ministros;",
                    "Por todos los que sirven a Dios en su Iglesia."
                ])),
                Document::from(Text::from("Por las necesidades e intereses especiales de esta\ncongregación.")),
                Document::from(Rubric::from("Pausa\n\nEl Pueblo puede añadir sus propias peticiones.")),
                Document::from(ResponsivePrayer::from([
                    "Atiéndenos, Señor;",
                    "Porque grande es tu misericordia."
                ])),
                Document::from(Text::from("Te damos gracias, Señor, por todas las bendiciones de esta vida.")),
                Document::from(Rubric::from("Pausa\n\nEl Pueblo puede añadir sus propias acciones de gracias.")),
                Document::from(ResponsivePrayer::from([
                    "Te exaltaremos, oh Dios nuestro Rey;",
                    "Y alabaremos tu Nombre para siempre."
                ])),
                Document::from(Text::from("Te pedimos por todos los que han muerto, para que\ntengan un lugar en tu reino eterno.")),
                Document::from(Rubric::from("Pausa\n\nEl Pueblo puede añadir sus propias peticiones.")),
                Document::from(ResponsivePrayer::from([
                    "Señor, concédeles tu misericordia;",
                    "Porque en ti han confiado."
                ])),
                Document::new()
                    .optional()
                    .content(Series::from(vec![
                        Document::from(Text::from("También te pedimos por el perdón de nuestros pecados.")),
                        Document::from(Rubric::from("Se puede guardar un período de silencio.\n\nEl que dirige y el Pueblo:")),
                        Document::from(Text::from("Ten misericordia de nosotros, Padre de toda bondad;\nen tu compasión perdona nuestros pecados,\nlos conocidos y los desconocidos;\nlo que hemos hecho y lo que hemos dejado de hacer.\nSustenta a tus siervos con tu Espíritu,\npara que vivamos y te sirvamos en novedad de vida,\npara honra y gloria de tu Nombre;\npor Jesucristo nuestro Señor. Amén.").display_format(DisplayFormat::Unison)),
                ])),
                Document::from(Rubric::from("El Celebrante concluye con una absolución o con una Colecta adecuada."))
        ]));
    }
}
