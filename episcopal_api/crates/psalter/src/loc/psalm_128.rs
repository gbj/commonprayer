use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_128: Psalm = Psalm {
		number: 128,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 674
				},
				local_name: String::from(""),
				latin_name: String::from("Beati omnes"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Dichosos todos los que temen al Señor; *"),
						b: String::from("y andan en sus caminos!")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Comerás el fruto de tu trabajo; *"),
						b: String::from("dicha y prosperidad tendrás.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Tu mujer será como parra fecunda en medio de tu casa, *"),
						b: String::from("tus hijos como renuevos de olivo alrededor de tu mesa.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Así será bendecido el hombre *"),
						b: String::from("que teme al Señor.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Bendígate el Señor desde Sion, *"),
						b: String::from("y veas la prosperidad de Jerusalén\ntodos los días de tu vida.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Que veas los hijos de tus hijos, *"),
						b: String::from("y la paz sea sobre Israel.")
					}
				]
			}
		]
	};
}