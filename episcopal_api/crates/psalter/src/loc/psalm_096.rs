use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_96: Psalm = Psalm {
		number: 96,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 619
				},
				local_name: String::from(""),
				latin_name: String::from("Cantate Domino"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Canten al Señor cántico nuevo;*"),
						b: String::from("canten al Señor, toda la tierra.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Canten al Señor, bendigan su Nombre; *"),
						b: String::from("proclamen de día en día su victoria.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Pregonen entre las naciones su gloria, *"),
						b: String::from("en todos los pueblos sus maravillas;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque grande es el Señor, y muy digno de alabanza; *"),
						b: String::from("más temible es que todos los dioses;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Porque todos los dioses de los pueblos son ídolos; *"),
						b: String::from("pero es el Señor que ha hecho los cielos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("¡Oh, la majestad y la magnificencia de su presencia! *"),
						b: String::from("¡Oh, la fuerza y el esplendor de su santuario!")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Rindan al Señor, oh familias de los pueblos, *"),
						b: String::from("rindan al Señor la honra y el poder.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Rindan al Señor la gloria debida a su Nombre; *"),
						b: String::from("traigan ofrendas, y entren en sus atrios.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Adoren al Señor en la hermosura de la santidad; *"),
						b: String::from("tiemble delante de él toda la tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Pregonen entre las naciones: \"El Señor es Rey; *"),
						b: String::from("de tal manera ha afirmado el orbe que no\nserá conmovido; juzgará a los pueblos con equidad\".")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Alégrense los cielos, y gócese la tierra; truene la mar y su plenitud; *"),
						b: String::from("regocíjese el campo, y todo lo que en él está.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Entonces aclamarán con júbilo todos los árboles del bosque,\ndelante del Señor cuando llegue, *"),
						b: String::from("cuando llegue a juzgar la tierra.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Juzgará al mundo con justicia, *"),
						b: String::from("y a los pueblos con SU verdad.")
					}
				]
			}
		]
	};
}