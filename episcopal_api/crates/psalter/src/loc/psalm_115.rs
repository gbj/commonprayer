use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_115: Psalm = Psalm {
		number: 115,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 649
				},
				local_name: String::from(""),
				latin_name: String::from("Non nobis, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("No a nosotros, oh Señor, no a nosotros, sino a tu Nombre da gloria, *"),
						b: String::from("a causa de tu bondad, de tu fidelidad.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("¿Por qué han de decir los paganos: *"),
						b: String::from("\"Dónde está ahora su Dios?\"")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Nuestro Dios está en los cielos; *"),
						b: String::from("lo que quiere, lo hace.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Los ídolos de ellos son plata y oro, *"),
						b: String::from("hechura de manos humanas.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Boca tienen, mas no hablan; *"),
						b: String::from("ojos tienen, mas no ven;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Orejas tienen, mas no oyen; *"),
						b: String::from("narices tienen, mas no huelen;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Manos tienen, mas no palpan; pies tienen, mas no andan; *"),
						b: String::from("no tiene voz su garganta.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Semejantes a ellos son los que los hacen, *"),
						b: String::from("y cualquiera que confía en ellos.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Oh Israel, confía en el Señor; *"),
						b: String::from("él es su ayuda y su escudo.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Oh casa de Aarón, confía en el Señor; *"),
						b: String::from("él es su ayuda y su escudo.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Los que temen al Señor, confíen en el Señor; *"),
						b: String::from("él es su ayuda y su escudo.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("El Señor se acordó de nosotros, y nos bendecirá; *"),
						b: String::from("bendecirá a la casa de Israel;\nbendecirá a la casa de Aarón.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Bendecirá a los que temen al Señor, *"),
						b: String::from("tanto a pequeños como a grandes.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Que el Señor les aumente más y más, *"),
						b: String::from("a ustedes y a su descendencia.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Sean bendecidos por el Señor, *"),
						b: String::from("que hizo los cielos y la tierra.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Los cielos de los cielos son del Señor, *"),
						b: String::from("mas la tierra se la ha dado a sus pueblos.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("No alaban los muertos al Señor, *"),
						b: String::from("ni cuantos descienden al silencio;")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Pero nosotros bendeciremos al Señor, *"),
						b: String::from("desde ahora y para siempre.\n¡Aleluya!")
					}
				]
			}
		]
	};
}