use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_49: Psalm = Psalm {
		number: 49,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 549
				},
				local_name: String::from(""),
				latin_name: String::from("Audite haec, omnes"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oigan esto, pueblos todos;\nescuchen, habitantes todos del mundo, *"),
						b: String::from("así los plebeyos como los nobles, el rico y el pobre juntamente.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Mi boca hablará sabiduría, *"),
						b: String::from("y el pensamiento de mi corazón, inteligencia.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Inclinaré mi oído al proverbio; *"),
						b: String::from("manifestaré mi secreto al son del arpa.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("¿Por qué he de temer en los días de adversidad, *"),
						b: String::from("cuando la iniquidad de mis insidiadores me cercare,")
					},
					PsalmVerse {
						number: 5,
						a: String::from("La iniquidad de los que confían en sus bienes, *"),
						b: String::from("y se jactan de sus muchas riquezas?")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Nadie puede redimirse a sí mismo, *"),
						b: String::from("ni pagar a Dios su propio rescate;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque el precio de nuestra redención es tan grande, *"),
						b: String::from("que nunca tendríamos suficiente para pagarlo,")
					},
					PsalmVerse {
						number: 8,
						a: String::from("A fin de vivir para siempre, *"),
						b: String::from("y nunca ver la sepultura.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Vemos que también los sabios mueren; perecen como el insensato y el necio, *"),
						b: String::from("y dejan a otros sus riquezas.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("El sepulcro será su habitación eterna,\nsu morada de generación en generación, *"),
						b: String::from("aunque hayan dado su nombre a sus tierras.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Aunque hayan recibido honra,\npueden vivir para siempre; *"),
						b: String::from("son como las bestias que perecen.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Tal es el camino de los que tontamente confían en sí mismos, *"),
						b: String::from("el fin de los que se complacen en sus propias palabras.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Como un rebaño de ovejas, son destinados a morir; la muerte es su pastor; *"),
						b: String::from("bajan directamente a la tumba.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Se desvanecerá su figura, *"),
						b: String::from("y en el Reino de los Muertos habitarán para siempre.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Empero Dios rescatará mi vida; *"),
						b: String::from("me arrebatará de las garras de la muerte.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("No envidies al que se enriquece, *"),
						b: String::from("y aumenta el lujo de su casa;")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Porque cuando muera no se llevará nada, *"),
						b: String::from("ni su lujo le seguirá.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Aunque mientras vivía, se sobreestimaba, *"),
						b: String::from("y era loado por su éxito,")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Se unirá a la generación de sus padres, *"),
						b: String::from("y nunca más verá la luz.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("El que recibe honra y no entiende, *"),
						b: String::from("es como las bestias que perecen.")
					}
				]
			}
		]
	};
}