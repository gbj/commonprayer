use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_77: Psalm = Psalm {
		number: 77,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 588
				},
				local_name: String::from(""),
				latin_name: String::from("Voce mea ad Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Con mi voz clamo a Dios; *"),
						b: String::from("a Dios clamo, y él me escuchará.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("A mi Soberano busqué en el día de mi angustia; *"),
						b: String::from("alzaba a él mis manos de noche, sin descanso;\nrehusé ser consolado.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Cuando pienso en Dios, estoy inquieto; *"),
						b: String::from("medito, y mi espíritu desfallece.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("No me dejas pegar los ojos; *"),
						b: String::from("estoy turbado, y no puedo hablar.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Considero los días antiguos; *"),
						b: String::from("recuerdo los años remotos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Converso con mi corazón de noche; *"),
						b: String::from("medito, y escudriño mi espíritu.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("¿Es que mi Soberano me rechazará para siempre, *"),
						b: String::from("y ya no volverá a favorecerme?")
					},
					PsalmVerse {
						number: 8,
						a: String::from("¿Ha cesado para siempre su misericordia? *"),
						b: String::from("¿Se ha acabado perpetuamente su promesa?")
					},
					PsalmVerse {
						number: 9,
						a: String::from("¿Ha olvidado Dios tener compasión? *"),
						b: String::from("¿Ha encerrado con ira sus piedades?")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Me dije: \"¡Qué pena la mía! *"),
						b: String::from("¡Ha perdido su poder la diestra del Altísimo!\"")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Me acordaré de las obras del Señor; *"),
						b: String::from("haré memoria de tus maravillas antiguas.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Meditaré en todas tus obras, *"),
						b: String::from("y consideraré tus hazañas.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Oh Dios, santo es tu camino. *"),
						b: String::from("¿Qué dios es tan grande como nuestro Dios?")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Tú eres el Dios que hace maravillas; *"),
						b: String::from("hiciste conocer a los pueblos tu poder.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Con tu brazo redimiste a tu pueblo, *"),
						b: String::from("a los hijos de Jacob y de José.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Te vieron las aguas, oh Dios;\nlas aguas te vieron, y temblaron; *"),
						b: String::from("aun los abismos se estremecieron.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Las nubes derramaron sus aguas; tronaron los cielos; *"),
						b: String::from("tus saetas destellaron de un lado a otro.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("El sonido de tu trueno estaba en el torbellino; tus relámpagos alumbraron el mundo; *"),
						b: String::from("se estremeció y tembló la tierra.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("En el mar fue tu camino,\ny tus sendas en las aguas profundas, *"),
						b: String::from("pero tus pisadas no fueron vistas.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Condujiste a tu pueblo como a un rebaño, *"),
						b: String::from("por mano de Moisés y Aarón.")
					}
				]
			}
		]
	};
}