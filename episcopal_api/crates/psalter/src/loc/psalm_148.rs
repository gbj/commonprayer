use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_148: Psalm = Psalm {
		number: 148,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 696
				},
				local_name: String::from(""),
				latin_name: String::from("Laudate Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nAlaben al Señor desde los cielos; *"),
						b: String::from("alábenle en las alturas.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Alábenle, todos sus ángeles; *"),
						b: String::from("alábenle, toda su hueste.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Alábenle, sol y luna; *"),
						b: String::from("alábenle, todas las estrellas lucientes.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Alábenle, cielos de los cielos; *"),
						b: String::from("alábenle, aguas que están sobre los cielos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Alaben el Nombre del Señor, *"),
						b: String::from("porque él mandó, y fueron creados.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Los afirmó eternamente y para siempre; *"),
						b: String::from("les dio una ley que no pasará.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Alaben al Señor desde la tierra, *"),
						b: String::from("monstruos marinos y todos los abismos;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Fuego y granizo, nieve y bruma, *"),
						b: String::from("viento tempestuoso que ejecuta su voluntad;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Montes y todas las colinas, *"),
						b: String::from("árboles frutales y todos los cedros;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Bestias silvestres y todo ganado, *"),
						b: String::from("reptiles y aves aladas;")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Reyes de la tierra y todos los pueblos, *"),
						b: String::from("príncipes y jefes del mundo;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Mozos y doncellas, *"),
						b: String::from("viejos y jóvenes juntos.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Alaben el Nombre del Señor, *"),
						b: String::from("porque sólo su Nombre es excelso,\nsu gloria sobre la tierra y los cielos.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Ha alzado el cuerno de su pueblo, y alabanza para todos sus fieles, *"),
						b: String::from("los hijos de Israel, el pueblo cercano a él. ¡Aleluya!")
					}
				]
			}
		]
	};
}