use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_110: Psalm = Psalm {
		number: 110,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 645
				},
				local_name: String::from(""),
				latin_name: String::from("Dixit Dominus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El Señor dijo a mi soberano: \"Siéntate a mi diestra, *"),
						b: String::from("hasta que ponga a tus enemigos por estrado de tus pies\".")
					},
					PsalmVerse {
						number: 2,
						a: String::from("El Señor enviará desde Sión el cetro de tu poder, *"),
						b: String::from("diciendo: \"Domina en medio de tus enemigos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Dignidad principesca ha sido tuya desde el día de tu nacimiento; *"),
						b: String::from("en la hermosura de la santidad te engendré, como rocío del seno de la aurora\".")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Juró el Señor, y no se retractará: *"),
						b: String::from("\"Tú eres sacerdote para siempre, según el orden de Melquisedec\".")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Mi soberano que está a tu diestra quebrantará a los reyes en el día de su ira; *"),
						b: String::from("dominará sobre las naciones.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Amontonará los cadáveres; *"),
						b: String::from("quebrantará las cabezas sobre la ancha tierra.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Junto al camino beberá del arroyo; *"),
						b: String::from("por tanto levantará la cabeza.")
					}
				]
			}
		]
	};
}