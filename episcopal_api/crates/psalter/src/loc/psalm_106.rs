use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_106: Psalm = Psalm {
		number: 106,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 634
				},
				local_name: String::from("Parte I"),
				latin_name: String::from("Confitemini Domino"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nDen gracias al Señor, porque es bueno, *"),
						b: String::from("porque para siempre es su misericordia.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("¿Quién puede declarar las poderosas obras del Señor? *"),
						b: String::from("¿Quién puede contar sus alabanzas?")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Dichosos los que respetan el derecho, *"),
						b: String::from("y actúan siempre con justicia.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Acuérdate de mí, oh Señor,\ncon el favor que muestras para tu pueblo; *"),
						b: String::from("visítame con tu salvación;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Para que yo vea la prosperidad de tus escogidos, y me alegre con la alegría de tu pueblo, *"),
						b: String::from("y me gloríe con tu heredad.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Hemos pecado como nuestros antepasados; *"),
						b: String::from("hemos hecho lo malo y cometimos iniquidades.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("En Egipto no percibieron tus maravillas,\nni se acordaron de tu abundante misericordia; *"),
						b: String::from("se rebelaron contra el Altísimo junto al Mar Rojo;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Pero él los salvó por amor de su Nombre, *"),
						b: String::from("para manifestar su poder.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Reprendió al Mar Rojo y lo secó; *"),
						b: String::from("los condujo por el abismo como por un desierto.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los salvó de mano del enemigo, *"),
						b: String::from("y los rescató de mano del adversario.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Cubrieron las aguas a sus opresores; *"),
						b: String::from("no quedó ni uno de ellos.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Entonces creyeron sus palabras, *"),
						b: String::from("y cantaron sus alabanzas.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Bien pronto olvidaron sus obras, *"),
						b: String::from("y no aguardaron su consejo.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Ardían de avidez en el desierto, *"),
						b: String::from("y tentaron a Dios en el yermo.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("El les dio lo que pidieron, *"),
						b: String::from("pero les envió flaqueza de alma.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Envidiaron a Moisés en el campamento, *"),
						b: String::from("y a Aarón, el consagrado del Señor.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Se abrió la tierra y se tragó a Datán, *"),
						b: String::from("y cubrió a la pandilla de Abiram.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 636
				},
				local_name: String::from("Parte II"),
				latin_name: String::from("Et fecerunt vitulum"),
				verses: vec![
					PsalmVerse {
						number: 18,
						a: String::from("Un fuego abrasó a su banda, *"),
						b: String::from("una llama consumió a los malvados.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("En Horeb hizo Israel un becerro, *"),
						b: String::from("y adoró una imagen de metal fundido.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Así cambiaron su Gloria *"),
						b: String::from("por la imagen de un buey que come hierba.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Se olvidaron de Dios su Salvador, *"),
						b: String::from("que había hecho prodigios en Egipto,")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Maravillas en el país de Cam, *"),
						b: String::from("cosas temibles junto al Mar Rojo.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Determinó Dios destruirlos,\nde no haberse interpuesto Moisés, su escogido, *"),
						b: String::from("a fin de apartar su indignación, para que no los consumiese.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("Despreciaron la tierra deseable, *"),
						b: String::from("y no creyeron en sus promesas.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Murmuraron en sus tiendas, *"),
						b: String::from("y no escucharon la voz del Señor.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Por tanto alzó la mano contra ellos, *"),
						b: String::from("para abatirlos en el desierto,")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Para arrojar su estirpe entre las naciones, *"),
						b: String::from("y esparcirlos por todos los pueblos.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Se unieron a Baal-Peor, *"),
						b: String::from("y comieron los sacrificios ofrecidos a los muertos.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Provocaron la ira de Dios con sus acciones, *"),
						b: String::from("y entre ellos brotó una plaga.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Entonces se levantó Finees e intercedió, *"),
						b: String::from("y se acabó la plaga.")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Esto le fue contado por rectitud, *"),
						b: String::from("de generación en generación para siempre.")
					},
					PsalmVerse {
						number: 32,
						a: String::from("También le enojaron junto a las aguas de Meribá, *"),
						b: String::from("de modo que castigó a Moisés por causa de ellos;")
					},
					PsalmVerse {
						number: 33,
						a: String::from("Porque de tal manera amargaron su espíritu, *"),
						b: String::from("que habló palabras imprudentes con sus labios.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("No exterminaron a los pueblos, *"),
						b: String::from("como el Señor les había mandado;")
					},
					PsalmVerse {
						number: 35,
						a: String::from("Sino que se mezclaron con los paganos, *"),
						b: String::from("y aprendieron sus costumbres idólatras;")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Así adoraron sus ídolos, *"),
						b: String::from("que se convirtieron en trampa para ellos.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("Ofrecieron sus hijos y sus hijas *"),
						b: String::from("en sacrificio a los demonios.")
					},
					PsalmVerse {
						number: 38,
						a: String::from("Derramaron sangre inocente,\nla sangre de sus hijos y de sus hijas, *"),
						b: String::from("que ofrecieron a los ídolos de Canaán, y la tierra fue contaminada de sangre.")
					},
					PsalmVerse {
						number: 39,
						a: String::from("Así se mancharon con sus acciones, *"),
						b: String::from("y se prostituyeron con sus hechos malos.")
					},
					PsalmVerse {
						number: 40,
						a: String::from("Por tanto, se encendió la ira del Señor sobre su pueblo, *"),
						b: String::from("y aborreció su heredad.")
					},
					PsalmVerse {
						number: 41,
						a: String::from("Los entregó en poder de los paganos, *"),
						b: String::from("y los que les odiaban se enseñorearon de ellos.")
					},
					PsalmVerse {
						number: 42,
						a: String::from("Sus enemigos los oprimieron, *"),
						b: String::from("y fueron humillados bajo su mano.")
					},
					PsalmVerse {
						number: 43,
						a: String::from("¡Cuántas veces los libró!\nMas ellos se rebelaron por su propio consejo, *"),
						b: String::from("y fueron abatidos por su iniquidad.")
					},
					PsalmVerse {
						number: 44,
						a: String::from("Con todo, él miró su angustia *"),
						b: String::from("al escuchar sus lamentaciones.")
					},
					PsalmVerse {
						number: 45,
						a: String::from("Se acordó de su pacto con ellos, *"),
						b: String::from("y se aplacó conforme a su gran misericordia.")
					},
					PsalmVerse {
						number: 46,
						a: String::from("Hizo que les tuvieran compasión *"),
						b: String::from("los que les tenían cautivos.")
					},
					PsalmVerse {
						number: 47,
						a: String::from("Sálvanos, oh Señor nuestro Dios,\ny recógenos de entre las naciones, *"),
						b: String::from("para que demos gracias a tu santo Nombre y nos gloriemos en tus alabanzas.")
					},
					PsalmVerse {
						number: 48,
						a: String::from("¡Bendito el Señor Dios de Israel, desde siempre y para siempre! *"),
						b: String::from("Y diga todo el pueblo: \"Amén\". ¡Aleluya!")
					}
				]
			}
		]
	};
}