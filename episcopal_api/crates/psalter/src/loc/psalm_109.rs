use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_109: Psalm = Psalm {
		number: 109,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 643
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, laudem"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Dios de mi alabanza, no calles; *"),
						b: String::from("porque la boca del malvado, la boca del engañador,\nse ha abierto contra mí.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Me hablan con lengua mentirosa, *"),
						b: String::from("me rodean con palabras de odio,\nme combaten sin causa.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("A pesar de mi amor, me acusan; *"),
						b: String::from("en cuanto a mí, yo oro por ellos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Me devuelven mal por bien, *"),
						b: String::from("y odio por amor.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Pon contra él un malvado, *"),
						b: String::from("y que un acusador esté a su diestra.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Cuando fuere juzgado, salga culpable, *"),
						b: String::from("y sea su apelación rehusada.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Sean pocos sus días, *"),
						b: String::from("y tome otro su oficio.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Sean huérfanos sus hijos, *"),
						b: String::from("y su mujer viuda.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Que sus hijos sean abandonados y mendiguen; *"),
						b: String::from("sean echados de las ruinas de sus hogares.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Que el acreedor se apodere de todo lo que tiene, *"),
						b: String::from("y extranjeros saqueen sus ganancias.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Que nadie le muestre clemencia, *"),
						b: String::from("y ninguno se compadezca de sus huérfanos.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Que su posteridad sea exterminada, *"),
						b: String::from("y borrado su apellido en la siguiente generación.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Que la maldad de sus padres se recuerde ante el Señor, *"),
						b: String::from("y el pecado de su madre no sea borrado;")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Que su pecado esté siempre presente delante del Señor; *"),
						b: String::from("mas su memoria arranque de la tierra;")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Porque no se acordó de hacer misericordia, *"),
						b: String::from("sino persiguió al pobre y menesteroso,\ny al atribulado buscó para darle muerte.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Amó la maldición: recaiga sobre él; *"),
						b: String::from("despreció la bendición: que se aparte de él.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Se vistió de maldición como de un traje; *"),
						b: String::from("que le cale como agua hasta las entrañas,\ny como aceite hasta los huesos.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Séale como el manto con que se envuelve, *"),
						b: String::from("como el cinturón que lo ciñe siempre.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Sea éste el pago del Señor a los que me acusan, *"),
						b: String::from("a los que me calumnian.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Pero tú, oh Señor mi Dios,\nfavoréceme por amor de tu Nombre: *"),
						b: String::from("líbrame por la ternura de tu bondad;")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Porque soy pobre y menesteroso, *"),
						b: String::from("y mi corazón está herido dentro de mí.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Desaparezco como la sombra cuando se alarga, *"),
						b: String::from("me sacuden como a la langosta.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Mis rodillas están debilitadas por no comer, *"),
						b: String::from("estoy flaco y descarnado.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("He llegado a ser oprobio para ellos; *"),
						b: String::from("cuando me ven, menean la cabeza.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Ayúdame, oh Señor mi Dios; *"),
						b: String::from("sálvame por tu misericordia.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Reconozcan que ésta es tu mano, *"),
						b: String::from("que eres tú, oh Señor, quien lo ha hecho.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Podrán maldecir, pero tú bendecirás; *"),
						b: String::from("que sean avergonzados los que se levantan contra mí,\ny se regocijará tu siervo.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Sean vestidos de infamia los que me acusan, *"),
						b: String::from("sean envueltos de vergüenza como con un manto.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Yo daré gracias al Señor con voz potente; *"),
						b: String::from("en medio de la muchedumbre le alabaré;")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Porque él se pone a la diestra del pobre, *"),
						b: String::from("para salvar la vida de los que le condenarían.")
					}
				]
			}
		]
	};
}