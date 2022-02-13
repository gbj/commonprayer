use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_57: Psalm = Psalm {
		number: 57,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 560
				},
				local_name: String::from(""),
				latin_name: String::from("Miserere mei, Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Ten misericordia de mí, oh Dios, ten misericordia, porque en ti he confiado; *"),
						b: String::from("me refugiaré a la sombra de tus alas, hasta que pasen mis quebrantos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Invocaré al Dios Altísimo, *"),
						b: String::from("al Dios que me vindica.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("El responderá desde los cielos,\ny me salvará de la infamia de los que me hostigan; *"),
						b: String::from("enviará su amor y fidelidad.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Estoy entre leones que devoran a los pueblos; *"),
						b: String::from("sus dientes son lanzas y saetas,\ny su lengua espada aguda.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Red han tendido a mis pies; mi alma está abatida; *"),
						b: String::from("hoyo han cavado delante de mí, pero ellos han caído en él.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Exáltate sobre los cielos, oh Dios, *"),
						b: String::from("y tu gloria sobre toda la tierra.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Mi corazón está firme, oh Dios, mi corazón está firme; *"),
						b: String::from("tocaré y cantaré salmos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Despierta, oh alma mía; despierten, lira y arpa; *"),
						b: String::from("yo mismo despertaré al alba.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Te confesaré entre los pueblos, oh Señor; *"),
						b: String::from("cantaré tus alabanzas entre las naciones;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Porque tu gracia es más grande que los cielos, *"),
						b: String::from("y tu fidelidad alcanza hasta las nubes.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Exáltate sobre los cielos, oh Dios, *"),
						b: String::from("y tu gloria sobre toda la tierra.")
					}
				]
			}
		]
	};
}