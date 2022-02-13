use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_86: Psalm = Psalm {
		number: 86,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 604
				},
				local_name: String::from(""),
				latin_name: String::from("Inclina, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Inclina, oh Señor, tu oído, y respóndeme, *"),
						b: String::from("porque estoy afligido y menesteroso.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Guarda mi vida, pues te soy fiel; *"),
						b: String::from("salva a tu siervo que en ti confía.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Ten misericordia de mí, porque tú eres mi Dios; *"),
						b: String::from("a ti clamo todo el día.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Alegra el alma de tu siervo, *"),
						b: String::from("porque a ti, oh Señor, levanto mi alma;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Porque tú, oh Señor, eres bueno y clemente, *"),
						b: String::from("y rico en misericordia con los que te invocan.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Escucha, oh Señor, mi oración; *"),
						b: String::from("atiende a la voz de mi súplica.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("En el día de mi angustia te llamaré, *"),
						b: String::from("porque tú me responderás.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Oh Señor, ninguno hay como tú entre los dioses, *"),
						b: String::from("ni nada que iguale tus obras.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Todas las naciones que hiciste, oh Señor, vendrán a adorarte, *"),
						b: String::from("y glorificarán tu Nombre;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Porque tú eres grande, y hacedor de maravillas; *"),
						b: String::from("sólo tú eres Dios.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Enséñame, oh Señor, tu camino, para que siga yo en tu verdad; *"),
						b: String::from("afirma mi corazón, para que tema tu Nombre.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Te daré gracias de todo corazón, oh Señor mi Dios; *"),
						b: String::from("glorificaré tu Nombre para siempre;")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Porque grande es tu misericordia para conmigo; *"),
						b: String::from("me has librado del Abismo profundo.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Oh Dios, los soberbios se levantan contra mí; una banda de hombres violentos busca mi vida; *"),
						b: String::from("no te han puesto delante de sus ojos;")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Mas tú, oh Señor, eres misericordioso y clemente, *"),
						b: String::from("tardo para la ira, y rico en gracia y verdad.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Mírame, y ten misericordia de mí; *"),
						b: String::from("da de tu fuerza a tu siervo,\ny salva al hijo de tu sierva.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Dame una señal de tu favor,\npara que la vean los que me odian, y se avergüencen; *"),
						b: String::from("porque tú, oh Señor, me ayudaste y me consolaste.")
					}
				]
			}
		]
	};
}