use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_99: Psalm = Psalm {
		number: 99,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 622
				},
				local_name: String::from(""),
				latin_name: String::from("Dominus regnavit"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El Señor es Rey; tiemblen los pueblos; *"),
						b: String::from("está entronizado sobre querubines;\nsacúdase la tierra.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("El Señor es grande en Sión; *"),
						b: String::from("es excelso sobre todos los pueblos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Alaben su Nombre, porque es grande y temible; *"),
						b: String::from("él es el Santo.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("\"Oh Rey poderoso, amante de la justicia, has establecido la equidad; *"),
						b: String::from("has administrado la justicia y el derecho en Jacob\".")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Proclamen la grandeza del Señor nuestro Dios, y póstrense ante el estrado de sus pies; *"),
						b: String::from("él es el Santo.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Moisés y Aarón entre sus sacerdotes,\ny Samuel entre los que invocan su Nombre, *"),
						b: String::from("invocaban al Señor, y él les respondía.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Desde la columna de nube les hablaba; *"),
						b: String::from("guardaban sus testimonios, y el decreto que les dio.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("\"Oh Señor Dios nuestro, en verdad les respondías; *"),
						b: String::from("tú eras para ellos un Dios de perdón;\ncon todo, les castigabas por sus malas obras\".")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Proclamen la grandeza del Señor nuestro Dios, y adórenle sobre su santo monte, *"),
						b: String::from("porque el Señor nuestro Dios es el Santo.")
					}
				]
			}
		]
	};
}