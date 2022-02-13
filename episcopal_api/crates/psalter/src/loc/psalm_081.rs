use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_81: Psalm = Psalm {
		number: 81,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 598
				},
				local_name: String::from(""),
				latin_name: String::from("Exultate Deo"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Canten con gozo a Dios, fortaleza nuestra, *"),
						b: String::from("al Dios de Jacob aclamen con júbilo.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Entonen canción, y tañan el pandero, *"),
						b: String::from("la lira templada y el arpa.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Toquen el corno en la luna nueva, *"),
						b: String::from("y en la luna llena, que es el día de nuestra fiesta;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque estatuto es de Israel, *"),
						b: String::from("ordenanza del Dios de Jacob.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Lo estableció como mandato solemne para José, *"),
						b: String::from("al salir del país de Egipto.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("\"Oí la voz de un desconocido; *"),
						b: String::from("retiré la carga de sus hombros;\nsus manos fueron libradas de los cestos\".")
					},
					PsalmVerse {
						number: 7,
						a: String::from("En tu angustia clamaste, y yo te salvé; *"),
						b: String::from("te respondí desde lo secreto del trueno;\nte probé junto a las aguas de Meribá.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Escucha, pueblo mío, y te amonestaré, *"),
						b: String::from("¡Ojalá me escuchases, oh Israel!")
					},
					PsalmVerse {
						number: 9,
						a: String::from("No habrá entre ustedes dios ajeno; *"),
						b: String::from("no adorarás un dios extranjero.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Yo soy el Señor tu Dios,\nque te saqué del país de Egipto; *"),
						b: String::from("y dije: \"Abre tu boca, y yo la llenaré\";")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Pero mi pueblo no escuchó mi voz, *"),
						b: String::from("e Israel no quiso obedecerme.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Lo entregué, por tanto, a la dureza de su corazón, *"),
						b: String::from("para que anduviese según su antojo.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Ojalá me escuchase mi pueblo, *"),
						b: String::from("y que Israel caminase por mis caminos!")
					},
					PsalmVerse {
						number: 14,
						a: String::from("En un momento sometería a sus enemigos, *"),
						b: String::from("y volvería mi mano contra sus adversarios.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Los que aborrecen al Señor se humillarían ante él, *"),
						b: String::from("y su condenación quedaría sellada para siempre;")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Pero yo a Israel alimentaría con el mejor trigo, *"),
						b: String::from("y con la miel de la peña le saciaría.")
					}
				]
			}
		]
	};
}