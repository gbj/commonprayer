use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_21: Psalm = Psalm {
		number: 21,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 508
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, in virtute tua"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El rey se alegra en tu poder, oh Señor; *"),
						b: String::from("en tu victoria, ¡cómo se goza!")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Le has concedido el deseo de su corazón, *"),
						b: String::from("y no le negaste la petición de sus labios.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Pues le has salido al encuentro\ncon bendiciones de prosperidad; *"),
						b: String::from("corona de oro fino has puesto sobre su cabeza.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Vida te demandó, y se la diste: *"),
						b: String::from("largura de días, por los siglos de los siglos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Grande es su gloria por tu victoria, *"),
						b: String::from("honra y majestad has puesto sobre él;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Porque lo bendecirás para siempre; *"),
						b: String::from("lo llenarás de alegría con tu presencia.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Por cuanto el rey confía en el Señor, *"),
						b: String::from("y en la misericordia del Altísimo no será conmovido.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Alcanzó tu izquierda a todos tus enemigos, *"),
						b: String::from("tu diestra alcanzó a los que te aborrecen.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Los pusiste como en horno ardiente, *"),
						b: String::from("en el tiempo de tu ira, oh Señor.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los deshiciste en tu furor; *"),
						b: String::from("el fuego los consumió.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Su fruto destruiste de la tierra, *"),
						b: String::from("y su descendencia de entre los pueblos;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Porque intentaron el mal contra ti; fraguaron maquinaciones; *"),
						b: String::from("mas no prevalecían.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Pues tú los pusiste en fuga; *"),
						b: String::from("en tus cuerdas dispusiste saetas contra sus rostros.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Ensálzate, oh Señor, en tu poder; *"),
						b: String::from("cantaremos y alabaremos tu poderío.")
					}
				]
			}
		]
	};
}