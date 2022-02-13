use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_17: Psalm = Psalm {
		number: 17,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 500
				},
				local_name: String::from(""),
				latin_name: String::from("Exaudi, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oye, oh Señor, mi causa justa; atiende a mi clamor; *"),
						b: String::from("escucha mi oración que no brota de labios mentirosos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("De tu presencia proceda mi vindicación; *"),
						b: String::from("vean tus ojos la rectitud.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Aunque ensayes mi corazón, visitándolo de noche, *"),
						b: String::from("aunque me sometas a pruebas de fuego,\nno encontrarás Impureza en mi.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Mi boca no hace transgresión como suelen los hombres; *"),
						b: String::from("he guardado los mandamientos de tus labios.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Me he mantenido en la senda de tu ley; *"),
						b: String::from("mis pisadas están firmes en tus senderos,\ny no vacilarán mis pasos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Yo te invoco, oh Dios, por cuanto tú me oirás; *"),
						b: String::from("inclina a mí tu oído, escucha mi palabra.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Muestra tus maravillosas misericordias, *"),
						b: String::from("tú que salvas a los que se refugian a tu diestra\nde los que se levantan contra ellos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Guárdame como a la niña de tus ojos; *"),
						b: String::from("escóndeme bajo la sombra de tus alas;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("De los malos que me asaltan, *"),
						b: String::from("de mis enemigos que buscan mi vida.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Han cerrado su corazón a la compasión, *"),
						b: String::from("con su boca hablan arrogantemente.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Han cercado ahora mis pasos; *"),
						b: String::from("tienen puestos sus ojos para echarme por tierra.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Son como león que desea hacer presa, *"),
						b: String::from("y como leoncillo que está en su escondite.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("¡Levántate, oh Señor; sal a su encuentro; póstrales! *"),
						b: String::from("Librame de los malos con tu espada.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Con tu mano, oh Señor, líbrame, *"),
						b: String::from("de aquellos cuya porción en esta vida, es el mundo;")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Cuyo vientre tú llenas de tu tesoro; *"),
						b: String::from("sacian a sus hijos,\ny aún sobra para sus pequeñuelos.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Pero yo, por mi rectitud, veré tu rostro; *"),
						b: String::from("al despertar, me saciaré de tu semejanza.")
					}
				]
			}
		]
	};
}