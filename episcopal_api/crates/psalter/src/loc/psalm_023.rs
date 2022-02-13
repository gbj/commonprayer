use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_23: Psalm = Psalm {
		number: 23,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 511
				},
				local_name: String::from(""),
				latin_name: String::from("Dominus regit me"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El Señor es mi pastor; *"),
						b: String::from("nada me faltará.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("En verdes pastos me hace yacer; *"),
						b: String::from("me conduce hacia aguas tranquilas.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Aviva mi alma *"),
						b: String::from("y me guía por sendas seguras por amor de su Nombre.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Aunque ande en valle de sombra de muerte, no temeré mal alguno; *"),
						b: String::from("porque tú estás conmigo;\ntu vara y tu cayado me infunden aliento.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Aderezarás mesa delante de mi\nen presencia de mis angustiadores; *"),
						b: String::from("unges mi cabeza con óleo; mi copa está rebosando.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Ciertamente el bien y la misericordia me seguirán todos los días de mi vida, *"),
						b: String::from("y en la casa del Señor moraré por largos días.")
					}
				]
			}
		]
	};
}