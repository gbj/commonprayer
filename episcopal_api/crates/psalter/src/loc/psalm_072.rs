use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_72: Psalm = Psalm {
		number: 72,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 581
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, judicium"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Dios, da tu juicio al Rey, *"),
						b: String::from("y tu justicia al Hijo del Rey;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Para que rija a tu pueblo con justicia, *"),
						b: String::from("y a tus pobres con juicio;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Para que los montes traigan prosperidad a tu pueblo, *"),
						b: String::from("y los collados justicia.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Defenderá a los necesitados del pueblo; *"),
						b: String::from("rescatará a los pobres y aplastará al opresor.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Vivirá mientras duren el sol y la luna, *"),
						b: String::from("de generación en generación.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Descenderá como el agua sobre el campo segado, *"),
						b: String::from("como la lluvia que empapa la tierra seca.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("En aquel día florecerán los justos, *"),
						b: String::from("y habrá abundancia de paz, hasta que no haya luna.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Dominará de mar a mar, *"),
						b: String::from("y del río hasta los confines de la tierra.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Ante él se postrarán sus adversarios, *"),
						b: String::from("y sus enemigos lamerán el polvo.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los reyes de Tarsis y de las islas le pagarán tributos, *"),
						b: String::from("y los reyes de Sabá y de Arabia le ofrecerán dones.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Todos los reyes se postrarán delante de él, *"),
						b: String::from("y todas las naciones le servirán;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Porque él librará al pobre que clamare, *"),
						b: String::from("y al oprimido que no tuviere quien le socorra.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Tendrá compasión de los humildes y de los menesterosos; *"),
						b: String::from("salvará la vida de los necesitados.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("De opresión y violencia redimirá sus vidas, *"),
						b: String::from("y la sangre de ellos será preciosa a sus ojos.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Viva el Rey! Que le traigan el oro de Sabá; *"),
						b: String::from("que se ore por él continuamente,\ny lo bendigan todo el día.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Que haya abundancia de grano en la tierra, y sobrepase las cumbres de los montes; *"),
						b: String::from("florezca su fruto como el Líbano,\ny su grano como la hierba de la tierra.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Permanezca su Nombre para siempre,\ny sea perpetuado mientras dure el sol; *"),
						b: String::from("en él sean benditas todas las naciones, y lo proclamen bienaventurado.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("¡Bendito el Señor Dios, el Dios de Israel, *"),
						b: String::from("el único que hace maravillas!")
					},
					PsalmVerse {
						number: 19,
						a: String::from("¡Bendito para siempre su Nombre glorioso! *"),
						b: String::from("Toda la tierra sea llena de su gloria.\nAmén y Amén")
					}
				]
			}
		]
	};
}