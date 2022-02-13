use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_120: Psalm = Psalm {
		number: 120,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 669
				},
				local_name: String::from(""),
				latin_name: String::from("Ad Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Al Señor clamé en mi angustia; *"),
						b: String::from("clamé, y él me respondió.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Líbrame, oh Señor, de los labios mentirosos, *"),
						b: String::from("de la lengua engañosa.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("¿Qué te hará, y qué te añadirá, *"),
						b: String::from("oh lengua engañosa?")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Agudas saetas de guerrero, *"),
						b: String::from("con ascuas de retama.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¡Ay de mí, que he de morar en Mesec, *"),
						b: String::from("y habitar entre las tiendas de Cedar!")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Demasiado he sufrido, *"),
						b: String::from("viviendo con los que odian la paz.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Estoy del lado de la paz, *"),
						b: String::from("pero cuando digo: \"Paz\", ellos dicen: \"Guerra\".")
					}
				]
			}
		]
	};
}