use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_15: Psalm = Psalm {
		number: 15,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 498
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, quis habitabit?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Señor, ¿quién habitará en tu tabernáculo? *"),
						b: String::from("¿Quién morará en tu santo monte?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("El que anda en integridad y hace justicia, *"),
						b: String::from("y habla verdad en su corazón.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("El que no detrae con su lengua, ni hace mal a su prójimo, *"),
						b: String::from("ni contra su vecino acoje oprobio alguno.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Aquél a cuyos ojos el vil es menospreciado, *"),
						b: String::from("pero honra a los que temen al Señor.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("El que jurando en daño suyo, *"),
						b: String::from("no por eso cambia.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El que presta, no esperando de ello nada, *"),
						b: String::from("ni contra el inocente admite cohecho.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("El que hace estas cosas, *"),
						b: String::from("no resbalará para siempre.")
					}
				]
			}
		]
	};
}