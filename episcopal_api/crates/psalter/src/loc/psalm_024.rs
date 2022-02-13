use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_24: Psalm = Psalm {
		number: 24,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 512
				},
				local_name: String::from(""),
				latin_name: String::from("Domini est terra"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Del Señor es la tierra y su plenitud, *"),
						b: String::from("el mundo y los que en él habitan;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Porque él la fundó sobre los mares, *"),
						b: String::from("y la afirmó sobre los ríos del abismo.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("\"¿Quién subirá al monte del Señor? *"),
						b: String::from("Y ¿quién estará en su santo lugar?\"")
					},
					PsalmVerse {
						number: 4,
						a: String::from("\"El limpio de manos, y puro de corazón, *"),
						b: String::from("el que no ha elevado su mente a un ídolo,\nni jurado por dios falso.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Recibirá bendición del Señor, *"),
						b: String::from("y recompensa merecida del Dios de su salvación\".")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Tal es la generación de los que le buscan, *"),
						b: String::from("de los que buscan tu rostro, oh Dios de Jacob.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Alcen, oh puertas, sus cabezas; álcense, oh puertas del Eterno; *"),
						b: String::from("y entrará el Rey de gloria.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("\"¿Quién es este Rey de gloria?\" *"),
						b: String::from("\"El Señor, fuerte y valiente,\nel Señor, poderoso en batalla\".")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Alcen, oh puertas, sus cabezas; álcense, oh puertas del Eterno; *"),
						b: String::from("y entrará el Rey de gloria.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("\"¿Quién es él, el Rey de gloria?\" *"),
						b: String::from("\"El Señor de las huestes,\nél es el Rey de gloria\".")
					}
				]
			}
		]
	};
}