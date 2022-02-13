use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_45: Psalm = Psalm {
		number: 45,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 544
				},
				local_name: String::from(""),
				latin_name: String::from("Eructavit cor meum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Me brota del corazón una canción gozosa; recitaré al rey mis versos; *"),
						b: String::from("mi lengua será pluma de buen escribano.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Eres el más bello de los hombres; *"),
						b: String::from("el hechizo se derrama de tus labios,\nporque Dios te ha bendecido desde la eternidad.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Cíñete tu espada sobre el muslo, oh valiente, *"),
						b: String::from("en tu grandeza y majestad.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Cabalga victorioso por causa de la verdad, *"),
						b: String::from("y por amor de la justicia.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Tu diestra te manifestará cosas asombrosas; *"),
						b: String::from("tus saetas son agudas, oh valeroso guerrero.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Caen los pueblos debajo de tus pies; *"),
						b: String::from("se desaniman los enemigos del rey.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Tu trono, oh Dios, es eterno y sempiterno; *"),
						b: String::from("cetro de justicia es el cetro de tu reino;\nhas amado la justicia y aborrecido la maldad.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Por ello te ha ungido Dios, el Dios tuyo, *"),
						b: String::from("con óleo de alegría, más que a tus compañeros.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Mirra, áloe y casia exhalan todos tus vestidos; *"),
						b: String::from("desde palacios de marfil los instrumentos\nde cuerda te alegran.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Hijas de reyes están entre las damas de tu corte; *"),
						b: String::from("a tu diestra está la reina, enjoyada con oro de Ofir.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("\"Oye, hija, considera e inclina tu oído: *"),
						b: String::from("Olvida tu pueblo y la casa de tu padre;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Porque el rey se deleitará en tu hermosura; *"),
						b: String::from("él es tu señor, ríndele homenaje.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("El pueblo de Tiro viene con regalos; *"),
						b: String::from("los ricos del pueblo imploran tu favor\".")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Toda gloriosa es la princesa al entrar; *"),
						b: String::from("de brocado de oro es su vestido.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Con vestidos bordados es llevada al rey; *"),
						b: String::from("en cortejo le siguen sus damas.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Con alegría y gozo son traídas, *"),
						b: String::from("y entran al palacio del rey.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("\"A cambio de padres, oh rey, tendrás hijos, *"),
						b: String::from("y los nombrarás príncipes sobre toda la tierra.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Haré perpetua la memoria de tu nombre, de generación en generación; *"),
						b: String::from("y los pueblos te alabaran por los siglos de los siglos\"")
					}
				]
			}
		]
	};
}