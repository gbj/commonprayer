use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_48: Psalm = Psalm {
		number: 48,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 548
				},
				local_name: String::from(""),
				latin_name: String::from("Magnus Dominus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Grande es el Señor, y digno de toda alabanza; *"),
						b: String::from("en la ciudad de nuestro Dios está su santo monte.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Hermoso y sublime, el gozo de toda la tierra, es el monte de Sión, *"),
						b: String::from("corazón del mundo y ciudad del gran Rey.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Dios está en su ciudadela; *"),
						b: String::from("descuella como un alcázar.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("He aquí los reyes de la tierra se aliaron; *"),
						b: String::from("y juntos avanzaron contra ella.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Al verla, se pasmaron; *"),
						b: String::from("se turbaron y huyeron.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Allí se estremecieron, *"),
						b: String::from("se retorcían como mujer que pare,\ncomo naves del mar cuando el solano las quebranta.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Lo que habíamos oído, lo hemos visto,\nen la ciudad del Señor de las huestes, en la\nciudad de nuestro Rey: *"),
						b: String::from("Dios la ha establecido para siempre.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Hemos meditado en tu bondad, oh Dios, *"),
						b: String::from("en medio de tu templo.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Tu alabanza, como tu Nombre, oh Dios, llega hasta los confines de la tierra; *"),
						b: String::from("de justicia está llena tu diestra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Alégrese el monte de Sión, gócense las ciudades de Judá, *"),
						b: String::from("a causa de tu Providencia.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Anden alrededor de Sión, rodéenla; *"),
						b: String::from("cuenten las torres que tiene.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Consideren bien su antemuro, examinen sus fuertes, *"),
						b: String::from("para que puedan contarlo a la generación venidera;")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Porque este Dios es nuestro Dios, eternamente y para siempre; *"),
						b: String::from("él nos guiará por siempre jamás.")
					}
				]
			}
		]
	};
}