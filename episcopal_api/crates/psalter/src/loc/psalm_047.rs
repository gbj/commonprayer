use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_47: Psalm = Psalm {
		number: 47,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 547
				},
				local_name: String::from(""),
				latin_name: String::from("Omnes gentes, plaudite"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Aplaudan, pueblos todos; *"),
						b: String::from("aclamen a Dios con voz de júbilo;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Porque el Señor Altísimo es temible, *"),
						b: String::from("Rey grande sobre toda la tierra.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Somete a los pueblos a nuestro dominio, *"),
						b: String::from("y sujeta a las naciones bajo nuestros pies.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Nos elige nuestra heredad, *"),
						b: String::from("el deleite de Jacob, a quien ama.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Ascendió Dios entre gritos de júbilo, *"),
						b: String::from("el Señor con sonido de trompeta.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Canten alabanzas a Dios, canten; *"),
						b: String::from("canten alabanzas a nuestro Rey, canten;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque Dios es Rey de toda la tierra; *"),
						b: String::from("canten alabanzas con esmero.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Dios reina sobre las naciones; *"),
						b: String::from("se sienta sobre su santo trono.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Los nobles de los pueblos se han unido *"),
						b: String::from("al pueblo del Dios de Abrahán.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los soberanos de la tierra pertenecen a Dios, *"),
						b: String::from("y él es excelso.")
					}
				]
			}
		]
	};
}