use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_88: Psalm = Psalm {
		number: 88,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 606
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, mi Dios, mi Salvador, *"),
						b: String::from("día y noche clamo a ti.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Llegue mi oración a tu presencia; *"),
						b: String::from("inclina tu oído a mi lamento;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque estoy hastiado de desdichas, *"),
						b: String::from("y mi vida está al borde de la tumba.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Soy contado entre los que bajan a la fosa; *"),
						b: String::from("soy como un inválido,")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Perdido entre los muertos, *"),
						b: String::from("como los caídos que yacen en el sepulcro,")
					},
					PsalmVerse {
						number: 6,
						a: String::from("De quienes no te acuerdas ya, *"),
						b: String::from("porque fueron arrancados de tu mano.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Me has colocado en lo profundo de la fosa, *"),
						b: String::from("en las tinieblas y en el abismo.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Pesa duramente sobre mí tu ira; *"),
						b: String::from("todas tus grandes olas me hunden.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Has alejado de mí a mis amigos;\nme has puesto por abominación ante ellos; *"),
						b: String::from("encerrado estoy, y no puedo salir.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los ojos se me nublan a causa de mi aflicción; *"),
						b: String::from("todos los días te he invocado, oh Señor;\nhe extendido a ti mis manos.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("¿Harás maravillas por los difuntos? *"),
						b: String::from("¿Se levantarán para darte gracias los que han muerto?")
					},
					PsalmVerse {
						number: 12,
						a: String::from("¿Será anunciada en el sepulcro tu misericordia, *"),
						b: String::from("o tu fidelidad en el reino de la muerte?")
					},
					PsalmVerse {
						number: 13,
						a: String::from("¿Serán reconocidas en las tinieblas tus maravillas, *"),
						b: String::from("o tu justicia en el país del olvido?")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Mas yo, oh Señor, te pido auxilio; *"),
						b: String::from("de mañana mi oración se presentará delante de ti.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("¿Por qué, oh Señor, me has rechazado? *"),
						b: String::from("¿Por qué escondes de mí tu rostro?")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Desde niño, he sido desgraciado\ny he estado al borde de la muerte; *"),
						b: String::from("he soportado tus terrores con mente medrosa.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Sobre mí ha pasado tu ira flamante, *"),
						b: String::from("y me han consumido tus terrores.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Me rodean como un diluvio todo el día; *"),
						b: String::from("a una me han cercado.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Has alejado de mí al amigo y al vecino, *"),
						b: String::from("y la oscuridad es mi única compañera.")
					}
				]
			}
		]
	};
}