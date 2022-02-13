use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_130: Psalm = Psalm {
		number: 130,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 675
				},
				local_name: String::from(""),
				latin_name: String::from("De profundis"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("De lo profundo, oh Señor, a ti clamo; Señor, escucha mi voz; *"),
						b: String::from("estén atentos tus oídos a la voz de mi súplica.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Si tú, oh Señor, notares los delitos, *"),
						b: String::from("¿quién, oh Señor, podrá mantenerse?")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Mas en ti hay perdón, *"),
						b: String::from("por tanto serás venerado.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Aguardo al Señor; le aguarda mi alma; *"),
						b: String::from("en su palabra está mi esperanza.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Mi alma aguarda al Señor,\nmás que los centinelas a la aurora, *"),
						b: String::from("más que los centinelas a la aurora.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Oh Israel, aguarda al Señor, *"),
						b: String::from("porque en el Señor hay misericordia;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Con él hay abundante redención, *"),
						b: String::from("y él redimirá a Israel de todos sus pecados.")
					}
				]
			}
		]
	};
}