use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_103: Psalm = Psalm {
		number: 103,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 626
				},
				local_name: String::from(""),
				latin_name: String::from("Benedic, anima mea"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Bendice, alma mía, al Señor, *"),
						b: String::from("y todo mi ser bendiga su santo Nombre.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Bendice, alma mía, al Señor, *"),
						b: String::from("y no olvides ninguno de sus beneficios.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("El perdona todas tus iniquidades, *"),
						b: String::from("y sana todas tus dolencias.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("El rescata del sepulcro tu vida, *"),
						b: String::from("y te corona de favor y misericordia.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("El sacia de bien tus anhelos, *"),
						b: String::from("y como el águila se renueva tu juventud.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El Señor hace justicia, *"),
						b: String::from("y defiende a todos los oprimidos.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Dio a conocer sus caminos a Moisés, *"),
						b: String::from("y al pueblo de Israel sus obras.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Misericordioso y compasivo es el Señor, *"),
						b: String::from("lento para la ira y rico en clemencia.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("No nos acusará para siempre, *"),
						b: String::from("ni para siempre guardará su enojo.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("No nos ha tratado conforme a nuestros pecados, *"),
						b: String::from("ni nos ha pagado conforme a nuestras maldades.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Así como se levantan los cielos sobre la tierra, *"),
						b: String::from("así se levanta su misericordia sobre sus fieles.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Como dista el oriente del occidente, *"),
						b: String::from("así aleja de nosotros nuestras rebeliones.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Como un padre cuida de sus hijos, *"),
						b: String::from("así cuida el Señor a los que le veneran;")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Porque él sabe de qué estamos hechos; *"),
						b: String::from("se acuerda de que no somos más que barro.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Como la hierba son nuestros días; *"),
						b: String::from("florecemos como la flor del campo,")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Que pasa el viento por ella, y ya no existe, *"),
						b: String::from("y su lugar no la conocerá más;")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Empero la misericordia del Señor perdura para siempre sobre los que le veneran, *"),
						b: String::from("y su rectitud sobre los hijos de los hijos;")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Sobre los que guardan su pacto, *"),
						b: String::from("y se acuerdan de sus mandatos y los cumplen.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("El Señor estableció en los cielos su trono, *"),
						b: String::from("y su soberanía domina sobre todos.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Bendigan al Señor, ustedes sus ángeles, potestades que ejecutan sus órdenes, *"),
						b: String::from("obedeciendo a la voz de su palabra.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Bendigan al Señor, ustedes sus huestes, *"),
						b: String::from("ministros suyos que hacen su voluntad.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Bendigan al Señor, ustedes sus obras, en todos los lugares de su dominio. *"),
						b: String::from("Bendice, alma mía, al Señor.")
					}
				]
			}
		]
	};
}