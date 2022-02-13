use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_18: Psalm = Psalm {
		number: 18,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 501
				},
				local_name: String::from("Parte I"),
				latin_name: String::from("Diligam te, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Te amo, oh Señor, fortaleza mía, *"),
						b: String::from("oh Señor, castillo mío, mi risco y mi abrigo.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Dios mío, roca mía en quien confiaré, *"),
						b: String::from("mi escudo, el cuerno de mi salvación y mi alto refugio,\neres digno de ser alabado.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Invocaré al Señor, *"),
						b: String::from("y seré salvo de mis enemigos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Me sumergieron las olas de muerte, *"),
						b: String::from("y torrentes del abismo me atemorizaron.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Ligaduras infernales me rodearon, *"),
						b: String::from("previniéronme lazos de muerte.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("En mi angustia invoqué al Señor, *"),
						b: String::from("y clamé a mi Dios.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("El oyó mi voz desde su templo, *"),
						b: String::from("y mi clamor llegó delante de él, a sus oídos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Se sacudió y tembló la tierra, *"),
						b: String::from("temblaron los cimientos de los montes,\nse sacudieron, porque él se indignó.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Humo subió de su nariz\ny de su boca fuego consumidor; *"),
						b: String::from("carbones fueron por él encendidos.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Partió los cielos, y descendió; *"),
						b: String::from("y había densas tinieblas debajo de sus pies.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Cabalgó sobre un querubín, y voló; *"),
						b: String::from("sobre las alas del viento se abalanzó.")
					},
					PsalmVerse {
						number: 12,
						a: String::from(". Puso tinieblas por su escondedero, su pabellón en derredor de sí; *"),
						b: String::from("oscuridad de aguas, nubes de los cielos.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Por el resplandor de su presencia, sus nubes pasaron; *"),
						b: String::from("granizo y carbones ardientes.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Tronó en los cielos el Señor; *"),
						b: String::from("el Altísimo dio su voz.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Envió sus saetas, y los disperso; *"),
						b: String::from("lanzó relámpagos, y los destruyó.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Aparecieron las honduras de las aguas,\ny se descubrieron los cimientos del mundo, *"),
						b: String::from("a tu grito de guerra, oh Señor,\npor la ráfaga del aliento de tu nariz.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Alargó la mano desde lo alto, y me agarró; *"),
						b: String::from("me sacó de las aguas profundas.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Me libró de mi poderoso enemigo, y de los que me aborrecían, *"),
						b: String::from("pues eran más fuertes que yo.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Me asaltaron en el día de mi quebranto, *"),
						b: String::from("mas el Señor fue mi apoyo.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 503
				},
				local_name: String::from("Parte II"),
				latin_name: String::from("Et retribuet mihi"),
				verses: vec![
					PsalmVerse {
						number: 20,
						a: String::from("Me sacó a un lugar espacioso; *"),
						b: String::from("me libró porque se agradó de mí.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("El Señor me ha premiado conforme a mi justicia; *"),
						b: String::from("conforme a la limpieza de mis manos me ha\nrecompensado;")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Porque yo he guardado los caminos del Señor, *"),
						b: String::from("y no me aparté impíamente de mi Dios.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Pues todos sus juicios estuvieron delante de mí, *"),
						b: String::from("y no me he apartado de sus estatutos.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("Fui íntegro para con él, *"),
						b: String::from("y me he apartado de iniquidad;")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Por lo cual me ha recompensado el Señor conforme a mi justicia, *"),
						b: String::from("conforme a la limpieza de mis manos delante de su vista.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Con el fiel te mostrarás fiel, *"),
						b: String::from("con el íntegro tú eres íntegro.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Con el sincero tú eres sincero; *"),
						b: String::from("pero con el astuto tú eres sagaz.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Tú salvarás al pueblo humilde, *"),
						b: String::from("y humillarás los ojos altivos.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Oh Señor, tú eres mi lámpara; *"),
						b: String::from("Dios mío, tú alumbras mis tinieblas.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Contigo abatiré cualquier baluarte; *"),
						b: String::from("con mi Dios puedo escalar cualquier muralla.")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Dios, perfecto su camino; acrisoladas las palabras del Señor; *"),
						b: String::from("escudo a todos los que en él esperan;")
					},
					PsalmVerse {
						number: 32,
						a: String::from("Porque ¿quién es Dios sino sólo el Señor? *"),
						b: String::from("¿Qué Roca hay fuera de nuestro Dios?")
					},
					PsalmVerse {
						number: 33,
						a: String::from("Dios es el que me inviste de fuerza, *"),
						b: String::from("quien hace perfecto mi camino.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("El hace mis pies como pies de ciervos, *"),
						b: String::from("y me hace estar firme sobre las alturas.")
					},
					PsalmVerse {
						number: 35,
						a: String::from("El adiestra mis manos para la batalla, *"),
						b: String::from("y mis brazos para tensar un arco de bronce.")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Me diste asimismo el escudo de tu victoria; *"),
						b: String::from("tu diestra me sustentó,\ny tu benignidad me ha engrandecido.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("Ensanchaste mis pasos debajo de mí, *"),
						b: String::from("y no flaquearon mis tobillos.")
					},
					PsalmVerse {
						number: 38,
						a: String::from("Perseguí a mis enemigos, y los alcancé, *"),
						b: String::from("y no volví hasta acabarlos.")
					},
					PsalmVerse {
						number: 39,
						a: String::from("Los herí de modo que no se levantasen; *"),
						b: String::from("cayeron debajo de mis pies.")
					},
					PsalmVerse {
						number: 40,
						a: String::from("Me investiste de fuerzas para la pelea; *"),
						b: String::from("has humillado a mis enemigos debajo de mí;\nhas hecho que mis enemigos me vuelvan las espaldas.")
					},
					PsalmVerse {
						number: 41,
						a: String::from("Destruí a los que me aborrecían; clamaron, y no hubo quien los salvase; *"),
						b: String::from("aun al Señor, pero no los oyó.")
					},
					PsalmVerse {
						number: 42,
						a: String::from("Los molí como polvo delante del viento; *"),
						b: String::from("los pisoteé como lodo de las calles.")
					},
					PsalmVerse {
						number: 43,
						a: String::from("Me has librado de las contiendas del pueblo; *"),
						b: String::from("me has hecho cabeza de las naciones.")
					},
					PsalmVerse {
						number: 44,
						a: String::from("Pueblo que yo no conocía me sirvió; al oír de mí, me obedecieron; *"),
						b: String::from("extranjeros se acobardaron delante de mí.")
					},
					PsalmVerse {
						number: 45,
						a: String::from("Los extranjeros flaquearon, *"),
						b: String::from("y salieron temblando de sus encierros.")
					},
					PsalmVerse {
						number: 46,
						a: String::from("¡Viva el Señor! ¡Bendita sea mi Roca! *"),
						b: String::from("¡Ensalzado sea el Dios de mi salvación!")
					},
					PsalmVerse {
						number: 47,
						a: String::from("Es el Dios que me dio el desquite, *"),
						b: String::from("y sometió pueblos debajo de mí;")
					},
					PsalmVerse {
						number: 48,
						a: String::from("El que me libra de mis enemigos,\ny aun me eleva sobre los que se levantan contra mí; *"),
						b: String::from("me libraste del varón violento.")
					},
					PsalmVerse {
						number: 49,
						a: String::from("Por tanto yo te confesaré entre las naciones, oh Señor, *"),
						b: String::from("y cantaré himnos a tu Nombre.")
					},
					PsalmVerse {
						number: 50,
						a: String::from("Grandes triunfos da a su rey; *"),
						b: String::from("hace misericordia a su ungido,\na David y a su descendencia para siempre.")
					}
				]
			}
		]
	};
}