use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_29: Psalm = Psalm {
		number: 29,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 519
				},
				local_name: String::from(""),
				latin_name: String::from("Afferte Domino"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Den al Señor, oh seres celestiales, *"),
						b: String::from("den al Señor la gloria y la fortaleza.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Den al Señor la gloria debida a su Nombre; *"),
						b: String::from("adoren al Señor en la hermosura de su santidad.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("La voz del Señor sobre las aguas; truena el Dios de gloria; *"),
						b: String::from("el Señor sobre las grandes aguas.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("La voz del Señor es voz potente; *"),
						b: String::from("la voz del Señor es voz gloriosa.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("La voz del Señor quebranta los cedros; *"),
						b: String::from("el Señor quebranta los cedros del Líbano.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Hace saltar al Líbano como becerro, *"),
						b: String::from("al Hermón como hijuelo de búfalo.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("La voz del Señor divide las llamas de fuego; la voz del Señor hace temblar el desierto; *"),
						b: String::from("hace temblar el Señor el desierto de Cades.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("La voz del Señor tuerce las encinas, *"),
						b: String::from("y desnuda los bosques.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Mientras, en el templo del Señor *"),
						b: String::from("todo proclama su gloria.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("El Señor se sienta por encima del diluvio; *"),
						b: String::from("el Señor se sienta como Rey por siempre jamás.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("El Señor dará fortaleza a su pueblo; *"),
						b: String::from("el Señor bendecirá a su pueblo con la paz.")
					}
				]
			}
		]
	};
}