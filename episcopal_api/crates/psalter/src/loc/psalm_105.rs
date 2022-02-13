use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_105: Psalm = Psalm {
		number: 105,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 631
				},
				local_name: String::from("Parte I"),
				latin_name: String::from("Confitemini Domino"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Den gracias al Señor, invoquen su Nombre; *"),
						b: String::from("den a conocer sus hazañas entre los pueblos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Cántenle, cántenle alabanzas; *"),
						b: String::from("hablen de todas sus obras maravillosas.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Gloríense en su santo Nombre; *"),
						b: String::from("alégrese el corazón de los que buscan al Señor.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Busquen al Señor y su poder; *"),
						b: String::from("busquen continuamente su rostro.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Acuérdense de las maravillas que él ha hecho, *"),
						b: String::from("de los prodigios y de los juicios de su boca,")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Oh vástago de Abrahán, su siervo, *"),
						b: String::from("oh hijos de Jacob, su escogido.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("El es el Señor nuestro Dios; *"),
						b: String::from("por todo el mundo prevalecen sus juicios.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Se acuerda eternamente de su pacto, *"),
						b: String::from("la promesa que hizo para mil generaciones:")
					},
					PsalmVerse {
						number: 9,
						a: String::from("El pacto que hizo con Abrahán, *"),
						b: String::from("el juramento que juró a Isaac,")
					},
					PsalmVerse {
						number: 10,
						a: String::from("El cual estableció como ley para Jacob, *"),
						b: String::from("para Israel como pacto sempiterno,")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Diciendo: \"A ti te daré la tierra de Canaán, *"),
						b: String::from("como porción de tu heredad\".")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Cuando ellos eran pocos en número, *"),
						b: String::from("sin importancia, y forasteros en la tierra,")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Errantes de nación en nación, *"),
						b: String::from("de un reino a otro,")
					},
					PsalmVerse {
						number: 14,
						a: String::from("No permitió que nadie los oprimiese, *"),
						b: String::from("y por amor a ellos castigó reyes,")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Diciendo: \"No toquen a mi ungido, *"),
						b: String::from("no hagan daño a mis profetas\".")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Entonces trajo hambre sobre la tierra, *"),
						b: String::from("cortando el sustento de pan.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Envió un varón delante de ellos, *"),
						b: String::from("a José, que fue vendido como esclavo.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Le trabaron los pies con grillos; *"),
						b: String::from("le pusieron argolla en el cuello.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Hasta la hora en que se cumplió su predicción, *"),
						b: String::from("la palabra del Señor le probó.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Mandó el rey, y le soltó; *"),
						b: String::from("el soberano de los pueblos lo libertó.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Lo puso por dueño de su casa, *"),
						b: String::from("por administrador de todas sus posesiones,")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 632
				},
				local_name: String::from("Parte II"),
				latin_name: String::from("Et intravit Israel"),
				verses: vec![
					PsalmVerse {
						number: 22,
						a: String::from("Para que instruyera a sus príncipes según su voluntad, *"),
						b: String::from("y a sus ancianos enseñara sabiduría.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Israel entró en Egipto, *"),
						b: String::from("y Jacob se hospedó en la tierra de Cam.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("El Señor hizo a su pueblo sumamente fecundo; *"),
						b: String::from("lo hizo más fuerte que sus enemigos.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Les cambió el corazón para que aborreciesen a su pueblo, y trataron injustamente a sus siervos."),
						b: String::from("")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Envió a Moisés, su siervo, *"),
						b: String::from("y a Aarón, al cual escogió.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Hicieron contra ellos las señales de Dios, *"),
						b: String::from("y sus prodigios en la tierra de Cam.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Envió tinieblas, y oscureció, *"),
						b: String::from("pero los egipcios se rebelaron contra sus palabras.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Volvió sus aguas en sangre, *"),
						b: String::from("e hizo morir sus peces.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Su tierra se infestó de ranas, *"),
						b: String::from("hasta en las cámaras de sus reyes.")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Habló, y vinieron enjambres de moscas, *"),
						b: String::from("piojos por todo su territorio.")
					},
					PsalmVerse {
						number: 32,
						a: String::from("Les dio granizo por lluvia, *"),
						b: String::from("y llamas de fuego en toda la tierra.")
					},
					PsalmVerse {
						number: 33,
						a: String::from("Asoló sus viñas y sus higueras, *"),
						b: String::from("y destrozó todos los árboles del país.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("Habló, y vinieron langostas, *"),
						b: String::from("y saltamontes sin número;")
					},
					PsalmVerse {
						number: 35,
						a: String::from("Comieron toda la hierba de su país, *"),
						b: String::from("y devoraron el fruto de sus campos.")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Hirió de muerte a los primogénitos de su tierra, *"),
						b: String::from("a las primicias de todo su vigor.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("Sacó a su pueblo con plata y oro; *"),
						b: String::from("entre sus tribus nadie tropezaba.")
					},
					PsalmVerse {
						number: 38,
						a: String::from("Egipto se alegró de su éxodo, *"),
						b: String::from("porque pavor cayó sobre ellos.")
					},
					PsalmVerse {
						number: 39,
						a: String::from("Puso el Señor una nube por cubierta, *"),
						b: String::from("y fuego para alumbrar la noche.")
					},
					PsalmVerse {
						number: 40,
						a: String::from("Pidieron, e hizo venir codornices; *"),
						b: String::from("los sació de pan del cielo.")
					},
					PsalmVerse {
						number: 41,
						a: String::from("Abrió la peña, y fluyeron aguas; *"),
						b: String::from("corrieron como un río por los sequedales.")
					},
					PsalmVerse {
						number: 42,
						a: String::from("Se acordó de su santo pacto, *"),
						b: String::from("y de Abrahán, su siervo.")
					},
					PsalmVerse {
						number: 43,
						a: String::from("Así sacó a su pueblo con gozo, *"),
						b: String::from("con júbilo a sus escogidos.")
					},
					PsalmVerse {
						number: 44,
						a: String::from("Les dio las tierras de las naciones, *"),
						b: String::from("y el fruto del trabajo de otros pueblos,")
					},
					PsalmVerse {
						number: 45,
						a: String::from("Para que guardasen sus estatutos *"),
						b: String::from("y cumpliesen sus leyes.\n¡Aleluya!")
					}
				]
			}
		]
	};
}