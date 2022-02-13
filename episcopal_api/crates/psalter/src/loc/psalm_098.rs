use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_98: Psalm = Psalm {
		number: 98,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 621
				},
				local_name: String::from(""),
				latin_name: String::from("Cantate Domino"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Canten al Señor cántico nuevo, *"),
						b: String::from("porque ha hecho maravillas.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Con su diestra, y con su santo brazo, *"),
						b: String::from("ha alcanzado la victoria.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("El Señor ha dado a conocer su victoria; *"),
						b: String::from("a la vista de las naciones ha descubierto su justicia.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Se acuerda de su misericordia y su fidelidad para con la casa de Israel; *"),
						b: String::from("los confines de la tierra\nhan visto la victoria de nuestro Dios.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Aclamen con júbilo al Señor, pueblos todos; *"),
						b: String::from("levanten la voz, gócense y canten.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Canten al Señor con el arpa, *"),
						b: String::from("con el arpa y la voz de cántico.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Con trompetas y al son de clarines, *"),
						b: String::from("aclamen con júbilo ante el Rey, el Señor.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Ruja el mar y cuanto contiene, *"),
						b: String::from("el mundo y los que en él habitan.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Den palmadas los ríos, aclamen los montes al Señor, *"),
						b: String::from("cuando llegue para juzgar la tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Juzgará al mundo con justicia, *"),
						b: String::from("y a los pueblos con equidad.")
					}
				]
			}
		]
	};
}