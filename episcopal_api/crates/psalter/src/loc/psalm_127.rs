use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_127: Psalm = Psalm {
		number: 127,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 673
				},
				local_name: String::from(""),
				latin_name: String::from("Nisi Dominus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Si el Señor no edificare la casa, *"),
						b: String::from("en vano trabajan los que la edifican.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Si el Señor no guardare la ciudad, *"),
						b: String::from("en vano vela el vigilante.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Vano es madrugar y acostarse tarde, vano también comer el pan del trabajo; *"),
						b: String::from("pues a su amado le da el sueño.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("He aquí, herencia del Señor son los hijos, *"),
						b: String::from("y el fruto del vientre, un don.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Como saetas en manos de un guerrero, *"),
						b: String::from("así son los hijos de nuestra juventud.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("¡Dichoso el que llena con ellas su aljaba! *"),
						b: String::from("No será avergonzado cuando contienda\ncon sus adversarios en la puerta.")
					}
				]
			}
		]
	};
}