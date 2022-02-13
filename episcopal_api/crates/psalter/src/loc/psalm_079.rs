use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_79: Psalm = Psalm {
		number: 79,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 596
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, venerunt"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Dios, han entrado los paganos en tu heredad; han profanado tu santo templo; *"),
						b: String::from("han reducido Jerusalén a escombros.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Han dado los cadáveres de tus siervos por comida a las aves de los cielos, *"),
						b: String::from("la carne de tus fieles a las fieras de la tierra.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Han derramado su sangre como agua en los alrededores de Jerusalén, *"),
						b: String::from("y no hubo quien los enterrase.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Somos el escarnio de nuestros vecinos, *"),
						b: String::from("la burla y mofa para los que nos rodean.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¿Hasta cuándo, oh Señor, estarás airado? *"),
						b: String::from("¿Arderá tu cólera como fuego para siempre?")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Derrama tu ira sobre los paganos que no te conocen *"),
						b: String::from("y sobre los reinos que no invocan tu Nombre;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque han devorado a Jacob, *"),
						b: String::from("y su morada han asolado.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("No te acuerdes de nuestros pecados anteriores; que tu compasión nos alcance pronto; *"),
						b: String::from("porque estamos muy abatidos.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Socórrenos, oh Dios nuestro Salvador,\npor la gloria de tu Nombre; *"),
						b: String::from("líbranos, y perdona nuestros pecados, por amor de tu Nombre.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("¿Por qué han de decir los paganos: \"Dónde está su Dios\"?*"),
						b: String::from("Que delante de nosotros sepan los paganos\nque tú vengas la sangre derramada de tus siervos.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Llegue delante de ti el gemido de los presos; *"),
						b: String::from("con tu brazo poderoso preserva a los condenados\na muerte.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("A nuestros vecinos devuélveles siete veces *"),
						b: String::from("la afrenta con que te han afrentado, oh Soberano mío.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Nosotros, pueblo tuyo, y ovejas de tu dehesa, *"),
						b: String::from("te daremos gracias para siempre,\ny proclamaremos tus alabanzas de generación en generación.")
					}
				]
			}
		]
	};
}