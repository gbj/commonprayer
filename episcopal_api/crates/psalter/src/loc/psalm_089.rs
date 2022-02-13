use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_89: Psalm = Psalm {
		number: 89,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 607
				},
				local_name: String::from("Parte I"),
				latin_name: String::from("Misericordias Domini"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Tu amor, oh Señor, cantaré perpetuamente; *"),
						b: String::from("de generación en generación\nanunciará mi boca tu fidelidad;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Porque seguro estoy que tu amor es para siempre; *"),
						b: String::from("en los cielos has afirmado tu fidelidad.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("\"Hice pacto con mi escogido; *"),
						b: String::from("juré a David mi siervo, diciendo:")
					},
					PsalmVerse {
						number: 4,
						a: String::from("'Para siempre confirmaré tu linaje, *"),
						b: String::from("y edificaré tu trono por todas las generaciones'\".")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Proclaman los cielos tus maravillas, oh Señor, *"),
						b: String::from("y tu fidelidad, en la asamblea de los seres celestiales;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Porque ¿quién en los cielos se compara al Señor? *"),
						b: String::from("¿Quién como el Señor entre los dioses?")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Dios es temido en el consejo de los seres celestiales, *"),
						b: String::from("grande y terrible para cuantos le rodean.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Oh Señor Dios de los Ejércitos, ¿quién como tú? *"),
						b: String::from("Dios poderoso, tu fidelidad te rodea.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Dominas la braveza del mar, *"),
						b: String::from("y sosiegas el furor de las olas.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Tú quebrantaste a Rahab con herida de muerte;\ncon tu brazo poderoso esparciste a tus enemigos."),
						b: String::from("")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Tuyos son los cielos, tuya también la tierra; *"),
						b: String::from("el mundo y su plenitud, tú lo fundaste.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 608
				},
				local_name: String::from("el Tabor y el Hermón cantan con júbilo en tu Nombre."),
				latin_name: String::from("el Tabor y el Hermón cantan con júbilo en tu Nombre."),
				verses: vec![
					PsalmVerse {
						number: 12,
						a: String::from("El norte y el sur, tú los creaste; *"),
						b: String::from("")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Tienes un brazo poderoso; *"),
						b: String::from("fuerte es tu izquierda y alta tu diestra.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Rectitud y justicia son el cimiento de tu trono; *"),
						b: String::from("amor y fidelidad van delante de tu rostro.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("¡Dichoso el pueblo que sabe la aclamación festiva! *"),
						b: String::from("Camina, oh Señor, a la luz de tu rostro.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("En tu Nombre se regocija todo el día, *"),
						b: String::from("y en tu justicia es jubiloso;")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Porque tú eres la gloria de su fortaleza, *"),
						b: String::from("y con tu favor se acrecienta nuestro poder.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 609
				},
				local_name: String::from("Parte II"),
				latin_name: String::from("Tunc locutus es"),
				verses: vec![
					PsalmVerse {
						number: 18,
						a: String::from("Ciertamente, el Señor es nuestro Soberano, *"),
						b: String::from("y nuestro Rey es el Santo de Israel.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Hablaste una vez a tus fieles en una visión, y dijiste: *"),
						b: String::from("\"He puesto la corona sobre un héroe;\nhe levantado a un escogido del pueblo.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Hallé a David mi siervo; *"),
						b: String::from("lo ungí con mi óleo sagrado.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Mi mano estará siempre con él; *"),
						b: String::from("mi brazo también lo fortalecerá.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("No lo engañará ningún enemigo, *"),
						b: String::from("ni cualquier malvado lo humillará.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Aplastaré delante de él a sus enemigos *"),
						b: String::from("y heriré a los que le aborrecen.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("Mi amor y fidelidad lo acompañarán, *"),
						b: String::from("y por mi Nombre será victorioso.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Pondré su izquierda sobre el mar, *"),
						b: String::from("y su diestra sobre el río.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("El me invocará: 'Tú eres mi Padre, *"),
						b: String::from("mi Dios, y la roca de mi salvación'.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Yo le pondré por primogénito, *"),
						b: String::from("el más excelso de los reyes de la tierra.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Sostendré mi amor por él para siempre, *"),
						b: String::from("y mi pacto continuará firme con él.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Afirmaré su descendencia para siempre, *"),
						b: String::from("y su trono como los días de los cielos\".")
					},
					PsalmVerse {
						number: 30,
						a: String::from("\"Si abandonaren sus hijos mi ley, *"),
						b: String::from("y no anduvieren de acuerdo con mis juicios;")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Si profanaren mis estatutos, *"),
						b: String::from("y no guardaren mis mandamientos;")
					},
					PsalmVerse {
						number: 32,
						a: String::from("Entonces castigaré con vara su rebelión, *"),
						b: String::from("y con azotes sus iniquidades;")
					},
					PsalmVerse {
						number: 33,
						a: String::from("Mas no retiraré de él mi amor, *"),
						b: String::from("ni falsearé mi fidelidad.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("No violaré mi pacto, *"),
						b: String::from("ni cambiaré lo que ha salido de mis labios.")
					},
					PsalmVerse {
						number: 35,
						a: String::from("De una vez por todas he jurado por mi santidad: *"),
						b: String::from("'No mentiré a David.")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Su linaje perdurará para siempre, *"),
						b: String::from("y su trono como el sol delante de mí.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("Será firme para siempre como la luna, *"),
						b: String::from("testigo fiel en los cielos'\".")
					},
					PsalmVerse {
						number: 38,
						a: String::from("Pero te has airado con tu ungido; *"),
						b: String::from("lo has rechazado y desechado.")
					},
					PsalmVerse {
						number: 39,
						a: String::from("Has roto el pacto con tu siervo; *"),
						b: String::from("has profanado su corona, arrojándola al suelo.")
					},
					PsalmVerse {
						number: 40,
						a: String::from("Has abierto brecha en sus murallas, *"),
						b: String::from("y has derribado sus fortalezas.")
					},
					PsalmVerse {
						number: 41,
						a: String::from("Lo saquean todos los que pasan por el camino; *"),
						b: String::from("es escarnio de sus vecinos.")
					},
					PsalmVerse {
						number: 42,
						a: String::from("Has exaltado la diestra de sus enemigos; *"),
						b: String::from("has alegrado a sus adversarios.")
					},
					PsalmVerse {
						number: 43,
						a: String::from("Has desviado el filo de su espada, *"),
						b: String::from("y no lo has sostenido en la batalla.")
					},
					PsalmVerse {
						number: 44,
						a: String::from("Has acabado con su esplendor, *"),
						b: String::from("y has derribado su trono por tierra.")
					},
					PsalmVerse {
						number: 45,
						a: String::from("Has acortado los días de su juventud, *"),
						b: String::from("y lo has cubierto de ignominia.")
					},
					PsalmVerse {
						number: 46,
						a: String::from("¿Hasta cuándo, oh Señor, te esconderás? ¿Te esconderás para siempre? *"),
						b: String::from("¿Hasta cuándo arderá tu ira como el fuego?")
					},
					PsalmVerse {
						number: 47,
						a: String::from("Recuerda, oh Señor, cuán breve es la vida, cuán frágil has creado toda carne."),
						b: String::from("")
					},
					PsalmVerse {
						number: 48,
						a: String::from("¿Quién vivirá y no verá la muerte? *"),
						b: String::from("¿Quién podrá salvar su vida del poder de la fosa?")
					},
					PsalmVerse {
						number: 49,
						a: String::from("Oh Señor, ¿dónde están tus antiguas misericordias, *"),
						b: String::from("que juraste a David en tu fidelidad?")
					},
					PsalmVerse {
						number: 50,
						a: String::from("Acuérdate, oh Señor, del escarnio de tu siervo, *"),
						b: String::from("de las mofas de muchos pueblos, que llevo en mi seno,")
					},
					PsalmVerse {
						number: 51,
						a: String::from("Las mofas que tus enemigos, oh Señor, han lanzado, *"),
						b: String::from("las que lanzaron a las huellas de tu ungido.")
					},
					PsalmVerse {
						number: 52,
						a: String::from("¡Bendito el Señor por siempre jamás! *"),
						b: String::from("Amén y Amén.\nDía Decimoctavo: oración Matutina")
					}
				]
			}
		]
	};
}