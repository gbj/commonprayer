use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_116: Psalm = Psalm {
		number: 116,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 650
				},
				local_name: String::from(""),
				latin_name: String::from("Dilexi, quoniam"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Amo al Señor, pues ha oído mi voz y mi súplica; *"),
						b: String::from("porque ha inclinado a mí su oído,\nsiempre que le invoco.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Ligaduras de muerte me enredaron;\nme alcanzaron las garras de la tumba; *"),
						b: String::from("hallé angustia y dolor.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Entonces invoqué el Nombre del Señor: *"),
						b: String::from("\"Oh Señor, dígnate salvar mi vida\".")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Clemente es el Señor y justo; *"),
						b: String::from("sí, misericordioso es nuestro Dios.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("El Señor guarda a los inocentes; *"),
						b: String::from("estaba yo postrado, y me salvó.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Vuelve, oh alma mía, a tu reposo; *"),
						b: String::from("porque el Señor te ha hecho bien;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Pues tú has librado mi vida de la muerte, *"),
						b: String::from("mis ojos de lágrimas\ny mis pies de la caída.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Caminaré en la presencia del Señor, *"),
						b: String::from("en el país de los vivientes.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Tenía fe, aun cuando dije:\n\"Estoy afligido en gran manera\". *"),
						b: String::from("En mi angustia dije: \"En nadie se puede fiar\".")
					},
					PsalmVerse {
						number: 10,
						a: String::from("¿Cómo pagaré al Señor *"),
						b: String::from("por todos sus beneficios para conmigo?")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Alzaré la copa de la salvación, *"),
						b: String::from("e invocaré el Nombre del Señor.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Pagaré mis votos al Señor *"),
						b: String::from("delante de todo su pueblo.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Preciosa a los ojos del Señor, *"),
						b: String::from("es la muerte de sus siervos.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Oh Señor, yo soy tu siervo;\nsiervo tuyo soy, hijo de tu sierva; *"),
						b: String::from("me has librado de mis prisiones.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Te ofreceré el sacrificio de alabanza, *"),
						b: String::from("e invocaré el Nombre del Señor.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Pagaré mis votos al Señor *"),
						b: String::from("delante de todo su pueblo,")
					},
					PsalmVerse {
						number: 17,
						a: String::from("En los atrios de la casa del Señor, *"),
						b: String::from("en medio de ti, oh Jerusalén.\n¡Aleluya!")
					}
				]
			}
		]
	};
}