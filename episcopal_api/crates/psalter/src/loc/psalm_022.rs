use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_22: Psalm = Psalm {
		number: 22,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 509
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, Deus meus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dios mío, Dios mío, ¿Por qué me has desamparado? *"),
						b: String::from("¿Por qué estás lejos de mi súplica,\ny de las palabras de mi clamor?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Dios mío, clamo de día, y no respondes; *"),
						b: String::from("de noche también, y no hay para mí reposo.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Pero tú eres el Santo, *"),
						b: String::from("entronizado sobre las alabanzas de Israel.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("En ti esperaron nuestros antepasados; *"),
						b: String::from("esperaron, y tú los libraste.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Clamaron a ti, y fueron librados; *"),
						b: String::from("confiaron en ti, y no fueron avergonzados.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Mas yo soy gusano, y no hombre, *"),
						b: String::from("oprobio de todos y desprecio del pueblo.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Todos los que me ven, escarnecen de mí; *"),
						b: String::from("estiran los labios y menean la cabeza, diciendo:")
					},
					PsalmVerse {
						number: 8,
						a: String::from("\"Acudió al Señor, líbrele él; *"),
						b: String::from("sálvele, si tanto lo quiere\".")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Pero tú eres el que me sacó del vientre, *"),
						b: String::from("y me tenías confiado en los pechos de mi madre.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("A ti fui entregado antes de nacer, *"),
						b: String::from("desde el vientre de mi madre, tú eres mi Dios.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("No te alejes de mí, porque la angustia está cerca, *"),
						b: String::from("porque no hay quien ayude.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Me rodean muchos novillos; *"),
						b: String::from("fuertes toros de Basán me circundan.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Abren sobre mí las bocas, *"),
						b: String::from("como león rapante y rugiente.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Soy derramado como aguas;\ntodos mis huesos se descoyuntan; *"),
						b: String::from("mi corazón, como cera, se derrite en mis entrañas.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Como un tiesto está seca mi boca; mi lengua se pega al paladar; *"),
						b: String::from("y me has puesto en el polvo de la muerte;")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Porque jaurías de perros me rodean, y pandillas de malignos me cercan; *"),
						b: String::from("horadan mis manos y mis pies; contar puedo todos mis huesos.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Me miran de hito en hito, y con satisfacción maligna; *"),
						b: String::from("reparten entre sí mis vestidos;\nsobre mi ropa echan suertes.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Mas tú, oh Señor, no te alejes; *"),
						b: String::from("fortaleza mía, apresúrate a socorrerme.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Salva de la espada mi garganta, *"),
						b: String::from("mi faz del filo del hacha.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Sálvame de la boca del león, *"),
						b: String::from("a este pobre, de los cuernos del búfalo.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Proclamaré tu Nombre a mis hermanos; *"),
						b: String::from("en medio de la congregación te alabaré.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Los que temen al Señor, alábenle; *"),
						b: String::from("glorifíquenle, oh vástago de Jacob;\ntengan miedo de él, oh descendencia de Israel;")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Porque no menospreció ni abominó la aflicción de los afligidos,\nni de ellos escondió su rostro; *"),
						b: String::from("sino que cuando clamaron a él, los oyó.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("De ti será mi alabanza en la gran congregación; *"),
						b: String::from("mis votos pagaré delante de los que le temen.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Comerán los pobres, y serán saciados, alabarán al Señor los que le buscan: *"),
						b: String::from("¡Viva su corazón para siempre!")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Se acordarán y se volverán al Señor todos los confines de la tierra, *"),
						b: String::from("y todas las familias de las naciones delante de ti se inclinan")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Porque del Señor es el reino, *"),
						b: String::from("y él rige las naciones.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Sólo ante él se postrarán los que duermen en la tierra; *"),
						b: String::from("delante de él doblarán la rodilla\ntodos los que bajan al polvo.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Me hará vivir para él;\nmi descendencia le servirá; *"),
						b: String::from("será contada como suya para siempre.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Vendrán y anunciarán al pueblo aún no nacido *"),
						b: String::from("los hechos asombrosos que hizo.")
					}
				]
			}
		]
	};
}