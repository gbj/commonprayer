use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_78: Psalm = Psalm {
		number: 78,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 590
				},
				local_name: String::from("Parte I"),
				latin_name: String::from("Attendite, popule"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Atiende, pueblo mío, mi enseñanza; *"),
						b: String::from("inclina el oído a las palabras de mi boca.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Abriré mi boca en parábolas; *"),
						b: String::from("declararé los enigmas de tiempos antiguos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Lo que hemos oído y conocido,\nlo que nuestros antepasados nos contaron, *"),
						b: String::from("no lo encubriremos de sus hijos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Contaremos a las generaciones venideras las hazañas loables del Señor, y su poder, *"),
						b: String::from("y las maravillas que ha hecho.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Entregó sus decretos a Jacob; estableció su ley en Israel, *"),
						b: String::from("y mandó que la enseñasen a sus hijos;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Para que lo supieran las generaciones siguientes y los hijos aún por nacer, *"),
						b: String::from("y para que a su vez lo contaran a sus hijos;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("A fin de que pusieran en Dios su confianza, y no se olvidaran de las obras de Dios, *"),
						b: String::from("sino que guardaran sus mandamientos;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Y no fueran como sus antepasados, generación contumaz y rebelde, *"),
						b: String::from("generación de corazón inconstante, de espíritu infiel a Dios.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Los guerreros de Efraín, provistos de arcos, *"),
						b: String::from("volvieron la espalda en el día de batalla.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("No guardaron el pacto de Dios; *"),
						b: String::from("se negaron a seguir su ley.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Se olvidaron de sus obras, *"),
						b: String::from("y de las maravillas que les había mostrado.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Hizo portentos a la vista de sus antepasados, *"),
						b: String::from("en la tierra de Egipto, en el campo de Zoán.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Dividió el mar, y los hizo pasar, *"),
						b: String::from("sujetando las aguas como muros.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Les guió de día con nube, *"),
						b: String::from("y toda la noche con resplandor de fuego.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Hendió las peñas en el desierto, *"),
						b: String::from("y les dio a beber como si fuera de grandes abismos.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Sacó de la peña, corrientes, *"),
						b: String::from("y brotaron las aguas como ríos.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Pero siguieron pecando contra él, *"),
						b: String::from("rebelándose contra el Altísimo en el desierto.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Tentaron a Dios en sus corazones, *"),
						b: String::from("exigiendo comida a su antojo.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Vituperaron a Dios y dijeron: *"),
						b: String::from("\"¿Podrá Dios preparar mesa en el desierto?")
					},
					PsalmVerse {
						number: 20,
						a: String::from("En verdad, hendió la peña,\nbrotaron aguas y los arroyos rebosaron, *"),
						b: String::from("pero, ¿podrá darnos pan, proveer de carne a su pueblo?\"")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Cuando el Señor oyó esto, se enojó; *"),
						b: String::from("se encendió un fuego contra Jacob,\nhirvió su cólera contra Israel;")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Porque no tenían fe en Dios, *"),
						b: String::from("ni confiaban en su auxilio.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Por tanto, dio orden a las altas nubes, *"),
						b: String::from("abrió las compuertas de los cielos.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("Hizo llover sobre ellos maná para que comiesen, *"),
						b: String::from("y les dio trigo de los cielos.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Así, los mortales comieron pan de ángeles; *"),
						b: String::from("les mandó comida hasta saciarles.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Hizo soplar desde el cielo el Levante, *"),
						b: String::from("y dirigió con su fuerza el viento Sur.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Hizo llover carne sobre ellos como polvo, *"),
						b: String::from("y como arena del mar, aves que vuelan.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Las hizo caer en medio del campamento *"),
						b: String::from("y alrededor de sus tiendas.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Comieron, y se saciaron, *"),
						b: String::from("porque él satisfizo su antojo.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Pero no terminó ahí su antojo, *"),
						b: String::from("aunque todavía estaba la comida en su boca.")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Por tanto, hirvió la ira de Dios contra ellos; *"),
						b: String::from("mató a los más robustos,\ny derribó a la flor de Israel.")
					},
					PsalmVerse {
						number: 32,
						a: String::from("A pesar de esto, aún pecaron, *"),
						b: String::from("y no tuvieron fe en sus proezas.")
					},
					PsalmVerse {
						number: 33,
						a: String::from("Consumió entonces sus días como un soplo, *"),
						b: String::from("y sus años en súbito terror.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("Cuando los mataba, ellos lo buscaban, *"),
						b: String::from("se arrepentían y lo buscaban de veras.")
					},
					PsalmVerse {
						number: 35,
						a: String::from("Se acordaban de que Dios era su Roca, *"),
						b: String::from("y el Dios Altísimo, su Redentor.")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Pero le lisonjeaban con su boca, *"),
						b: String::from("y con su lengua le mentían.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("Su corazón no era firme con él, *"),
						b: String::from("ni eran fieles a su pacto;")
					},
					PsalmVerse {
						number: 38,
						a: String::from("Pero él, misericordioso,\nperdonaba sus pecados y no los destruía; *"),
						b: String::from("contuvo muchas veces su ira, y no despertó su enojo;")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 593
				},
				local_name: String::from("Parte II"),
				latin_name: String::from("Quoties exacerbaverunt"),
				verses: vec![
					PsalmVerse {
						number: 39,
						a: String::from("Porque se acordó de que eran carne, *"),
						b: String::from("soplo que se va y no vuelve.")
					},
					PsalmVerse {
						number: 40,
						a: String::from("¡Cuántas veces el pueblo\nse rebeló contra Dios en el desierto, *"),
						b: String::from("y le ofendió en el yermo!")
					},
					PsalmVerse {
						number: 41,
						a: String::from("Una y otra vez tentaron a Dios, *"),
						b: String::from("y provocaron al Santo de Israel.")
					},
					PsalmVerse {
						number: 42,
						a: String::from("No se acordaron de su poder *"),
						b: String::from("el día que los rescató del enemigo,")
					},
					PsalmVerse {
						number: 43,
						a: String::from("Cuando hizo prodigios en Egipto, *"),
						b: String::from("portentos en el campo de Zoán.")
					},
					PsalmVerse {
						number: 44,
						a: String::from("Convirtió en sangre sus ríos, *"),
						b: String::from("para que no bebiesen de sus corrientes.")
					},
					PsalmVerse {
						number: 45,
						a: String::from("Envió entre ellos enjambres de moscas que los devoraban, *"),
						b: String::from("y ranas que los destruían.")
					},
					PsalmVerse {
						number: 46,
						a: String::from("Dio también a la oruga sus cosechas, *"),
						b: String::from("y a la langosta, el fruto de su labor.")
					},
					PsalmVerse {
						number: 47,
						a: String::from("Acabó sus viñas con granizo, *"),
						b: String::from("y sus sicómoros con escarcha.")
					},
					PsalmVerse {
						number: 48,
						a: String::from("Entregó al pedrisco sus ganados, *"),
						b: String::from("y sus rebaños a los rayos.")
					},
					PsalmVerse {
						number: 49,
						a: String::from("Envió sobre ellos el ardor de su ira: *"),
						b: String::from("furor, indignación y angustia,\nun tropel de ángeles destructores.")
					},
					PsalmVerse {
						number: 50,
						a: String::from("Dio rienda suelta a su furor;\nno eximió sus almas de la muerte, *"),
						b: String::from("sino que entregó sus vidas a la peste.")
					},
					PsalmVerse {
						number: 51,
						a: String::from("Hirió a los primogénitos de Egipto, *"),
						b: String::from("a las primicias de su fuerza, en las tiendas de Cam.")
					},
					PsalmVerse {
						number: 52,
						a: String::from("Sacó como ovejas a su pueblo, *"),
						b: String::from("y los guió como un rebaño por el desierto.")
					},
					PsalmVerse {
						number: 53,
						a: String::from("Los condujo seguros, y no temieron, *"),
						b: String::from("mientras el mar cubría a sus enemigos.")
					},
					PsalmVerse {
						number: 54,
						a: String::from("Los trajo a su santo monte, *"),
						b: String::from("la montaña que ganó su diestra.")
					},
					PsalmVerse {
						number: 55,
						a: String::from("Ante ellos arrojó a los cananeos, les asignó por lote su heredad, *"),
						b: String::from("e hizo habitar en sus tiendas a las tribus de Israel.")
					},
					PsalmVerse {
						number: 56,
						a: String::from("Pero ellos tentaron al Dios Altísimo y lo desafiaron; *"),
						b: String::from("no guardaron sus mandamientos.")
					},
					PsalmVerse {
						number: 57,
						a: String::from("Se desviaron y se volvieron infieles como sus padres; *"),
						b: String::from("fallaron como arco sin tensar.")
					},
					PsalmVerse {
						number: 58,
						a: String::from("Le agraviaron con sus altares paganos, *"),
						b: String::from("y le desagradaron con sus ídolos.")
					},
					PsalmVerse {
						number: 59,
						a: String::from("Cuando Dios lo oyó, se enojó, *"),
						b: String::from("y rechazó totalmente a Israel.")
					},
					PsalmVerse {
						number: 60,
						a: String::from("Abandonó su morada de Silo, *"),
						b: String::from("el tabernáculo en que había morado en medio\nde su pueblo.")
					},
					PsalmVerse {
						number: 61,
						a: String::from("Entregó a cautiverio el arca, *"),
						b: String::from("su gloria en mano del enemigo.")
					},
					PsalmVerse {
						number: 62,
						a: String::from("Entregó su pueblo a la espada, *"),
						b: String::from("y se enojó contra su heredad.")
					},
					PsalmVerse {
						number: 63,
						a: String::from("El fuego devoró a sus jóvenes, *"),
						b: String::from("y no hubo cantos nupciales para sus doncellas.")
					},
					PsalmVerse {
						number: 64,
						a: String::from("Sus sacerdotes cayeron a espada, *"),
						b: String::from("y sus viudas no hicieron lamentación.")
					},
					PsalmVerse {
						number: 65,
						a: String::from("Entonces se despertó el Señor como de un sueño, *"),
						b: String::from("como un guerrero refrescado con vino.")
					},
					PsalmVerse {
						number: 66,
						a: String::from("Hirió a sus enemigos por detrás, *"),
						b: String::from("y los avergonzó para siempre.")
					},
					PsalmVerse {
						number: 67,
						a: String::from("Rechazó las tiendas de José, *"),
						b: String::from("y no escogió la tribu de Efraín,")
					},
					PsalmVerse {
						number: 68,
						a: String::from("Sino que escogió la tribu de Judá, *"),
						b: String::from("y el Monte Sión, al cual amó.")
					},
					PsalmVerse {
						number: 69,
						a: String::from("Edificó su santuario como los cielos altos, *"),
						b: String::from("como la tierra que cimentó para siempre.")
					},
					PsalmVerse {
						number: 70,
						a: String::from("Eligió a David su siervo, *"),
						b: String::from("y lo sacó de los apriscos;")
					},
					PsalmVerse {
						number: 71,
						a: String::from("De andar tras las ovejas, lo quitó, *"),
						b: String::from("y lo hizo pastor de Jacob, su pueblo,\nde Israel, su heredad.")
					},
					PsalmVerse {
						number: 72,
						a: String::from("Con un corazón íntegro los pastoreó, *"),
						b: String::from("y los guió con la destreza de su mano.")
					}
				]
			}
		]
	};
}