use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_126: Psalm = Psalm {
		number: 126,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 673
				},
				local_name: String::from(""),
				latin_name: String::from("In convertendo"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Cuando el Señor cambió la suerte de Sión, *"),
						b: String::from("éramos como los que sueñan.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Entonces nuestra boca se llenó de risa, *"),
						b: String::from("y nuestra lengua de gritos de alegría.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Y decían entre las naciones: *"),
						b: String::from("\"Ha hecho el Señor proezas con ellos\".")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Proezas ha hecho el Señor con nosotros, *"),
						b: String::from("y estamos sumamente alegres.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Tú, oh Señor, has cambiado nuestra suerte, *"),
						b: String::from("como los torrentes del Neguev.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Los que sembraron con lágrimas, *"),
						b: String::from("con gritos de alegría segarán.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Los que van llorando, llevando la semilla, *"),
						b: String::from("volverán entre cantares, trayendo sus gavillas.")
					}
				]
			}
		]
	};
}