use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_137: Psalm = Psalm {
		number: 137,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 682
				},
				local_name: String::from(""),
				latin_name: String::from("Super flumina"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Junto a los ríos de Babilonia, allí nos sentamos a llorar, *"),
						b: String::from("al acordarnos de ti, oh Sión.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Sobre los álamos, en medio de ella, *"),
						b: String::from("colgamos nuestras arpas;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque los que nos llevaron cautivos pedían una canción; nuestros opresores pedían alegría: *"),
						b: String::from("\"Cántennos un cántico de Sión\".")
					},
					PsalmVerse {
						number: 4,
						a: String::from("¿Cómo cantaremos cántico del Señor *"),
						b: String::from("en tierra extranjera?")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Si me olvidare de ti, oh Jerusalén, *"),
						b: String::from("pierda mi diestra su destreza.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Que se me pegue la lengua al paladar, Si no me acordare de ti, *"),
						b: String::from("Si no pusiere a Jerusalén\npor encima de mi suma alegría.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Acuérdate del día de Jerusalén, oh Señor, en contra de los edomitas, *"),
						b: String::from("que decían: \"¡Arrásenla, arrásenla hasta los cimientos!\"")
					},
					PsalmVerse {
						number: 8,
						a: String::from("¡Oh hija de Babilonia, asoladora, *"),
						b: String::from("dichoso el que te dé el pago\nde lo que tú nos hiciste!")
					},
					PsalmVerse {
						number: 9,
						a: String::from("¡Dichoso el que tome tus niños *"),
						b: String::from("y los estrelle contra la peña!")
					}
				]
			}
		]
	};
}