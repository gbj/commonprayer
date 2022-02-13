use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_50: Psalm = Psalm {
		number: 50,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 551
				},
				local_name: String::from(""),
				latin_name: String::from("Deus deorum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El Dios de dioses, el Señor, ha hablado; *"),
						b: String::from("ha convocado la tierra\ndesde el nacimiento del sol hasta donde se pone.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("De Sión, perfección de hermosura, *"),
						b: String::from("Dios ha resplandecido.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Vendrá nuestro Dios, y no callará; *"),
						b: String::from("delante de él, fuego consumidor,\na su alrededor, tempestad poderosa.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Desde lo alto convocó a los cielos y a la tierra, *"),
						b: String::from("como testigos del juicio de su pueblo.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("\"Reúnanme a mis devotos, *"),
						b: String::from("los que conmigo hicieron pacto,\ny lo sellaron con sacrificio\".")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Proclame el cielo su justicia, *"),
						b: String::from("pues Dios mismo está juzgando.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Escucha, pueblo mío, y hablaré; \"Oh Israel, testificaré contra ti; *"),
						b: String::from("yo soy Dios, el Dios tuyo.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("No te reprendo por tus sacrificios, *"),
						b: String::from("ni por tus holocaustos, que están siempre delante de mí.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("No tomaré becerros de tus corrales, *"),
						b: String::from("ni machos cabríos de tus apriscos;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Porque mía es toda bestia del bosque, *"),
						b: String::from("y míos los rebaños en los collados.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Conozco todas las aves del cielo, *"),
						b: String::from("y todo lo que se mueve en los campos está a mi vista.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Si yo tuviese hambre, no te lo diría, *"),
						b: String::from("porque mío es el mundo y toda su plenitud.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("¿He de comer yo carne de toros, *"),
						b: String::from("o de beber sangre de machos cabríos?")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Sacrifica a Dios alabanza, *"),
						b: String::from("y paga tus votos al Altísimo.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Invócame en el día de angustia; *"),
						b: String::from("yo te libraré, y tú me honrarás\".")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Pero al malvado dice Dios: *"),
						b: String::from("\"¿Por qué recitas mis leyes,\ny tomas mi pacto en tus labios,")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Tú que aborreces la corrección, *"),
						b: String::from("y arrojas a tu espalda mis palabras?")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Si ves al ladron, tú corres con él, *"),
						b: String::from("y con los adúlteros echas tu suerte.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Sueltas tu lengua para el mal, *"),
						b: String::from("y enjaeces tu boca para la mentira.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Calumnias continuamente a tu hermano, *"),
						b: String::from("y contra el hijo de tu madre lanzas infamia.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Estas cosas hiciste, y yo callé, *"),
						b: String::from("y pensaste que yo era como tú\".")
					},
					PsalmVerse {
						number: 22,
						a: String::from("\"He hecho mi acusación; *"),
						b: String::from("he puesto en orden mi causa delante de ti.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Entiendan bien esto, los que se olvidan de Dios; *"),
						b: String::from("no sea que los despedace, y no haya quien los libre.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("El que me ofrece sacrificio de alabanza, me honra; *"),
						b: String::from("pero a los que guardan mi camino\nles haré ver la salvación de Dios\".")
					}
				]
			}
		]
	};
}