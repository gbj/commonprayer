use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_147: Psalm = Psalm {
		number: 147,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 694
				},
				local_name: String::from(""),
				latin_name: String::from("Laudate Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\n¡Cuán bueno es cantar alabanzas a nuestro Dios! *"),
						b: String::from("¡Cuán agradable es honrarle con loores!")
					},
					PsalmVerse {
						number: 2,
						a: String::from("El Señor reconstruye Jerusalén; *"),
						b: String::from("a los desterrados de Israel recoge.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("El sana a los quebrantados de corazón, *"),
						b: String::from("y venda sus heridas.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Cuenta el número de las estrellas; *"),
						b: String::from("a todas ellas llama por su nombre.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Grande es el Señor nuestro, incomparable su poder, *"),
						b: String::from("infinita su sabiduría.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El Señor levanta a los humildes, *"),
						b: String::from("mas humilla hasta el polvo a los malvados.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Canten al Señor con acción de gracias; *"),
						b: String::from("toquen el arpa a nuestro Dios.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El cubre los cielos de nubes, *"),
						b: String::from("y prepara la lluvia para la tierra;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Hace brotar la hierba en los montes, *"),
						b: String::from("y plantas verdes para la humanidad.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Da alimento a los ganados, *"),
						b: String::from("y a las crías de cuervo que graznan.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("No se deleita en el vigor del caballo, *"),
						b: String::from("ni se complace en la fortaleza del hombre.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Se complace el Señor en los que le veneran, *"),
						b: String::from("en los que confían en su gracia y favor.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Glorifica al Señor, oh Jerusalén; *"),
						b: String::from("alaba a tu Dios, oh Sión;")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Porque ha fortalecido los cerrojos de tus puertas; *"),
						b: String::from("ha bendecido a tus hijos dentro de ti.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Ha establecido la paz en tus fronteras; *"),
						b: String::from("te sacia con lo mejor del trigo.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("El envía su decreto a la tierra, *"),
						b: String::from("y su palabra corre veloz.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Despliega la nieve como lana; *"),
						b: String::from("derrama la escarcha como ceniza.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Esparce su granizo como migajas; *"),
						b: String::from("ante su frío, ¿quién resistirá?")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Envía su palabra, y se derriten; *"),
						b: String::from("sopla su viento, y corren las aguas.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Declara su palabra a Jacob, *"),
						b: String::from("sus estatutos y sus juicios a Israel.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("No ha tratado así a ninguna otra nación, *"),
						b: String::from("ni les ha dado a conocer sus mandatos.\nAleluya !")
					}
				]
			}
		]
	};
}