use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_138: Psalm = Psalm {
		number: 138,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 683
				},
				local_name: String::from(""),
				latin_name: String::from("Confitebor tibi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Te daré gracias, oh Señor, de todo corazón; *"),
						b: String::from("delante de los dioses cantaré tus alabanzas.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Me postraré hacia tu santo templo, y alabaré tu Nombre, *"),
						b: String::from("por tu amor y tu fidelidad;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque has glorificado tu Nombre, *"),
						b: String::from("y tu palabra por encima de todo.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Cuando te invoqué, me respondiste, *"),
						b: String::from("fortaleciste mi alma con vigor.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Te alabarán, oh Señor, todos los reyes de la tierra, *"),
						b: String::from("al escuchar las palabras de tu boca.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Cantarán de los caminos del Señor: *"),
						b: String::from("\"¡Cuán grande la gloria del Señor!\"")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Aunque excelso es el Señor, cuida del humilde, *"),
						b: String::from("y al altivo percibe de lejos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Aunque camine entre peligros, tú me guardas seguro; *"),
						b: String::from("contra la ira de mis enemigos extiendes tu mano,\ny tu diestra me salvará.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("El Señor cumplirá en mí su propósito; *"),
						b: String::from("tu misericordia, oh Señor, es para siempre;\nno desampares la obra de tus manos.")
					}
				]
			}
		]
	};
}