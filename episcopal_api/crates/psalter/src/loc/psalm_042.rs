use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_42: Psalm = Psalm {
		number: 42,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 540
				},
				local_name: String::from(""),
				latin_name: String::from("Quemadmodum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Como anhela el ciervo las corrientes de aguas, *"),
						b: String::from("así te anhela, oh Dios, el alma mía.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Mi alma tiene sed de Dios, del Dios vivo; *"),
						b: String::from("¿cuándo vendré, y me presentaré delante de Dios?")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Fueron mis lágrimas mi alimento de día y de noche, *"),
						b: String::from("mientras me dicen todos los días:\n\"¿Dónde está tu Dios?\"")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Doy rienda suelta a mi dolor, cuando pienso en estas cosas: *"),
						b: String::from("de cómo fui con la multitud,\ny la conduje hasta la casa de Dios,")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Con voz de alegría y de alabanza, *"),
						b: String::from("haciendo fiesta la multitud.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("¿Por qué te abates, oh alma mía, *"),
						b: String::from("y te turbas dentro de mí?")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Pon tu confianza en Dios, *"),
						b: String::from("porque aún he de alabarle,\nSalvador, Presencia y Dios mío.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 541
				},
				local_name: String::from("desde la cima de Mizhar entre las cumbres de Hermón."),
				latin_name: String::from("desde la cima de Mizhar entre las cumbres de Hermón."),
				verses: vec![
					PsalmVerse {
						number: 8,
						a: String::from("Mi alma está abatida dentro de mí; *"),
						b: String::from("me acordaré, por tanto, de ti desde la tierra del Jordán,")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Un abismo clama a otro a la voz de tus cascadas; *"),
						b: String::from("todos tus torrentes y riadas sobre mí han pasado.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("De día otorga el Señor su gracia; *"),
						b: String::from("de noche su cántico está conmigo,\noración al Dios de mi vida.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Diré a Dios, Roca mía:\n\"¿Por qué te has olvidado de mí? *"),
						b: String::from("¿Por qué he de andar enlutado por la opresión de mis enemigos?\"")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Mientras me están quebrantando los huesos, *"),
						b: String::from("mis adversarios me afrentan.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Todo el día se burlan de mí, diciendo: *"),
						b: String::from("\"¿Dónde está tu Dios?\"")
					},
					PsalmVerse {
						number: 14,
						a: String::from("¿Por qué te abates, oh alma mía, y te turbas dentro de mí?\""),
						b: String::from("")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Pon tu confianza en Dios, *"),
						b: String::from("porque aún he de alabarle,\nSalvador, Presencia y Dios mío.")
					}
				]
			}
		]
	};
}