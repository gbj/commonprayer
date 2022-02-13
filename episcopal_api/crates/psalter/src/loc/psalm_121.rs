use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_121: Psalm = Psalm {
		number: 121,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 670
				},
				local_name: String::from(""),
				latin_name: String::from("Levavi oculos"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Levanto mis ojos a los montes; *"),
						b: String::from("¿de dónde vendrá mi socorro?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Mi socorro viene del Señor, *"),
						b: String::from("que hizo los cielos y la tierra.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("No permitirá que resbale tu pie, *"),
						b: String::from("ni se dormirá el que te guarda.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("He aquí, el que guarda a Israel *"),
						b: String::from("no se adormecerá ni dormirá.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("El Señor es tu guardián, *"),
						b: String::from("el Señor es tu sombra a tu diestra.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Elsolnoteharádañodedía,*"),
						b: String::from("ni la luna de noche.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("El Señor te guardará de todo mal; *"),
						b: String::from("él guardará tu vida.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El Señor guardará tu salida y tu entrada, *"),
						b: String::from("desde ahora y para siempre.")
					}
				]
			}
		]
	};
}