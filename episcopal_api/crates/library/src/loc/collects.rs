use crate::{CollectData, CollectId};
use calendar::{CommonOfSaints, Feast, LiturgicalWeek, Proper, VariousOccasions};
use language::Language;
use liturgy::{Document, Heading, HeadingLevel, Series, Text, Version};

lazy_static! {
    pub static ref COLECTAS: Vec<(CollectId, CollectData)> = vec![
        (
            CollectId::Week(LiturgicalWeek::Advent1),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, danos gracia para despojarnos de las obras de las tinieblas y revestirnos con las armas de la luz, ahora en esta vida mortal, en la cual Jesucristo tu Hijo, con gran humildad, vino a visitarnos; a fin de que en el día postrero, cuando vuelva con majestad gloriosa a juzgar a vivos y muertos, resucitemos a la vida inmortal; mediante él, quien vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Primer Domingo de Adviento")
                .page(125)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Adviento".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent2),
            CollectData {
                document: Document::from(
                    Text::from("Dios de misericordia, que enviaste a tus mensajeros, los profetas, a predicar el arrepentimiento y preparar el camino de nuestra salvación: Danos gracia para atender sus advertencias y abandonar nuestros pecados, a fin de que recibamos gozosamente la venida de Jesucristo nuestro Redentor; que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Segundo Domingo de Adviento")
                .page(125)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Adviento".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent3),
            CollectData {
                document: Document::from(
                    Text::from("Suscita tu poder, oh Señor, y con gran potencia ven a nosotros; ya que estamos impedidos penosamente por nuestros pecados, haz que tu abundante gracia y misericordia nos ayuden y libren prontamente; por Jesucristo nuestro Señor, a quien contigo y el Espíritu Santo, sea el honor y la gloria, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Tercer Domingo de Adviento")
                .page(126)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Adviento".into(),
                rubric_before: None,
                rubric_after: Some("El miércoles, viernes y sábado de esta semana son los tradicionales Días de Témporas de invierno.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent4),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, te suplicamos que purifiques nuestra conciencia con tu visitación diaria, para que, cuando venga tu Hijo Jesucristo, encuentre en nosotros la mansión que le ha sido preparada; quien vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Cuarto Domingo de Adviento")
                .page(126)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Adviento".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, tú nos alegras anualmente con la festividad del nacimiento de tu único Hijo Jesucristo: Concédenos que, así como le recibimos con júbilo como Redentor, de la misma manera le contemplemos con segura confianza cuando venga a ser nuestro Juez; quien vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Natividad de Nuestro Señor: Día de Navidad")
                    .subtitle("Diciembre 25")
                .page(126)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o la siguiente:".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasEve),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que has hecho resplandecer esta noche santa con la claridad de la Luz verdadera: Concede a los que hemos conocido el misterio de esa Luz en la tierra, que también nos gocemos de él plenamente, en el cielo; donde vive y reina contigo y el Espíritu santo, un solo Dios, en gloria eterna.")
                        .response("Amén.")
                )
                .label("Natividad de Nuestro Señor: Día de Navidad")
                    .subtitle("Diciembre 25")
                .page(127)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after:  Some("o bien:".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("Omnipotente Dios, tú has dado a tu unigénito Hijo para asumir nuestra naturaleza, y nacer [este día] de una virgen pura: Concede que, siendo nacidos de nuevo y hechos tus hijos por adopción y gracia, seamos renovados cada día con tu Espíritu Santo; mediante nuestro Señor Jesucristo, a quien contigo y el mismo Espíritu sea el honor y la gloria, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Natividad de Nuestro Señor: Día de Navidad")
                    .subtitle("Diciembre 25")
                .page(127)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: None,
                rubric_after: Some("La Colecta que precede, y cualquiera de las tres series de Lecciones Propias del Día de Navidad, sirven para cualquier otro día de entre semana desde el Día de los Santos Inocentes hasta el Primer Domingo después del Día de Navidad.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Christmas1),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, tú has derramado sobre nosotros la nueva luz de tu Verbo encarnado: Concede que esta luz, que arde en nuestro corazón, resplandezca en nuestra vida; mediante nuestro Señor Jesucristo, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Primer Domingo después del Día de Navidad")
                .page(127)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: Some("Este domingo tiene preferencia sobre las tres conmemoraciones que siguen al Día de Navidad. Si fuera necesario, la observancia de una, dos o todas ellas ha de ser pospuesta un día.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyName),
            CollectData {
                document: Document::from(
                    Text::from("Padre eterno, tú diste a tu Hijo encarnado el santo nombre de Jesús para ser el signo de nuestra salvación: Te suplicamos que siembres en cada corazón el amor de quien es el Salvador del mundo, nuestro Señor Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, en gloria eterna.")
                        .response("Amén.")
                )
                .label("El Santo Nombre de Jesús")
                    .subtitle("Enero 1")
                .page(128)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Christmas2),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que maravillosamente creaste y aún más maravillosamente restauraste la dignidad de la naturaleza humana: Concede que compartamos la vida divina de quien se humilló para compartir nuestra humanidad, tu Hijo Jesucristo; que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Segundo Domingo después del Día de Navidad")
                .page(128)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Epiphany),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que por la guía de una estrella manifestaste tu único Hijo a los pueblos de la tierra: Guía a tu presencia a los que ahora te conocemos por fe, para que veamos tu gloria cara a cara; mediante Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("La Epifanía")
                    .subtitle("Enero 6")
                .page(128)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía".into(),
                rubric_before: None,
                rubric_after: Some("La Colecta que precede, junto con el Salmo y las Lecciones de la Epifanía, o aquéllos señalados para el Segundo Domingo después del Día de Navidad, sirven para los días de entre semana desde la Epifanía hasta el domingo siguiente. Se usa el Prefacio de Epifanía.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany1),
            CollectData {
                document: Document::from(
                    Text::from("Padre celestial, que en el bautismo de Jesús en el Río Jordán, le proclamaste tu Hijo amado y le ungiste con el Espíritu Santo: Concede que todos los que son bautizados en su Nombre, guarden el pacto que han hecho, y valerosamente le confiesen como Señor y Salvador; quien contigo y el Espíritu Santo vive y reina, un solo Dios, en gloria eterna.")
                        .response("Amén.")
                )
                .label("Primer Domingo después de la Epifanía: Bautismo de Nuestro Señor")
                .page(129)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany2),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, cuyo Hijo nuestro Salvador Jesucristo es la luz del mundo: Concede que tu pueblo, iluminado por tu Palabra y Sacramentos, brille con el resplandor de la gloria de Cristo, para que el sea conocido, adorado y obedecido hasta los confines de la tierra; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Segundo Domingo después de la Epifanía")
                .page(129)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía o del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany3),
            CollectData {
                document: Document::from(
                    Text::from("Danos gracia, Señor, para responder prestamente al llamamiento de nuestro Salvador Jesucristo y proclamar las Buenas Nuevas de su salvación a todos los pueblos; para que nosotros, y todo el mundo, percibamos la gloria de sus obras maravillosas; quien vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Tercer Domingo después de la Epifanía")
                .page(129)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía o del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany4),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, tú riges todas las cosas tanto en el cielo como en la tierra: Escucha con misericordia las súplicas de tu pueblo, y en nuestro tiempo concédenos tu paz; por nuestro Señor Jesucristo, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Cuarto Domingo después de la Epifanía")
                .page(130)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía o del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany5),
            CollectData {
                document: Document::from(
                    Text::from("Líbranos, oh Dios, de la esclavitud de nuestros pecados, y danos la libertad de esa vida abundante que nos has manifestado en tu Hijo, nuestro Salvador Jesucristo; que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Quinto Domingo después de la Epifanía")
                .page(130)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía o del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany6),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, fortaleza de los que ponen su confianza en ti: Acepta con misericordia nuestras súplicas, y puesto que, por nuestra flaqueza, no podemos hacer nada bueno sin ti, danos el auxilio de tu gracia; para que, al guardar tus mandamientos, te agrademos, tanto de voluntad como de hecho; por nuestro Señor Jesucristo, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Sexto Domingo después de la Epifanía")
                .page(130)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía o del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany7),
            CollectData {
                document: Document::from(
                    Text::from("Oh Señor, tú nos has enseñado que todo lo que hacemos sin amor es de ningún valor: Envía tu Espíritu Santo, y derrama en nuestros corazones tu excelentísimo don, que es el amor, el vínculo verdadero de la paz y de todas las virtudes, sin el cual todos aquéllos que viven son considerados como muertos ante ti. Concédenos esto, por amor de tu único Hijo Jesucristo, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Séptimo Domingo después de la Epifanía")
                .page(131)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía o del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany8),
            CollectData {
                document: Document::from(
                    Text::from("Amantísimo Padre, cuya voluntad es que te demos gracias por todas las cosas, que no temamos nada sino el perderte a ti, y que te confiemos todas nuestras preocupaciones, pues cuidas de nosotros: Presérvanos de temores infieles y de ansiedades mundanas, para que ninguna nube de esta vida mortal oculte de nosotros la luz de ese amor inmortal que tú nos has manifestado en tu Hijo Jesucristo nuestro Señor; que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Octavo Domingo después de la Epifanía")
                .page(131)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía o del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::LastEpiphany),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que antes de la pasión de tu unigénito Hijo, revelaste su gloria en el monte santo: Concédenos que, al contemplar por fe la luz de su rostro, seamos fortalecidos para llevar nuestra cruz y ser transformados a su imagen de gloria en gloria; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Ultimo Domingo después de la Epifanía")
                .page(131)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Epifanía".into(),
                rubric_before: Some("Este Propio siempre se usa el domingo antes del Miércoles de Ceniza.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::AshWednesday),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, tú no aborreces nada de lo que has creado, y perdonas los pecados de todos los penitentes: Crea y forma en nosotros, corazones nuevos y contritos, para que, lamentando debidamente nuestros pecados y reconociendo nuestra miseria, obtengamos de ti, Dios de toda misericordia, perfecta remisión y perdón; mediante Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Miércoles de Ceniza")
                .page(132)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Cuaresma".into(),
                rubric_before: Some("La Liturgia Propia de este día se encuentra en la página 182.".into()),
                rubric_after: Some("Esta Colecta, junto con el Salmo y las Lecciones correspondientes, también sirven para los siguientes días hasta el domingo próximo, a no ser que se señale de otro modo.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent1),
            CollectData {
                document: Document::from(
                    Text::from("Omnipotente Dios, cuyo bendito Hijo fue llevado por el Espíritu para ser tentado por Satanás: Apresúrate a socorrer a los que somos atacados por múltiples tentaciones; y así como tú conoces las flaquezas de cada uno de nosotros, haz que cada uno te halle poderoso para salvar; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Primer Domingo en Cuaresma")
                .page(132)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Cuaresma".into(),
                rubric_before: None,
                rubric_after: Some("El miércoles, viernes y sábado de esta semana son los tradicionales Días de Témporas de primavera.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent2),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, cuya gloria es siempre tener misericordia: Sé benigno a todos los que se han descarriado de tus caminos, y tráelos de nuevo con corazones penitentes y fe firme, para recibir y abrazar la verdad inmutable de tu Verbo, Jesucristo tu Hijo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Segundo Domingo en Cuaresma")
                .page(133)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Cuaresma".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent3),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, tú sabes que en nosotros no hay poder para ayudarnos: Guárdanos tanto exteriormente en cuerpo como interiormente en alma, para que seamos defendidos de todas las adversidades que puedan sobrevenir al cuerpo, y de los malos pensamientos que puedan asaltar y herir el alma; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Tercer Domingo en Cuaresma")
                .page(133)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Cuaresma".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent4),
            CollectData {
                document: Document::from(
                    Text::from("Padre bondadoso, cuyo bendito Hijo Jesucristo descendió del cielo para ser el pan verdadero que da vida al mundo: Danos siempre este pan, para que él viva en nosotros y nosotros en él; quien vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Cuarto Domingo en Cuaresma")
                .page(133)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Cuaresma".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent5),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, sólo tú puedes ordenar los afectos y voluntades rebeldes de los pecadores: Concede gracia a tu pueblo para amar lo que tú dispones y desear lo que tú prometes; a fin de que, en medio de los rápidos y variados cambios del mundo, nuestros corazones permanezcan fijos allí donde se encuentran los verdaderos goces; por nuestro Señor Jesucristo, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Quinto Domingo en Cuaresma")
                .page(134)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Cuaresma".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::HolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente y eterno, en tu tierno amor hacia el género humano, enviaste a tu Hijo nuestro Salvador Jesucristo para asumir nuestra naturaleza, y padecer muerte en la cruz, mostrándonos ejemplo de su gran humildad: Concédenos, en tu misericordia, que caminemos por el sendero de su padecimiento y participemos también en su resurrección; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Domingo de Pasión: Domingo de Ramos")
                .page(134)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: Some("La Liturgia Propia de este día se encuentra en la página 189.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MondayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, cuyo muy amado Hijo no ascendió al gozo de tu presencia sin antes padecer, ni entró en gloria sin antes ser crucificado: Concédenos, por tu misericordia, que nosotros, caminando por la vía de la cruz, encontremos que ésta es la vía de la vida y de la paz; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Lunes Santo")
                .page(135)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TuesdayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que por la pasión de tu bendito Hijo convertiste a un instrumento de muerte vergonzosa en un medio de vida para nosotros: Concede que de t ;l modo nos gloriemos en la cruz de Cristo que suframos con alegría la vergüenza y privación por causa de tu Hijo nuestro Salvador Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Martes Santo")
                .page(135)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::WednesdayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Señor Dios, cuyo bendito Hijo nuestro Salvador entregó su cuerpo a los azotes y su rostro al esputo: Otórganos tu gracia para soportar gozosamente los sufrimientos de esta vida temporal, confiados en la gloria que ha de ser revelada; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Miércoles Santo")
                .page(135)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MaundyThursday),
            CollectData {
                document: Document::from(
                    Text::from("Padre todopoderoso, cuyo amado Hijo, en la víspera de su padecimiento, instituyó el Sacramento de su Cuerpo y Sangre: Concédenos, en tu misericordia, que lo recibamos con gratitud como memorial de Jesucristo nuestro Señor, que en estos santos misterios nos da una prenda de la vida eterna; quien vive ahora y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Jueves Santo")
                .page(136)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: Some("La Liturgia Propia de este día se encuentra en la página 193.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::GoodFriday),
            CollectData {
                document: Document::from(
                    Text::from("Mira con bondad, te suplicamos, Dios omnipotente, a esta tu familia, por la cual nuestro Señor Jesucristo aceptó ser traicionado y entregado a hombres crueles, y sufrir muerte en la cruz; quien vive ahora y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Viernes Santo")
                .page(136)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: Some("La Liturgia Propia de este día se encuentra en la página 195.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolySaturday),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, Creador de cielo y tierra: Concede que, así como el cuerpo crucificado de tu amado Hijo fue puesto en el sepulcro y descansó en este Sábado santo, de la misma manera aguardemos con él la venida del tercer día, y resucitemos con él a la vida nueva; quien vive ahora y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Sábado Santo")
                .page(136)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: Some("La Liturgia Propia de este día se encuentra en la página 20;.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que por nuestra redención entregaste a tu unigénito Hijo a muerte de cruz, y por su resurrección gloriosa nos libraste del poder de nuestro enemigo: Concédenos morir diariamente al pecado, de tal manera que, en el gozo de su resurrección, vivamos siempre con Jesucristo tu Hijo nuestro Señor; que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Día de Pascua")
                .page(137)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: Some("La Liturgia de la Vigilia Pascual se encuentra en la página 205.".into()),
                rubric_after:  Some("o bien:".into())
            }
        ),
        (
            CollectId::Feast(Feast::EasterVigil),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que hiciste resplandecer esta noche santísima con la gloria de la resurrección del Señor: Aviva en tu Iglesia aquel Espíritu de adopción que nos es dado en el Bautismo, para que nosotros, siendo renovados tanto en cuerpo como en mente, te adoremos en sinceridad y verdad; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Día de Pascua")
                .page(137)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after:  Some("o bien:".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, que por medio de tu Hijo unigénito Jesucristo has vencido la muerte y nos abriste la puerta de la vida eterna: Concede a los que celebramos con gozo el día de la resurrección del Señor, que seamos resucitados de la muerte del pecado por tu Espíritu vivificador; mediante Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Día de Pascua")
                .page(137)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MondayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Concédenos, te suplicamos, Dios omnipotente, que quienes celebramos con reverencia la fiesta Pascual, seamos hallados dignos de alcanzar los goces eternos; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Lunes de Pascua")
                .page(138)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TuesdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que por la gloriosa resurrección de tu Hijo Jesucristo destruiste la muerte y nos alumbraste con vida e inmortalidad: Concede a los que hemos resucitado con él, que habitemos en su presencia, y nos gocemos en la esperanza de la gloria eterna; por Jesucristo nuestro Señor, a quien contigo y el Espíritu Santo, sea el dominio y la alabanza, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Martes de Pascua")
                .page(138)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::WednesdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, cuyo bendito Hijo se dio a conocer a sus discípulos en la fracción del pan: Abre los ojos de nuestra fe, para que podamos contemplarle en toda su obra redentora; quien vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Miércoles de Pascua")
                .page(138)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThursdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, que en el misterio Pascual has establecido el nuevo pacto de la reconciliación: Concede a todos los que nacen de nuevo en la comunión del Cuerpo de Cristo que manifiesten en sus vidas lo que por fe profesan; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Jueves de Pascua")
                .page(139)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::FridayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Padre todopoderoso, que entregaste a tu único Hijo para morar por nuestros pecados y resucitar para nuestra justificación: Danos gracia para desechar la levadura de malicia e iniquidad, de tal modo que te sirvamos siempre con pureza de vida y verdad; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Viernes de Pascua")
                .page(139)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::SaturdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Te damos gracias, Padre celestial, porque nos has librado del poder del pecado y de la muerte, y nos has traído al reino de tu Hijo; y te suplicamos que, así como por su muerte nos has devuelto a la vida, igualmente por su amor nos resucite a los goces eternos; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Sábado de Pascua")
                .page(139)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter2),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, que en el misterio Pascual has establecido el nuevo pacto de la reconciliación: Concede a todos los que nacen de nuevo en la comunión del Cuerpo de Cristo que manifiesten en sus vidas lo que por fe profesan; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Segundo Domingo de Pascua")
                .page(140)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter3),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, cuyo bendito Hijo se dio a conocer a sus discípulos en la fracción del pan: Abre los ojos de nuestra fe, para que podamos contemplarle en toda su obra redentora; quien vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Tercer Domingo de Pascua")
                .page(140)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter4),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, cuyo Hijo Jesús es el buen pastor de tu pueblo: Concede que, al escuchar su voz, reconozcamos a aquél que llama a cada uno de nosotros por su nombre, y le sigamos a donde nos guíe; quien contigo y el Espíritu Santo vive y reina, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Cuarto Domingo de Pascua")
                .page(140)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter5),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, conocerte verdaderamente es vida eterna: Concede que conozcamos tan perfectamente que tu Hijo Jesucristo es el camino, la verdad y la vida, que sigamos sus pasos con perseverancia en el camino que conduce a la vida eterna; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Quinto Domingo de Pascua")
                .page(141)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter6),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, tú has preparado para los que te aman cosas tan buenas que sobrepasan nuestro entendimiento: Infunde en nuestros corazones tal amor hacia ti, que, amándote en todo y sobre todas las cosas, obtengamos tus promesas, que exceden todo lo que podamos anhelar; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Sexto Domingo de Pascua")
                .page(141)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pascua".into(),
                rubric_before: None,
                rubric_after: Some("El lunes, martes y miércoles de esta semana son los tradicionales Días de Rogativa.".into())
            }
        ),
        (
            CollectId::Feast(Feast::AscensionDay),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, cuyo bendito Hijo nuestro Señor Jesucristo ascendió por encima de todos los cielos para llenarlo todo: Danos fe, por tu misericordia, para percibir que, según su promesa, habita con su Iglesia en la tierra, hasta el final de los tiempos; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, en gloria eterna.")
                        .response("Amén.")
                )
                .label("Día de la Ascensión")
                .page(141)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o la siguiente:".into())
            }
        ),
        (
            CollectId::Feast(Feast::AscensionDay),
            CollectData {
                document: Document::from(
                    Text::from("Concédenos, te suplicamos, Dios omnipotente, que, así como creemos que tu unigénito Hijo nuestro Señor Jesucristo ascendió a los cielos, así también ascendamos allá en corazón y mente, y habitemos siempre con él; quien vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Día de la Ascensión")
                .page(142)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Ascensión".into(),
                rubric_before: None,
                rubric_after:  Some("Cualquiera de las dos Colectas anteriores, junto con el Salmo y las Lecciones para el Día de la Ascensión, pueden usarse los siguientes días de esta semana, a no ser que se señale de otro modo.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter7),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, Rey de la gloria, que con gran triunfo exaltaste a tu único Hijo Jesucristo a tu reino celestial: No nos dejes desconsolados, mas envíanos tu Espíritu Santo para fortalecernos y exaltarnos al mismo lugar, adonde nuestro Salvador Cristo nos ha precedido; quien vive y reina contigo y el Espíritu Santo, un solo Dios, en gloria eterna.")
                        .response("Amén.")
                )
                .label("Séptimo Domingo de Pascua: Domingo después de la Ascensión")
                .page(142)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Ascensión".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Pentecost),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, en este día abriste el camino de la vida eterna a toda raza y nación por el don prometido de tu Espíritu Santo: Esparce este don sobre todo el mundo por la predicación del Evangelio, para que llegue a los confines de la tierra; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Día de Pentecostés")
                .page(143)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pentecostés".into(),
                rubric_before: Some("Cuando se celebra la Vigilia de Pentecostés, ésta comienza con el Rito de la Luz, página 73 (usando, si se desea, el “Gloria in excelsis” en lugar del “Phos hilaron”, página 76) y continúa con la Salutación y la colecta del Día. Antes del Evangelio se leen tres o más de las Lecciones señaladas, cada una de ellas seguida por un Salmo, Cántico o himno. Después del Sermón, sigue el Santo Bautismo, la Confirmación (comenzando con la Presentación de los Candidatos) o la Renovación de Votos Bautismales, página 213.".into()),
                rubric_after: Some("o bien:".into())
            }
        ),
        (
            CollectId::Feast(Feast::Pentecost),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que en este día enseñaste a los corazones de tus fieles, enviándoles la luz de tu Espíritu Santo: Concédenos por el mismo Espíritu, que tengamos un juicio acertado en todas las cosas, y que nos regocijemos siempre en su santa fortaleza; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Día de Pentecostés")
                .page(143)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pentecostés".into(),
                rubric_before: None,
                rubric_after: Some("De lunes a sábado de esta semana se usa el Propio numerado que corresponda a la fecha más cercana del Día de Pentecostés en este año. Véase la página 124.\n\nEl miércoles, viernes y sábado de esta semana son los tradicionales Días de Témporas de verano.".into())
            }
        ),
        (
            CollectId::Feast(Feast::TrinitySunday),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente y eterno, que por la confesión de una fe verdadera nos diste a tus siervos la gracia de reconocer la gloria de la Trinidad eterna, y de adorar la Unidad en el poder de tu divina Majestad: Consérvanos firmes en esta fe y adoración, y llévanos al fin a contemplarte en tu sola y eterna gloria; tú que vives y reinas, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Primer Domingo después de Pentecostés: Domingo de Trinidad")
                .page(143)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Domingo de la Trinidad".into(),
                rubric_before: None,
                rubric_after: Some("De lunes a sábado de esta semana se usa el Propio numerado que corresponda a la fecha más cercana del Domingo de Trinidad en este año.".into())
            }
        ),
        (
            CollectId::Proper(Proper::Proper1),
            CollectData {
                document: Document::from(
                    Text::from("Recuerda, oh Señor, lo que has forjado en nosotros y no lo que merecemos; y, puesto que nos has llamado para servirte, haznos dignos de nuestro llamado; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 1")
                    .subtitle("Semana del domingo más cercano a Mayo 11")
                .page(144)
                .tags(["Estaciones del Año", "La Estación después de Pentecostés"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "No se usa Prefacio Propio.".into(),
                rubric_before: Some("Las reglas para el uso de los Propios que siguen se encuentran en la página 124.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper2),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente y de misericordia, guárdanos en tu bondad de todo aquello que pueda causarnos daño; para que, dispuestos tanto en mente como en cuerpo, y con alegría de corazón, logremos lo que sea propio a tus designios; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 2")
                    .subtitle("Semana del domingo más cercano a Mayo 18")
                .page(144)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "No se usa Prefacio Propio.".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper3),
            CollectData {
                document: Document::from(
                    Text::from("Concede, oh Señor, que el curso de este mundo sea gobernado pacíficamente por tu providencia, y que tuIglesia pueda servirte con gozo, confianza y serenidad;por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 3")
                    .subtitle("El domingo más cercano a Mayo 25")
                .page(145)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper4),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, tú infalible providencia ordena todas las cosas en el cielo como en la tierra: Aparta de nosotros todo mal, te suplicamos, y concédenos aquellos beneficios que puedan ayudarnos; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 4")
                    .subtitle("El domingo más cercano a Junio 1")
                .page(145)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper5),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, de quien procede todo lo bueno: Concede, por tu inspiración, que pensemos lo justo y, guiados por ti, podamos hacerlo; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos. Amen.")
                        .response("Amén.")
                )
                .label("Propio 5")
                    .subtitle("El domingo más cercano a Junio 8")
                .page(145)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper6),
            CollectData {
                document: Document::from(
                    Text::from("Mantén, oh Señor, a tu familia, la Iglesia, en tu constante fe y amor; para que, mediante tu gracia, proclamemos tu verdad con valentía, y administremos tu justicia con compasión; por amor de nuestro Salvador Jesucristo, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 6")
                    .subtitle("El domingo más cercano a Junio 15")
                .page(145)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper7),
            CollectData {
                document: Document::from(
                    Text::from("Oh Señor, haz que tengamos perpetuo amor y reverencia a tu santo Nombre, pues nunca privas de tu auxilio y guía a los que has establecido sobre la base firme de tu bondad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 7")
                    .subtitle("El domingo más cercano a Junio 22")
                .page(146)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper8),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, has edificado tu Iglesia sobre el fundamento de los apóstoles y profetas siendo Jesucristo mismo la piedra angular: Concédenos que estemos unidos en espíritu por su enseñanza, de tal modo que lleguemos a ser un templo santo aceptable a ti; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 8")
                    .subtitle("El domingo más cercano a Junio 29")
                .page(146)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper9),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, tú nos has enseñado a guardar tus mandamientos amándote a ti y a nuestro prójimo: Danos la gracia de tu Espíritu Santo para que nos consagremos a ti de todo corazón, y nos unamos unos a otros con afecto puro; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 9")
                    .subtitle("El domingo más cercano a Julio 6")
                .page(146)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper10),
            CollectData {
                document: Document::from(
                    Text::from("Oh Señor, atiende, en tu bondad, las súplicas de tu pueblo que clama a ti, y concede que podamos percibir y comprender lo que debemos hacer, y tengamos también la gracia y el poder para cumplirlo fielmente; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 10")
                    .subtitle("El domingo más cercano a Julio 13")
                .page(147)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper11),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, fuente de toda sabiduría, tú conoces nuestras necesidades antes de que te pidamos, y nuestra ignorancia en pedir: Ten compasión de nuestras flaquezas, y danos, por tu misericordia, aquellas cosas que por nuestra indignidad y ceguedad no sabemos ni nos atrevemos a pedirte; por los méritos de Jesucristo tu Hijo nuestro Señor; que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 11")
                    .subtitle("El domingo más cercano a Julio 20")
                .page(147)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper12),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, protector de cuantos en ti confían, sin quien nada es fuerte, nada es santo: Multiplica en nosotros tu misericordia, a fin de que, bajo tu dirección y guía, nos sirvamos de los bienes temporales, de tal manera que no perdamos los eternos; por Jesucristo nuestro Señor que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 12")
                    .subtitle("El domingo más cercano a Julio 27")
                .page(147)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper13),
            CollectData {
                document: Document::from(
                    Text::from("Que tu constante misericordia purifique y defienda a tu Iglesia, oh Señor; y, puesto que no puede continuar en seguridad sin tu auxilio, protégela y dirígela siempre por tu bondad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 13")
                    .subtitle("El domingo más cercano a Agosto 3")
                .page(148)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper14),
            CollectData {
                document: Document::from(
                    Text::from("Otórganos, te suplicamos, oh Señor, el espíritu de pensar y hacer siempre lo justo; para que nosotros, que sin ti no podemos existir, seamos capaces, con tu ayuda, de vivir según tu voluntad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 14")
                    .subtitle("El domingo más cercano a Agosto 10")
                .page(148)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper15),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, por nosotros entregaste a tu Hijo único como sacrificio por los pecados y como ejemplo de vida piadosa: Danos gracia para recibir con gratitud los frutos de su obra redentora, y seguir de día en día las huellas benditas de su santísima vida; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 15")
                    .subtitle("El domingo más cercano a Agosto 17")
                .page(148)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper16),
            CollectData {
                document: Document::from(
                    Text::from("Concede, oh Dios de misericordia, que tu Iglesia,congregada en unidad por tu Espíritu Santo, manifieste tu poder entre todos los pueblos, para gloria de tu Nombre; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 16")
                    .subtitle("El domingo más cercano a Agosto 24")
                .page(149)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper17),
            CollectData {
                document: Document::from(
                    Text::from("Señor de todo poder y fortaleza, autor y dador de todo bien: Injerta en nuestros corazones el amor a tu Nombre, acrecienta en nosotros la verdadera religión, nútrenos con toda bondad, y produce en nosotros los frutos de buenas obras; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 17")
                    .subtitle("El domingo más cercano a Agosto 31")
                .page(149)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper18),
            CollectData {
                document: Document::from(
                    Text::from("Concede, oh Señor, que confiemos en ti de todo corazón; porque, así como tú siempre resistes a los soberbios que confían en su propia fortaleza, de la misma manera jamás abandonas a aquéllos que se glorían en tu misericordia; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 18")
                    .subtitle("El domingo más cercano a Septiembre 7")
                .page(149)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper19),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, puesto que sin ti no podemos complacerte: Concede, por tu misericordia, que tu Espíritu Santo dirija y gobierne nuestros corazones; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 19")
                    .subtitle("El domingo más cercano a Septiembre 14")
                .page(149)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: Some("El miércoles, viernes y sábado después del 14 de septiembre son los tradicionales Días de Témporas de otoño.".into())
            }
        ),
        (
            CollectId::Proper(Proper::Proper20),
            CollectData {
                document: Document::from(
                    Text::from("Concede, oh Señor, que no nos afanemos por las cosas terrenales, sino que amemos las celestiales, y aun ahora que estamos inmersos en cosas transitorias, haz que anhelemos lo que permanece para siempre; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 20")
                    .subtitle("El domingo más cercano a Septiembre 21")
                .page(150)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper21),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que manifiestas tu infinito poder especialmente mostrando piedad y misericordia: Derrama sobre nosotros la plenitud de tu gracia; a fin de que, esforzándonos para obtener tus promesas, seamos partícipes de tus tesoros celestiales; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 21")
                    .subtitle("El domingo más cercano a Septiembre 28")
                .page(150)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper22),
            CollectData {
                document: Document::from(
                    Text::from("Omnipotente y sempiterno Dios, tú estás siempre más presto a escuchar que nosotros a orar, y a ofrecer más de lo que deseamos o merecemos: Derrama sobre nosotros la abundancia de tu misericordia, perdonándonos todo aquello que perturba nuestra conciencia, y otorgándonos aquello que no somos dignos de pedirte, sino por los méritos y mediación de Jesucristo nuestro Salvador; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 22")
                    .subtitle("El domingo más cercano a Octubre 5")
                .page(151)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper23),
            CollectData {
                document: Document::from(
                    Text::from("Te rogamos, oh Señor, que tu gracia siempre nos preceda y acompañe, para que continuamente nos dediquemos a buenas obras; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 23")
                    .subtitle("El domingo más cercano a Octubre 12")
                .page(151)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper24),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, que en Cristo has revelado tu gloria a todas las naciones: Mantén las obras de tu misericordia; a fin de que tu Iglesia, esparcida por todo el mundo, persevere con fe inquebrantable en la confesión de tu Nombre; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 24")
                    .subtitle("El domingo más cercano a Octubre 19")
                .page(151)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper25),
            CollectData {
                document: Document::from(
                    Text::from("Todopoderoso y eterno Dios, aumenta en nosotros tus dones de fe, esperanza y amor; y para que obtengamos tus promesas, haz que amemos lo que mandas; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos. Amen.")
                        .response("Amén.")
                )
                .label("Propio 25")
                    .subtitle("El domingo más cercano a Octubre 26")
                .page(151)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper26),
            CollectData {
                document: Document::from(
                    Text::from("Dios de poder y piedad, sólo de ti mana el don que hace posible que tu pueblo fiel te sirva sincera y laudablemente: Concédenos que, para lograr el premio de tus promesas celestiales, podamos correr sin tropiezos; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 26")
                    .subtitle("El domingo más cercano a Noviembre 2")
                .page(152)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper27),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, cuyo bendito Hijo vino al mundo para destruir las obras de Satanás y hacernos hijos de Dios y herederos de la vida eterna: Concede que, teniendo esta esperanza, nos purifiquemos así como él es puro; para que, cuando vuelva con poder y gran gloria, seamos hechos a su semejanza en su glorioso y eterno reino; donde contigo y el Espíritu Santo, vive y reina, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 27")
                    .subtitle("El domingo más cercano a Noviembre 9")
                .page(152)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper28),
            CollectData {
                document: Document::from(
                    Text::from("Bendito Señor, tú que inspiraste las Sagradas Escrituras para nuestra enseñanza: Concede que de tal manera las oigamos, las leamos, las consideremos, las aprendamos e interiormente las asimilemos, que podamos abrazar y siempre mantener la esperanza bendita de la vida eterna, que nos has dado en nuestro Salvador Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Propio 28")
                    .subtitle("El domingo más cercano a Noviembre 16")
                .page(152)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper29),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente y eterno, cuya voluntad es restaurar todas las cosas en tu muy amado Hijo, el Rey de reyes y Señor de señores: Concede, de tu piedad, que todos los pueblos de la tierra, divididos y esclavizados por el pecado, sean libertados y unificados bajo su reino de amor; quien vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Propio 29")
                    .subtitle("El domingo más cercano a Noviembre 23")
                .page(153)
                .tags(["Estaciones del Año"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Día del Señor, o del Bautismo".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Andrew),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que diste a tu apóstol Andrés una gracia tal que prestamente obedeció el llamado de tu Hijo Jesucristo y trajo a su hermano con él: Concédenos, a los llamados por tu Santo Verbo, la gracia para seguirle sin demora y traer a su bondadosa presencia a los que amamos; por aquél que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("San Andrés")
                    .subtitle("Noviembre 30")
                .page(153)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Thomas),
            CollectData {
                document: Document::from(
                    Text::from("Eterno Dios, que fortaleciste a tu apóstol Tomás con una fe cierta y firme en la resurrección de tu Hijo: Concede que creamos en Jesucristo, nuestro Señor y nuestro Dios, tan perfectamente y sin duda, que nuestra fe no se halle deficiente a tus ojos; por aquél que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Santo Tomás")
                    .subtitle("Diciembre 21")
                .page(153)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Stephen),
            CollectData {
                document: Document::from(
                    Text::from("Te damos gracias, oh Señor de la gloria, por el ejemplo del protomártir Esteban, quien, mirando hacia el cielo, intercedió por sus perseguidores ante tu Hijo Jesucristo, que está a tu diestra; donde vive y reina contigo y el Espíritu Santo, un solo Dios, en gloria eterna.")
                        .response("Amén.")
                )
                .label("San Esteban")
                    .subtitle("Diciembre 26")
                .page(154)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::John),
            CollectData {
                document: Document::from(
                    Text::from("Derrama, oh Señor, sobre tu Iglesia el resplandor de tu luz, para que, iluminados por la enseñanza de tu apóstol y evangelista Juan, andemos en la luz de tu verdad de tal manera que al fin alcancemos la plenitud de la vida eterna; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("San Juan")
                    .subtitle("Diciembre 27")
                .page(154)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyInnocents),
            CollectData {
                document: Document::from(
                    Text::from("Recordamos en este día, oh Dios, la matanza de los niños inocentes de Belén, ordenada por el Rey Herodes. Recibe, te suplicamos, en tus brazos de misericordia, a todas las víctimas inocentes; y por tu gran poder frustra los designios de tiranos sanguinarios, y establece tu dominio de justicia, amor y paz; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Los Santos Inocentes")
                    .subtitle("Diciembre 28")
                .page(154)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ConfessionOfStPeter),
            CollectData {
                document: Document::from(
                    Text::from("Padre todopoderoso, que inspiraste a Simón Pedro, el primero entre los apóstoles, a confesar a Jesús como Mesías e Hijo del Dios vivo: Mantén a tu Iglesia firme sobre la roca de esta fe, para que, en paz y unidad, proclamemos la única verdad y sigamos al único Señor, nuestro Salvador Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Confesión de San Pedro")
                    .subtitle("Enero 18")
                .page(155)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ConversionOfStPaul),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que por la predicación de tu apóstol Pablo hiciste que la luz del Evangelio resplandeciera por todo el mundo: Concede, te suplicamos, que nosotros, recordando su portentosa conversión, manifestemos nuestra gratitud siguiendo su santa enseñanza; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Conversión de San Pablo")
                    .subtitle("Enero 25")
                .page(155)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThePresentation),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, humildemente te rogamos que, así como tu Hijo unigénito fue presentado en el templo en este día, así seamos presentados ante ti con corazones puros y limpios; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("La Presentación")
                    .subtitle("Febrero 2")
                .page(155)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Epifanía".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Matthias),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, que en lugar de Judas escogiste a tu fiel siervo Matías para ser contado entre el número de los Doce: Concede que tu Iglesia, librada de falsos apóstoles, sea guiada y gobernada siempre por pastores fieles y verdaderos; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("San Matías")
                    .subtitle("Febrero 24")
                .page(156)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Joseph),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que de la familia de tu siervo David levantaste a José para ser el guardián de tu Hijo encarnado, y esposo de su virgen madre: Danos gracia para imitar su rectitud de vida y su obediencia a tus mandatos; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("San José")
                    .subtitle("Marzo 19")
                .page(156)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Epifanía".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Annunciation),
            CollectData {
                document: Document::from(
                    Text::from("Derrama tu gracia en nuestros corazones, oh Señor, para que los que hemos conocido la encarnación de tu Hijo Jesucristo, anunciada por un ángel a María la Virgen, seamos llevados por la cruz y pasión de Cristo a la gloria de su resurrección; quien vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("La Anunciación")
                    .subtitle("Marzo 25")
                .page(156)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Epifanía".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Mark),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, que por la mano del evangelista Marcos has entregado a tu Iglesia el Evangelio de Jesucristo el Hijo de Dios: Te damos gracias por este testimonio, y te rogamos nos mantengas firmes en su verdad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("San Marcos")
                    .subtitle("Abril 25")
                .page(157)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Todos los Santos".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::PhilipAndJames),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que diste gracia y fortaleza a tus apóstoles Felipe y Santiago para dar testimonio de la verdad: Concede que, recordando su victoria de fe, glorifiquemos, tanto en la vida como en la muerte, el Nombre de nuestro Señor Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("San Felipe y Santiago")
                    .subtitle("Mayo 1")
                .page(157)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TheVisitation),
            CollectData {
                document: Document::from(
                    Text::from("Padre celestial, por tu gracia la virgen madre de tu Hijo encarnado fue bendita al llevarlo en su seno, y aún más bendita al guardar tu palabra: Concede a los que honramos la exaltación de su humildad que sigamos el ejemplo de su devoción a tu voluntad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("La Visitación")
                    .subtitle("Mayo 31")
                .page(157)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Epifanía".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Barnabas),
            CollectData {
                document: Document::from(
                    Text::from("Concédenos, oh Dios, que sigamos el ejemplo de tu fiel siervo Bernabé, que no buscaba su propio provecho sino el bienestar de tu Iglesia, y ofrecía generosamente sus bienes y su vida para el socorro de los pobres y la propagación del Evangelio; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("San Bernabé")
                    .subtitle("Junio 11")
                .page(158)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::NativityOfStJohnTheBaptist),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, por cuya providencia nació maravillosamente tu siervo Juan el Bautista, y fue enviado a preparar el camino de tu Hijo nuestro Salvador, predicando el arrepentimiento: Haz que sigamos de tal manera su enseñanza y santa vida que verdaderamente nos arrepintamos según su predicación, y que, a ejemplo suyo, constantemente hablemos la verdad, audazmente reprochemos el vicio y pacientemente suframos por causa de la verdad; por Jesucristo tu Hijo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Natividad de San Juan Bautista")
                    .subtitle("Junio 24")
                .page(158)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Adviento".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::PeterAndPaul),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, cuyos benditos apóstoles Pedro y Pablo te glorificaron con su martirio: Concede que tu Iglesia, instruida por su enseñanza y ejemplo, y entrelazada en unidad por tu Espíritu, permanezca siempre firme sobre el único cimiento, que es Jesucristo nuestro Señor; que vive y reina contigo, en la unidad del Espíritu Santo, un soloDios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("San Pedro y San Pablo")
                    .subtitle("Junio 29")
                .page(158)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::IndependenceDay),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, cuyo bendito Hijo restauró a María Magdalena a la salud de cuerpo y mente, y la llamó a ser testigo de su resurrección: Concede, en tu misericordia, que por tu gracia seamos sanados de todas nuestras enfermedades y te conozcamos en el poder de la vida perdurable de Cristo; que contigo y el Espíritu Santo vive y reina, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Santa María Magdalena")
                    .subtitle("Julio 22")
                .page(159)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Todos los Santos".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MaryMagdalene),
            CollectData {
                document: Document::from(
                    Text::from("Dios bondadoso, recordamos hoy en tu presencia a tu siervo y apóstol Santiago, el primero entre los Doce en padecer martirio por el Nombre de Jesucristo; y te suplicamos que derrames sobre los dirigentes de tu Iglesia ese espíritu de servicio abnegado por el cual sólo pueden tener verdadera autoridad entre tu pueblo; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Santiago")
                    .subtitle("Julio 25")
                .page(159)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::James),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que en el santo monte revelaste ante testigos escogidos a tu muy amado Hijo, maravillosamente transfigurado, con vestiduras blancas y resplandecientes: Concede, en tu misericordia, que, librados de la inquietud de este mundo, contemplemos por fe al Rey en toda su hermosura; quien contigo, oh Padre, y contigo, oh Espíritu Santo, vive y reina, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("La Transfiguración")
                    .subtitle("Agosto 6")
                .page(159)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Epifanía".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TheTransfiguration),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que tomaste para ti a la bienaventurada Virgen María, madre de tu Hijo encarnado: Concede que, redimidos por la sangre de Cristo, compartamos con ella la gloria de tu reino eterno; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Santa María Virgen")
                    .subtitle("Agosto 15")
                .page(160)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Encarnación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Mary),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, que diste gracia a tu apóstol Bartolomé para creer verdaderamente y predicar tu Palabra: Concede que tu Iglesia ame lo que él creyó y predique lo que él enseñó; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("San Bartolomé")
                    .subtitle("Agosto 24")
                .page(160)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Bartholomew),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, cuyo Hijo nuestro Salvador fue levantado en lo alto de la cruz, a fin de atraer hacia él a todo el mundo: Concede, en tu misericordia, a quienes nos gloriamos en el misterio de nuestra redención, que recibamos tu gracia para tomar nuestra cruz y seguirle; quien vive y reina contigo y el Espíritu Santo, un solo Dios, en gloria sempiterna.")
                        .response("Amén.")
                )
                .label("Día de la Santa Cruz")
                    .subtitle("Septiembre 14")
                .page(160)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyCross),
            CollectData {
                document: Document::from(
                    Text::from("Te damos gracias, Padre celestial, por el testimonio que tu apóstol y evangelista Mateo dio a las Buenas Nuevas de tu Hijo nuestro Salvador; y rogamos que, siguiendo su ejemplo, obedezcamos con voluntades y corazones dispuestos el llamado de nuestro Señor a seguirle; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("San Mateo")
                    .subtitle("Septiembre 21")
                .page(161)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Matthew),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios eterno, que has establecido y constituido en orden maravilloso los ministerios de los ángeles y los mortales: Concede, en tu misericordia, que así como tus santos ángeles continuamente te sirven y adoran en el cielo, asimismo, por tu mandato, nos socorran y defiendan en la tierra; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("San Miguel y Todos Los Angeles")
                    .subtitle("Septiembre 29")
                .page(161)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Michael),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, que inspiraste a tu siervo Lucas el médico a manifestar en el Evangelio el amor y poder sanativo de tu Hijo: Continúa en tu Iglesia, por tu gracia, el mismo amor y poder de sanidad, para alabanza y gloria de tu Nombre; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("San Lucas")
                    .subtitle("Octubre 18")
                .page(161)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Todos los Santos".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Luke),
            CollectData {
                document: Document::from(
                    Text::from("Concede, oh Dios, que tu Iglesia, siguiendo el ejemplo de tu siervo Santiago el Justo, hermano de nuestro Señor, se dedique continuamente a la oración y la reconciliación de todos los que están en desacuerdo y enemistad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Santiago de Jerusalén")
                    .subtitle("Octubre 23")
                .page(162)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Todos los Santos".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::JamesOfJerusalem),
            CollectData {
                document: Document::from(
                    Text::from("Te damos gracias, oh Señor, por la gloriosa compañía de los apóstoles, y especialmente en este día por Simón y Judas; y te rogamos que, así como ellos fueron fieles y celosos en su misión, asimismo, con ardiente devoción, demos a conocer el amor y la misericordia de nuestro Señor y Salvador Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("San Simón y San Judas")
                    .subtitle("Octubre 28")
                .page(162)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::SimonAndJude),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso tú has entrelazado a tus elegidos en una sola comunión y hermandad en el cuerpo místico de tu Hijo Cristo nuestro Señor: Danos gracia para que de tal modo sigamos a tus benditos santos en toda virtuosa y santa vida que alcancemos los gozos inefables que tú has preparado para los que te aman sinceramente; por Jesucristo nuestro Señor, que contigo y el Espíritu Santo vive y reina, un solo Dios, en gloria sempiterna.")
                        .response("Amén.")
                )
                .label("Día de Todos Los Santos")
                    .subtitle("Noviembre 1")
                .page(162)
                .tags(["Días Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Todos los Santos".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::AllSaintsDay),
            CollectData {
                document: Document::from(
                    Text::from("Señor Dios omnipotente, en cuyo Nombre los fundadores de este país ganaron su libertad y la nuestra [y encendieron la antorcha de la libertad para naciones que todavía no existían]: Concede que nosotros y todos los habitantes de esta tierra recibamos tu gracia para mantener nuestras libertades en justicia y paz; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("Día de la Independencia")
                .page(163)
                .tags(["Otras Conmemoraciones"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: Some("Puede usarse la Colecta “Por la Nación,” en la página 176, en vez de la anterior".into())
            }
        ),
        (
            CollectId::Feast(Feast::ThanksgivingDay),
            CollectData {
                document: Document::from(
                    Text::from("Padre omnipotente y bondadoso, te damos gracias por los frutos de la tierra en su tiempo, y por la labor de quienes los cosechan: Haznos fieles mayordomos de tus dádivas abundantes, que recibimos para la satisfacción de nuestras necesidades y el alivio de los menesterosos, para gloria de tu Nombre; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("Día de Acción de Gracias")
                .page(163)
                .tags(["Otras Conmemoraciones"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: Some("Para la Oración de los Fieles puede usarse la Letanía de Acción de Gracias, en la página 728.".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que diste firmeza a tu siervo N. para confesar ante los gobernantes de este mundo el Nombre de nuestro Salvador Jesucristo, y valor para morir por esta fe: Concede que siempre estemos prestos a dar razón de la esperanza que hay en nosotros, y dispuestos a sufrir por causa de nuestro Señor Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Mártir")
                .page(164)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o bien:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, por cuya gracia y poder tu santo mártir N. triunfó sobre el sufrimiento, y fue fiel hasta la muerte: Concede ahora a cuantos le recordamos con acción de gracias que seamos fieles en nuestro testimonio de ti en este mundo, de tal modo que recibamos con él la corona de vida; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Mártir")
                .page(164)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o la siguiente:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, que encendiste la llama de tu amor en el corazón de tu santo mártir N.: Concédenos la misma fe y poder del amor a tus humildes siervos, que, así como nos regocijamos en su triunfo, también podamos beneficiarnos de su ejemplo; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Mártir")
                .page(165)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de un Santo".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Missionary),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso y eterno, te damos gracias por tu siervo N., a quien llamaste para predicar el Evangelio al pueblo de ___________________________ (o al pueblo _____________________ ): Levanta en éste y en todos los países evangelistas y heraldos de tu reino; para que tu Iglesia proclame las insondables riquezas de nuestro Salvador Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("De un Misionero")
                .page(165)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o bien:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Missionary),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, cuya voluntad es ser glorificado en tus santos, y que levantaste a tu siervo N. para ser una luz en el mundo: Resplandece, te rogamos, en nuestros corazones, para que también en nuestra generación manifestemos tu alabanza, tú que nos has llamado de las tinieblas a tu luz maravillosa; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("De un Misionero")
                .page(165)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pentecostés".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Pastor),
            CollectData {
                document: Document::from(
                    Text::from("Padre celestial, Pastor de tu pueblo, te damos gracias por tu siervo N., quien fielmente estuvo al cuidado y formación de tu grey; y te suplicamos que, siguiendo el ejemplo y la enseñanza de su santa vida, crezcamos por tu gracia a la estatura de la plenitud de nuestro Señor y Salvador Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Pastor")
                .page(166)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o bien:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Pastor),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, nuestro Padre celestial, que levantaste a tu siervo fiel, N., para ser [obispo y] pastor en tu Iglesia, y para alimentar a tu rebaño: Otorga en abundancia los dones de tu Espíritu Santo a todos los pastores, a fin de que ministren a tu familia como verdaderos siervos de Cristo y mayordomos de tus divinos misterios; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Pastor")
                .page(166)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de un Santo".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Theologian),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, por tu Espíritu Santo concedes a algunos palabra de sabiduría, a otros palabra de ciencia, y a otros palabra de fe: Alabamos tu Nombre por los dones de gracia manifestados en tu siervo N., y rogamos que tu Iglesia nunca sea privada de dichos dones; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Teólogo y Maestro")
                .page(166)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o la siguiente:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Theologian),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que diste a tu siervo N. dones especiales de gracia para entender y enseñar la verdad revelada en Cristo Jesús: Concede, por medio de esta enseñanza, que te conozcamos a ti, el único Dios verdadero y al que tú has enviado, Jesucristo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Teólogo y Maestro")
                .page(167)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de un Santo, o del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Monastic),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, cuyo bendito Hijo se hizo pobre, para que por su pobreza seamos enriquecidos: Líbranos del amor inapropiado a este mundo, para que, inspirados por la devoción de tu siervo N., te sirvamos con sencillez de corazón, y alcancemos las riquezas de los siglos venideros; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("De un Religioso")
                .page(167)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after:  Some("o bien:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Monastic),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, por cuya gracia tu siervo N., encendido con la llama de tu amor, llegó a ser una luz ardiente y brillante en tu Iglesia: Concede que también nosotros seamos encendidos con el espíritu de amor y disciplina, y andemos siempre ante ti como hijos de la luz; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("De un Religioso")
                .page(167)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de un Santo".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que nos has rodeado de una nube grande de testigos: Concede que, fortalecidos por el buen ejemplo de tu siervo N., perseveremos en la carrera que nos queda por delante, hasta que al fin, con él, alcancemos tu gozo eterno; por Jesucristo, el autor y consumador de nuestra fe, quien vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Santo")
                .page(168)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after:  Some("o bien:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que nos has acercado a la compañía innumerable de los ángeles y de los espíritus de los justos hechos perfectos: Concede que moremos en su comunión durante nuestra peregrinación terrenal, y en nuestra patria celestial lleguemos a ser partícipes de su gozo; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("De un Santo")
                .page(168)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after:  Some("o bien:".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que por tu Santo Espíritu nos has hecho uno con tus santos en el cielo y en la tierra: Concede que en nuestro peregrinaje terrenal seamos continuamente sostenidos por esta comunión de amor y oración, sabiéndonos rodeados por su testimonio de tu poder y misericordia. Te lo pedimos por amor de Jesucristo, en quien todas nuestras intercesiones son aceptables por medio del Espíritu, y que vive y reina por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("De un Santo")
                .page(168)
                .tags(["Común de los Santos"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de un Santo".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyTrinity),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, que has revelado a tu Iglesia tu ser eterno de gloriosa majestad y amor perfecto como un solo Dios en Trinidad de Personas: Danos gracia para continuar firmes en la confesión de esta fe, y constantes en nuestra adoración a ti, Padre, Hijo y Espíritu Santo; tú que vives y reinas, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("1. De la Santísima Trinidad")
                .page(169)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolySpirit),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente y de toda misericordia, concede que, morando en nosotros tu Espíritu Santo, seamos iluminados y fortalecidos para servirte; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("2. Del Espíritu Santo")
                .page(169)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pentecostés".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyAngels),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios eterno, que has establecido y constituido en orden maravilloso los ministerios de los ángeles y los mortales: Concede, en tu misericordia, que así como tus santos ángeles continuamente te sirven y adoran en el cielo, asimismo, por tu mandato, nos socorran y defiendan en la tierra; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("3. De los Santos Angeles")
                .page(169)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Incarnation),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que maravillosamente creaste y aún más maravillosamente restauraste la dignidad de la naturaleza humana: Concede que compartamos la vida divina de quien se humilló para compartir nuestra humanidad, tu Hijo Jesucristo; que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("4. De la Encarnación")
                .page(170)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Epifanía".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyEucharist),
            CollectData {
                document: Document::from(
                    Text::from("Dios y Padre nuestro, cuyo Hijo nuestro Señor Jesucristo nos dejó en un Sacramento maravilloso el memorial de su pasión: Concede que de tal modo veneremos los sagrados misterios de su Cuerpo y Sangre, que podamos discernir constantemente en nosotros el fruto de su redención; quien vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("5. De la Santa Eucaristía")
                .page(170)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Epifanía".into(),
                rubric_before: Some("Especialmente adecuada para los jueves".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyCross),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, cuyo amado Hijo sufrió voluntariamente la agonía e ignominia de la cruz por nuestra redención: Danos valor para tomar nuestra cruz y seguirle; quien vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("6. De la Santa Cruz")
                .page(170)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Semana Santa".into(),
                rubric_before: Some("Especialmente adecuada para los viernes".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::AllBaptizedChristians),
            CollectData {
                document: Document::from(
                    Text::from("Concede, oh Señor Dios, a los que hemos sido bautizados en la muerte y resurrección de tu Hijo Jesucristo, que, así como hemos desechado la vieja vida de pecado, seamos también renovados en el espíritu de nuestras mentes, y vivamos en justicia y verdadera santidad; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("7. Por todos los Bautizados")
                .page(171)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Bautismo".into(),
                rubric_before: Some("Especialmente adecuada para los sábados".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::TheDeparted),
            CollectData {
                document: Document::from(
                    Text::from("Oh Señor Dios eterno, que mantienes en vida a todas las almas: Concede a toda tu Iglesia en el paraíso y en la tierra tu luz y tu paz; y permite que, siguiendo los buenos ejemplos de los que te han servido aquí y ahora descansan, podamos al fin entrar con ellos a tu gozo eterno; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("8. Por los Difuntos")
                .page(171)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after:  Some("o bien:".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::TheDeparted),
            CollectData {
                document: Document::from(
                    Text::from("Omnipotente Dios, recordamos hoy en tu presencia a tu siervo fiel, N., y te rogamos que, habiendo abierto para él las puertas de una vida más amplia, le recibas más y más en tu grato servicio, para que, con todos los que te han servido fielmente en el pasado, participe del triunfo eterno de Jesucristo nuestro Señor; que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("8. Por los Difuntos")
                .page(172)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Conmemoración de los Fieles Difuntos".into(),
                rubric_before: None,
                rubric_after: Some("Puede usarse cualquiera de las Colectas del Rito de Entierro en vez de las anteriores.\n\nPuede usarse una de las fórmulas señaladas en el Rito de Entierro para la Oración de los Fieles.\n\nPuede usarse la oración de poscomunión de la página 400.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::ReignOfChrist),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente y eterno, cuya voluntad es restaurar todas las cosas en tu muy amado Hijo, el Rey de reyes y Señor de señores: Concede, de tu piedad, que todos los pueblos de la tierra, divididos y esclavizados por el pecado, sean libertados y unificados bajo su reino de amor; quien vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("9. Del Reinado de Cristo")
                .page(172)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Ascensión, o del Bautismo".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Baptism),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, que por nuestro bautismo en la muerte y resurrección de tu Hijo Jesucristo nos conviertes de la vieja vida de pecado: Concede que, renaciendo a una vida nueva en él, vivamos en justicia y santidad todos nuestros días; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("10. En un Bautismo")
                .page(172)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Bautismo".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Confirmation),
            CollectData {
                document: Document::from(
                    Text::from("Concede, oh Dios todopoderoso, a quienes hemos sido redimidos de la vieja vida de pecado por nuestro bautismo en la muerte y resurrección de tu Hijo Jesucristo, que seamos renovados en tu Espíritu Santo, y vivamos en justicia y verdadera santidad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("11. En una Confirmación")
                .page(173)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Bautismo, o de Pentecostés".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Dedication),
            CollectData {
                document: Document::from(
                    Text::from("Dios todopoderoso, para cuya gloria celebramos la dedicación de esta casa de oración: Te damos gracias por la comunión de quienes te han adorado en este lugar, y te rogamos que cuantos aquí te busquen te encuentren, y sean llenos de tu gozo y paz; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre. Amén.La Letanía de Acción de Gracias por una Iglesia puede usarse para la Oración de los Fieles (Véase la página 480.)")
                        .response("Amén.")
                )
                .label("12. En el Aniversario de la Dedicación de una Iglesia")
                .page(173)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Dedicación de una Iglesia".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::ChurchConvention),
            CollectData {
                document: Document::from(
                    Text::from("Padre todopoderoso y eterno, tú nos has dado el Espíritu Santo para morar con nosotros por siempre: Bendice, te rogamos, con su gracia y presencia, a los obispos, los otros clérigos y los laicos aquí (o ahora, o que estarán) reunidos en tu Nombre, a fin de que tu Iglesia, preservada en la verdadera fe y santa disciplina, lleve a cabo todo lo que tuvo en mente aquél que la amó y se entregó por ella, tu Hijo Jesucristo nuestro Salvador; que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("13. Por una Convención de la Iglesia")
                .page(173)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Pentecostés, o de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::UnityOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("Padre omnipotente, cuyo bendito Hijo, antes de su pasión, oró por sus discípulos, para que fueran uno, como tú y él son uno: Concede que tu Iglesia, congregada en amor y obediencia a ti, sea unida en un solo cuerpo por un solo Espíritu, a fin de que el mundo crea en quien tú has enviado, tu Hijo Jesucristo nuestro Señor; que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("14. Por la Unidad de la Iglesia")
                .page(174)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Bautismo, o del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::EmberDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "I. Por los que van a ser ordenados"))),
                        Document::from(
                            Text::from("Dios omnipotente, dador de toda buena dádiva, por tu divina providencia has establecido diversas órdenes en tu Iglesia: Otorga tu gracia, humildemente te suplicamos, a todos los que son llamados [ahora] a cualquier oficio y ministerio para tu pueblo; llénalos con la verdad de tu doctrina, y revístelos de santidad de vida, de tal modo que te sirvan fielmente, para gloria de tu excelso Nombre y para beneficio de tu santa Iglesia; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                                .response("Amén.")
                        ),
                    ])
                )
                .label("15. Por el Ministerio (Días de Témporas)")
                .page(174)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de Apóstoles".into(),
                rubric_before: Some("Para usarse en los días tradicionales, o en otras ocasiones".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::EmberDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "II. Por la selección de personas idóneas para el ministerio"))),
                        Document::from(
                            Text::from("Oh Dios, que inspiraste a tus santos apóstoles a ordenar ministros en todo lugar: Concede que tu Iglesia, bajo la dirección del Espíritu Santo, escoja personas idóneas para el ministerio de la Palabra y de los Sacramentos, y susténtalas en su obra para la extensión de tu reino; por él que es el Pastor y Obispo de nuestras almas, Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                                .response("Amén.")
                        )
                    ])
                )
                .label("15. Por el Ministerio (Días de Témporas)")
                .page(175)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::EmberDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "III. Por todo cristiano en su vocación"))),
                        Document::from(
                            Text::from("Dios todopoderoso y eterno, cuyo Espíritu gobierna y santifica a todo el cuerpo de tu pueblo fiel: Recibe las súplicas y oraciones que te ofrecemos por todos los miembros de tu santa Iglesia, para que en su vocación y ministerio te sirvan verdadera y devotamente; por nuestro Señor y Salvador Jesucristo, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                                .response("Amén.")
                        )
                    ])
                )
                .label("15. Por el Ministerio (Días de Témporas)")
                .page(175)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Bautismo, o de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::MissionOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios, que has hecho de una sola sangre a todos los pueblos de la tierra, y enviaste a tu bendito Hijo a predicar la paz, tanto a los que están lejos como a los que están cerca: Concede que la gente en todo lugar te busque y te encuentre; trae a las naciones a tu redil; derrama tu Espíritu sobre toda carne; y apresura la venida de tu reino; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("16. Por la Misión de la Iglesia")
                .page(175)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("o la siguiente:".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::MissionOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("Oh Dios de todas la naciones de la tierra: Acuérdate de las multitudes que han sido creadas a tu imagen, pero no han conocido la obra redentora de nuestro Salvador Jesucristo; y concede que, por medio de las oraciones y del trabajo de tu santa Iglesia, sean traídas a conocerte y adorarte, según has sido revelado en tu Hijo; que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("16. Por la Misión de la Iglesia")
                .page(176)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación, o de Pentecostés".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Nation),
            CollectData {
                document: Document::from(
                    Text::from("Señor Dios omnipotente, que has hecho a todos los pueblos de la tierra para tu gloria, y para servirte en libertad y paz: Otorga a los habitantes de nuestro país tal celo por la justicia y tal fuerza de moderación, que usemos nuestra libertad de acuerdo con tu santa voluntad; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("17. Por la Nación")
                .page(176)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio del Domingo de Trinidad".into(),
                rubric_before: None,
                rubric_after: Some("En vez de esta Colecta puede usarse la del Día de la Independencia en la página 163.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Peace),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, enciende, te suplicamos, en cada corazón el verdadero amor por la paz, y dirige con tu sabiduría a los que deliberan en nombre de las naciones de la tierra; para que en tranquilidad tu señorío aumente hasta que toda la tierra se colme con el conocimiento de tu amor; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("18. Por la Paz")
                .page(176)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::RogationDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "I. Por estaciones fructíferas"))),
                        Document::from(
                            Text::from("Dios omnipotente, Señor del cielo y de la tierra: Te suplicamos humildemente que, de tu bondadosa providencia, nos des y conserves para nuestro uso los frutos de la tierra y de los mares, y hagas prosperar a todos los que trabajan para obtenerlos, a fin de que nosotros, que recibimos continuamente la abundancia de tu generosidad, te demos siempre gracias; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                                .response("Amén.")
                        )
                    ])
                )
                .label("19. Para los Días de Rogativa")
                .page(177)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: Some("Para usarse en los días tradicionales o en otras ocasiones".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::RogationDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "II. Por el comercio y la industria"))),
                        Document::from(
                        Text::from("Dios todopoderoso, cuyo Hijo Jesucristo en su vida terrenal compartió nuestro esfuerzo y santificó nuestro trabajo: Sé con tu pueblo dondequiera que trabaje; haz que cuantos se ocupan de la industria y el comercio de esta tierra sean sensibles a tu voluntad; concédenos satisfacción en todo lo que hagamos y una justa retribución por nuestro trabajo; por Jesucristo nuestro Señor, que vive y reina contigo, en la unidad del Espíritu Santo, un solo Dios, ahora y por siempre.")
                                .response("Amén.")
                        )
                    ])
                )
                .label("19. Para los Días de Rogativa")
                .page(177)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::RogationDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "III. Por la mayordomía de la creación"))),
                        Document::from(
                            Text::from("Oh bondadoso Creador, cuya mano está abierta para satisfacer las necesidades de todo ser viviente: Haz que seamos siempre agradecidos por tu amorosa providencia; y concede que, recordando la cuenta que un día hemos de rendir, seamos fieles mayordomos de tus dones; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                                .response("Amén.")
                        )
                    ])
                )
                .label("19. Para los Días de Rogativa")
                .page(178)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Sick),
            CollectData {
                document: Document::from(
                    Text::from("Padre celestial, dador de vida y de salud: Consuela y alivia a tus siervos enfermos (y especialmente a N., o NN.) y concede tu poder de sanidad a quienes les ministran en sus necesidades, para que aquéllos por quienes se ofrecen nuestras oraciones sean fortalecidos en su debilidad, y tengan confianza en tu amoroso cuidado; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("20. Por los Enfermos")
                .page(178)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: Some("Puede usarse la oración de poscomunión en la página 378.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::SocialJustice),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, que nos has creado a tu propia imagen: Concédenos gracia para luchar valerosamente contra el mal, y nunca transigir con la opresión; y, para que hagamos reverente uso de nuestra libertad, ayúdanos a emplearla en el sostenimiento de la justicia en nuestras comunidades y entre las naciones, para gloria de tu santo Nombre; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, ahora y por siempre.")
                        .response("Amén.")
                )
                .label("21. Por la Justicia Social")
                .page(179)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::SocialService),
            CollectData {
                document: Document::from(
                    Text::from("Padre celestial, cuyo bendito Hijo vino no para ser servido sino para servir: Bendice a todos aquéllos que, siguiendo sus huellas, se entregan al servicio de los demás; para que, con sabiduría, paciencia y valor, ministren en su Nombre a los que sufren, a los necesitados y a los que no tienen amigos; por amor de aquél que entregó su vida por nosotros, tu Hijo nuestro Salvador Jesucristo, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("22. Por el Servicio Social")
                .page(179)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Education),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, fuente de toda sabiduría: Ilumina con tu Espíritu Santo a los que enseñan y a los que aprenden, para que, regocijándose en el conocimiento de tu verdad, te adoren y te sirvan de generación en generación; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("23. Por la Educación")
                .page(179)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Vocation),
            CollectData {
                document: Document::from(
                    Text::from("Omnipotente Dios, nuestro Padre celestial, tú proclamas tu gloria y manifiestas la obra de tus manos en los cielos y en la tierra: Líbranos en nuestras diversas ocupaciones de servirnos a nosotros mismos, para que realicemos en verdad, con belleza y para el bien común, el trabajo que nos has encomendado; por amor de aquél que vino a nosotros como el que sirve, tu Hijo Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("24. Por la Vocación en el Trabajo Diario")
                .page(180)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::LaborDay),
            CollectData {
                document: Document::from(
                    Text::from("Dios omnipotente, de tal modo tú has relacionado nuestras vidas con las de los demás, que lo que hacemos nos afecta a todos, para bien o para mal: Guíanos de tal manera en nuestro trabajo a fin de no hacerlo para nosotros mismos, sino para el bien común; y así como buscamos una justa retribución por nuestro trabajo, haznos conscientes de las legítimas aspiraciones de los demás trabajadores, y despierta en nosotros inquietud por los desempleados; por Jesucristo nuestro Señor, que vive y reina contigo y el Espíritu Santo, un solo Dios, por los siglos de los siglos.")
                        .response("Amén.")
                )
                .label("25. Para el Día del Trabajo")
                .page(180)
                .tags(["Ocasiones Varias"])
                .version(Version::LibroDeOracionComun)
                .language(Language::Es),
                preface: "Prefacio de la Estación".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),

    ];
}
