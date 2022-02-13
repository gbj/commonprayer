use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_41: Psalm = Psalm {
		number: 41,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 539
				},
				local_name: String::from(""),
				latin_name: String::from("Beatus qui intelligit"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Bienaventurados los que cuidan al pobre y menesteroso; *"),
						b: String::from("en el día malo los librará el Señor.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("El Señor los guardará y los preservará en vida, para que sean dichosos en la tierra; *"),
						b: String::from("y no los entregará a la voluntad de sus enemigos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("El Señor los sostendrá en el lecho del dolor, *"),
						b: String::from("y les ministrará en su enfermedad.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Yo dije: \"Señor, ten misericordia de mí; *"),
						b: String::from("sáname, porque contra ti he pecado\".")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Mis enemigos hablan mal de mí, preguntando: *"),
						b: String::from("\"¿Cuándo morirá, y perecerá su nombre?\"")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Aun cuando vienen a verme, hablan mentiras; *"),
						b: String::from("su corazón recoge rumores falsos;\nal salir fuera los divulgan.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Reunidos murmuran contra mí todos mis enemigos; *"),
						b: String::from("contra mí idean daño, diciendo:")
					},
					PsalmVerse {
						number: 8,
						a: String::from("\"Cosa pestilente se ha apoderado de él, *"),
						b: String::from("y el que cayó en cama no volverá a levantarse\".")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Aun mi amigo íntimo, en quien yo fiaba, el que de mi pan comía, *"),
						b: String::from("alzó contra mí el calcañar, y me abandonó.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Mas tú, oh Señor, ten misericordia de mí; *"),
						b: String::from("hazme levantar, y les daré el pago.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Por ello conoceré que te he agradado, *"),
						b: String::from("que mi enemigo no se huelga de mí.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("En cuanto a mí, en mi integridad sostenme; *"),
						b: String::from("hazme estar delante de ti para siempre.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Bendito sea el Señor, el Dios de Israel, *"),
						b: String::from("por los siglos de los siglos. Amén y Amén.")
					}
				]
			}
		]
	};
}