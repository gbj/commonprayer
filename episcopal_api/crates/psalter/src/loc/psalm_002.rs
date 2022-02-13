use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_2: Psalm = Psalm {
		number: 2,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 486
				},
				local_name: String::from(""),
				latin_name: String::from("Quare fremuerunt gentes?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¿Por qué se amotinan las gentes, *"),
						b: String::from("y los pueblos piensan cosas vanas?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("¿Por qué se levantan los reyes de la tierra, y príncipes consultan unidos *"),
						b: String::from("contra el Señor y contra su Ungido?")
					},
					PsalmVerse {
						number: 3,
						a: String::from("\"Rompamos sus ligaduras\", dicen; *"),
						b: String::from("\"echemos de nosotros sus cuerdas\".")
					},
					PsalmVerse {
						number: 4,
						a: String::from("El que mora en los cielos se ríe; *"),
						b: String::from("el Señor se burla de ellos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Luego les habla en su furor, *"),
						b: String::from("y los turba con su ira, diciendo:")
					},
					PsalmVerse {
						number: 6,
						a: String::from("\"Yo mismo he puesto mi rey *"),
						b: String::from("sobre Sión, mi santo monte\".")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Yo publicaré el decreto: *"),
						b: String::from("El Señor me ha dicho: \"Mi Hijo eres tú;\nyo te engendré hoy.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Pídeme, y te daré por herencia las naciones, *"),
						b: String::from("y como posesión tuya los confines de la tierra.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Los quebrantarás con vara de hierro, *"),
						b: String::from("como vasija de alfarero los desmenuzarás\".")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Ahora, pues, oh reyes, sean prudentes; *"),
						b: String::from("admitan amonestación, jueces de la tierra.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Sirvan al Señor con temor, *"),
						b: String::from("y alégrense con temblor.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Honren al Hijo, para que no se enoje, y perezcan en el camino; *"),
						b: String::from("pues se inflama de pronto su ira.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Bienaventurados son *"),
						b: String::from("todos los que en él confían.")
					}
				]
			}
		]
	};
}