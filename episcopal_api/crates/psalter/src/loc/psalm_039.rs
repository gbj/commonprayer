use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_39: Psalm = Psalm {
		number: 39,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 536
				},
				local_name: String::from(""),
				latin_name: String::from("Dixi, Custodiam"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Yo dije: \"Atenderé a mis caminos, *"),
						b: String::from("para no pecar con mi lengua.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Pondré bozal en mi boca, *"),
						b: String::from("en tanto que el maligno esté delante de mí")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Enmudecí, guardé silencio, *"),
						b: String::from("me refrené de palabras imprudentes,\npero se agravó mi tormento.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Se enardeció mi corazón dentro de mí pensándolo, me requemaba; *"),
						b: String::from("hasta que solté la lengua:")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Hazme saber, oh Señor, mi fin,\ny cuánta sea la medida de mis días,\npara que sepa yo cuán frágil soy."),
						b: String::from("")
					},
					PsalmVerse {
						number: 6,
						a: String::from("He aquí, me diste sólo un puñado de días,\ny toda mi vida es como nada en tu presencia; *"),
						b: String::from("ciertamente no más que un soplo es todo mortal.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Merodeamos como una sombra, y en vano nos afanamos; *"),
						b: String::from("amontonamos riquezas, y no sabemos quién las recogerá.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Y ahora, Señor, ¿qué esperaré? *"),
						b: String::from("Mi esperanza está en ti.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Líbrame de todas mis transgresiones;\nno me pongas por escarnio del insensato."),
						b: String::from("")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Enmudecí, no abrí la boca; *"),
						b: String::from("porque tú eres el que actúa.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Quítame tu aflicción; *"),
						b: String::from("estoy consumido por los golpes de tu mano.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Nos reprendes con castigos por el pecado; como polilla deshaces nuestro cuerpo; *"),
						b: String::from("ciertamente no más que un soplo es todo mortal.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Oye mi oración, oh Señor, y escucha mi clamor; *"),
						b: String::from("no calles ante mis lágrimas;")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Porque forastero soy para ti, *"),
						b: String::from("y advenedizo, como todos mis antepasados.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Desvía de mí tu mirada, para que me consuele un poco, *"),
						b: String::from("antes de que me vaya, y deje de existir.")
					}
				]
			}
		]
	};
}