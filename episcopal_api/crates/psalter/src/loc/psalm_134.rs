use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_134: Psalm = Psalm {
		number: 134,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 678
				},
				local_name: String::from(""),
				latin_name: String::from("Ecce nunc"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Y ahora bendigan al Señor, siervos todos del Señor, *"),
						b: String::from("los que de noche están de pie en la casa del Señor.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Eleven las manos hacia el santuario, y bendigan al Señor. *"),
						b: String::from("El Señor que hizo los cielos y la tierra, te bendiga desde Sión.")
					}
				]
			}
		]
	};
}