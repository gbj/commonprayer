use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_144: Psalm = Psalm {
		number: 144,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 690
				},
				local_name: String::from(""),
				latin_name: String::from("Benedictus Dominus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Bendito el Señor, roca mía! *"),
						b: String::from("El adiestra mis manos para el combate,\ny mis dedos para la pelea;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Mi auxilio y mi fortaleza,\nmi refugio y mi libertador, *"),
						b: String::from("mi escudo en quien confío,\nque somete los pueblos a mi dominio.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Oh Señor, ¿qué somos,\npara que de nosotros cuides? *"),
						b: String::from("¿Que somos los mortales,\npara que en nosotros pienses?")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Somos igual que un soplo, *"),
						b: String::from("y nuestros días como la sombra que pasa.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Oh Señor, inclina tus cielos, y desciende; *"),
						b: String::from("toca los montes, y echarán humo.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Lanza los relámpagos, y dispérsalos; *"),
						b: String::from("tira tus saetas, y ponlos en fuga.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Extiende tu mano desde las alturas; *"),
						b: String::from("rescátame, y líbrame de las grandes aguas,\nde la mano de pueblos extranjeros,")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Cuya boca habla mentiras, *"),
						b: String::from("y cuya diestra jura en falso.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Oh Dios, a ti cantaré cántico nuevo; *"),
						b: String::from("tañeré para ti con lira de diez cuerdas.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Tú das victoria a los reyes, *"),
						b: String::from("y has rescatado a David tu siervo.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Rescátame de la espada que hiere, *"),
						b: String::from("y líbrame de la mano de pueblos extranjeros,")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Cuya boca habla mentiras, *"),
						b: String::from("y cuya diestra jura en falso.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Sean nuestros hijos como plantas bien criadas desde su juventud, *"),
						b: String::from("y nuestras hijas como las esquinas labradas de un palacio.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Que rebosen nuestros graneros\nde toda suerte de cosechas; *"),
						b: String::from("se acrecienten por millares\nlos ganados en nuestras praderas;\nestén nuestros bueyes bien nutridos.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Que no haya brechas en las murallas, ni deportación, *"),
						b: String::from("ni lamento en nuestras plazas.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("¡Dichoso el pueblo que goza de todo esto! *"),
						b: String::from("¡Dichoso el pueblo cuyo Dios es el Señor")
					}
				]
			}
		]
	};
}