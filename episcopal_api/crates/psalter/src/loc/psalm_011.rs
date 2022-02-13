use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_11: Psalm = Psalm {
		number: 11,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 495
				},
				local_name: String::from(""),
				latin_name: String::from("In Domino confido"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("En el Señor he confiado; *"),
						b: String::from("¿cómo dicen ustedes a mi alma:\n\"Escapa al monte cual ave?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Porque he aquí, los malos tienden el arco, y disponen sus saetas sobre la cuerda, *"),
						b: String::from("para asaetar en oculto a los rectos de corazón;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Si fueren destruidos los fundamentos, *"),
						b: String::from("¿qué ha de hacer el justo?\"")
					},
					PsalmVerse {
						number: 4,
						a: String::from("El Señor está en su santo templo; *"),
						b: String::from("el Señor tiene en el cielo su trono.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Sus ojos observan, sus párpados examinan *"),
						b: String::from("a los habitantes de la tierra.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El Señor examina al justo y al malo; *"),
						b: String::from("pero al que ama la violencia lo aborrece.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Sobre los malos hará llover brasas, fuego y azufre; *"),
						b: String::from("viento abrasador será la porción de su cáliz;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Porque el Señor es justo, y ama la justicia; *"),
						b: String::from("quien es recto mirará su rostro.\nDía Segundo: oración Vespertina")
					}
				]
			}
		]
	};
}