use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_87: Psalm = Psalm {
		number: 87,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 605
				},
				local_name: String::from(""),
				latin_name: String::from("Fundamenta ejus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("En el monte santo está la ciudad que él fundó; *"),
						b: String::from("ama el Señor las puertas de Sión\nmás que todas las moradas de Jacob.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("De ti se dicen cosas gloriosas, *"),
						b: String::from("oh ciudad de nuestro Dios.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Cuento a Egipto y a Babilonia entre los que me conocen; *"),
						b: String::from("he aquí, Filistea, Tiro y Etiopía:\nen Sión fueron nacidos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("De Sión se dirá: \"Todos han nacido en ella, *"),
						b: String::from("y el Altísimo mismo la sostendrá\".")
					},
					PsalmVerse {
						number: 5,
						a: String::from("El Señor escribirá en el registro de los pueblos: *"),
						b: String::from("\"Estos también nacieron allí\".")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Los cantores y los que danzan dirán: *"),
						b: String::from("\"Todas mis fuentes están en ti\".")
					}
				]
			}
		]
	};
}