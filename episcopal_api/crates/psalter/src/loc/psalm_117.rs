use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_117: Psalm = Psalm {
		number: 117,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 652
				},
				local_name: String::from(""),
				latin_name: String::from("Laudate Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Alaben al Señor, naciones todas; *"),
						b: String::from("pueblos todos, aclámenlo;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Porque grande es su misericordia para con nosotros, *"),
						b: String::from("y la fidelidad del Señor es para siempre.\n¡Aleluya!")
					}
				]
			}
		]
	};
}