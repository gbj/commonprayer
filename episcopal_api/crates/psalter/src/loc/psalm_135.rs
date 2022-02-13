use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_135: Psalm = Psalm {
		number: 135,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 679
				},
				local_name: String::from(""),
				latin_name: String::from("Laudate nomen"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nAlaben el Nombre del Señor; *"),
						b: String::from("alábenle, siervos del Señor,")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Los que están de pie en la casa del Señor, *"),
						b: String::from("en los atrios de la casa de nuestro Dios.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Alaben al Señor, porque el Señor es bueno; *"),
						b: String::from("canten alabanzas a su Nombre, que es amable;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque el Señor ha escogido a Jacob para sí, *"),
						b: String::from("y a Israel por posesión suya.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Yo sé que el Señor es grande, *"),
						b: String::from("y nuestro Soberano, mayor que todos los dioses.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El Señor hace lo que quiere *"),
						b: String::from("en los cielos y en la tierra,\nen los mares y en todos los abismos.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Hace subir las nubes de los linderos de la tierra; *"),
						b: String::from("con los relámpagos desata la lluvia,\ny saca de sus depósitos los vientos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El es quien hirió a los primogénitos de Egipto, *"),
						b: String::from("tanto del hombre como de la bestia.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Envió señales y prodigios en medio de ti, oh Egipto, *"),
						b: String::from("contra Faraón y contra todos sus siervos.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Derribó a grandes naciones *"),
						b: String::from("y mató a reyes poderosos:")
					},
					PsalmVerse {
						number: 11,
						a: String::from("A Sehón, rey amorreo, y a Og, rey de Basán, *"),
						b: String::from("y a todos los reinos de Canaán.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Dio la tierra de ellos en heredad, *"),
						b: String::from("en heredad a Israel su pueblo.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Oh Señor, eterno es tu Nombre, *"),
						b: String::from("tu renombre, oh Señor, de edad en edad.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("El Señor defiende a su pueblo, *"),
						b: String::from("y a sus siervos muestra compasión.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Los ídolos de los paganos son plata y oro, *"),
						b: String::from("hechura de manos humanas.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Boca tienen, mas no hablan; *"),
						b: String::from("ojos tienen, mas no ven;")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Orejas tienen, mas no oyen; *"),
						b: String::from("no hay aliento en su boca.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Semejantes a ellos son los que los hacen, *"),
						b: String::from("y cualquiera que confía en ellos.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Casa de Israel, bendice al Señor; *"),
						b: String::from("casa de Aarón, bendice al Señor;")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Casa de Leví, bendice al Señor; *"),
						b: String::from("los que temen al Señor, bendigan al Señor.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Desde Sión sea bendito el Señor, *"),
						b: String::from("quien mora en Jerusalén.\n¡Aleluya!")
					}
				]
			}
		]
	};
}