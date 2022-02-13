use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_131: Psalm = Psalm {
		number: 131,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 676
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, non est"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, mi corazón no es arrogante, *"),
						b: String::from("ni mis ojos engreídos;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("No me ocupo de cosas grandes, *"),
						b: String::from("ni de las que superan mi capacidad;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Acallo mi alma y la sosiego,\ncomo un niño en brazos de su madre; *"),
						b: String::from("mi alma está calmada dentro de mí.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Oh Israel, aguarda al Señor, *"),
						b: String::from("desde ahora y para siempre.")
					}
				]
			}
		]
	};
}