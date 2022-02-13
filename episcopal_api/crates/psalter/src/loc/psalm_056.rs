use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_56: Psalm = Psalm {
		number: 56,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 559
				},
				local_name: String::from(""),
				latin_name: String::from("Miserere mei, Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Ten misericordia de mí, oh Dios, porque me hostigan mis enemigos; *"),
						b: String::from("me atacan y me acosan todo el día.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Todo el día me hostigan; *"),
						b: String::from("en verdad, son muchos los que pelean contra mí,\noh Altísimo.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Cuando tengo miedo, *"),
						b: String::from("en ti confío.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("En Dios, cuya palabra alabo, en Dios confío, y no temo; *"),
						b: String::from("¿qué pueden hacerme los mortales?")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Todo el día pervierten mi causa; *"),
						b: String::from("sólo piensan en hacerme daño.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Se apandillan; me acechan; *"),
						b: String::from("espían mis pasos, porque me quieren matar.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("¿Escaparán ellos, a pesar de su iniquidad? *"),
						b: String::from("Oh Dios, en tu furor derriba a los pueblos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Tú has notado mis gemidos;\nhas puesto mis lágrimas en tu redoma; *"),
						b: String::from("¿no están ellos en tu libro?")
					},
					PsalmVerse {
						number: 9,
						a: String::from("En el día que te invoque, mis enemigos serán dispersos; *"),
						b: String::from("esto sé, porque Dios está de mi parte.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("En Dios el Señor, cuya palabra alabo, en Dios confío, y no temo; *"),
						b: String::from("¿qué pueden hacerme los mortales?")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Te debo, oh Dios, los votos que hice; *"),
						b: String::from("los cumpliré con acción de gracias;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Porque has librado mi vida de la muerte, mis pies de la caída, *"),
						b: String::from("para que ande delante de Dios en la luz de los que viven.")
					}
				]
			}
		]
	};
}