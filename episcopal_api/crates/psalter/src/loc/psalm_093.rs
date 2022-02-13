use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_93: Psalm = Psalm {
		number: 93,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 615
				},
				local_name: String::from(""),
				latin_name: String::from("Dominus regnavit"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El Señor es Rey; se ha vestido de esplendor; *"),
						b: String::from("el Señor se ha vestido y ceñido de poder.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("De tal manera afirmó el orbe, *"),
						b: String::from("que no se le puede mover.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Firme es tu trono desde siempre; *"),
						b: String::from("tú eres eternamente.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Alzaron las aguas, oh Señor, las aguas alzaron su voz; *"),
						b: String::from("alzaron sus ondas aplastantes.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Más potente que la voz de muchas aguas, más potente que los rompientes del mar, *"),
						b: String::from("más potente es el Señor en las alturas.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Tus testimonios son muy firmes; *"),
						b: String::from("la santidad es el adorno de tu casa, oh Señor,\npor los siglos y para siempre.")
					}
				]
			}
		]
	};
}