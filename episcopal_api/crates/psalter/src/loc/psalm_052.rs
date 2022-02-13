use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_52: Psalm = Psalm {
		number: 52,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 554
				},
				local_name: String::from(""),
				latin_name: String::from("Quid gloriaris?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¿Por qué te jactas de maldad, oh tirano, *"),
						b: String::from("contra el devoto todo el día?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Urdes agravios;\ncomo navaja afilada es tu lengua, *"),
						b: String::from("tú que obras engaño.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Amas el mal más que el bien, *"),
						b: String::from("la mentira más que la verdad.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Amas toda suerte de palabras hirientes, *"),
						b: String::from("oh lengua engañosa.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¡Oh, si Dios te derribara totalmente, *"),
						b: String::from("te asolara y te arrancara de tu morada,\ny te desarraigara de la tierra de los vivientes!")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Verán los justos, y temerán; *"),
						b: String::from("entonces se reirán de él, diciendo:")
					},
					PsalmVerse {
						number: 7,
						a: String::from("\"He aquí el que no puso a Dios por fortaleza, *"),
						b: String::from("sino que confió en sus muchas riquezas,\ny persistió en su maldad\".")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Pero yo estoy en la casa de Dios como olivo verde; *"),
						b: String::from("en la misericordia de Dios confío eternamente\ny para siempre.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Siempre te daré gracias por lo que has hecho, *"),
						b: String::from("y proclamaré, en la presencia de tus santos,\nque tu Nombre es bueno.")
					}
				]
			}
		]
	};
}