use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_113: Psalm = Psalm {
		number: 113,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 647
				},
				local_name: String::from(""),
				latin_name: String::from("Laudate, pueri"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nAlaben las obras del Señor; *"),
						b: String::from("alaben el Nombre del Señor")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Sea bendito el Nombre del Señor, *"),
						b: String::from("desde ahora y para siempre.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Desde el nacimiento del sol hasta donde se pone, *"),
						b: String::from("sea alabado el Nombre del Señor.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Excelso sobre todas las naciones es el Señor, *"),
						b: String::from("sobre los cielos su gloria.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¿Quién como el Señor nuestro Dios,\nque se sienta entronizado en las alturas, *"),
						b: String::from("mas se humilla a mirar a los cielos y a la tierra?")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El levanta del polvo al desvalido, *"),
						b: String::from("y al menesteroso alza del muladar,")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Para sentarlos con los príncipes, *"),
						b: String::from("con los príncipes de su pueblo.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El hace que la mujer estéril *"),
						b: String::from("sea madre gozosa de hijos.")
					}
				]
			}
		]
	};
}