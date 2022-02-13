use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_7: Psalm = Psalm {
		number: 7,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 490
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, Deus meus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor Dios mío, a ti me acojo; *"),
						b: String::from("sálvame de todos los que me persiguen, y líbrame;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("No sea que me desgarren cual león, *"),
						b: String::from("y me destrocen sin que haya quien me libre.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Oh Señor Dios mío, si yo he hecho esto: *"),
						b: String::from("si hay en mis manos iniquidad;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Si he dado mal pago a mi amigo, *"),
						b: String::from("o despojado al que sin causa era mi enemigo;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Que el enemigo me persiga y me alcance, *"),
						b: String::from("huelle en tierra mi vida, y mi honra ponga en el polvo.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Levántate, oh Señor, con tu ira; *"),
						b: String::from("álzate en contra de la furia de mis adversarios.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Despierta en favor mío el juicio que mandaste; *"),
						b: String::from("que te rodee la congregación de los pueblos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Vuélvete a sentar en tu trono sobre lo alto, *"),
						b: String::from("oh Señor, juzga a los pueblos.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Júzgame conforme a mi justicia, oh Señor, *"),
						b: String::from("y conforme a mi integridad, oh Altísimo.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Perezca ahora la maldad de los inicuos, mas establece tú al justo; *"),
						b: String::from("porque tú pruebas la mente y el corazón, oh Dios justo.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Mi escudo está en Dios, *"),
						b: String::from("que salva a los rectos de corazón.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Dios es juez justo; *"),
						b: String::from("Dios sentencia cada día.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Si no se convierten, Dios afilará su espada; *"),
						b: String::from("armado tiene su arco, y lo ha preparado.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Ha preparado armas de muerte, *"),
						b: String::from("y ha labrado saetas ardientes.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Miren: el impío concibió maldad, *"),
						b: String::from("se preñó de iniquidad y dio a luz el engaño.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Pozo ha cavado, y lo ha ahondado, *"),
						b: String::from("y en el hoyo que hizo caerá.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Su malicia volverá sobre su cabeza, *"),
						b: String::from("y su violencia caerá sobre su propia coronilla.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Yo confesaré la justicia del Señor; *"),
						b: String::from("y alabaré el Nombre del Señor Altísimo.")
					}
				]
			}
		]
	};
}