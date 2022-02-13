use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_139: Psalm = Psalm {
		number: 139,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 684
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, probasti"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, tú me has probado y conocido; *"),
						b: String::from("conoces mi sentarme y mi levantarme;\npercibes de lejos mis pensamientos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Observas mis viajes y mis lugares de reposo, *"),
						b: String::from("y todos mis caminos te son conocidos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Aún no está la palabra en mis labios, *"),
						b: String::from("y he aquí, oh Señor, tú la conoces.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Me rodeas delante y detrás, *"),
						b: String::from("y sobre mí pones tu mano.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Tal conocimiento es demasiado maravilloso para mí; *"),
						b: String::from("sublime es, y no lo puedo alcanzar.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("¿A dónde huiré de tu Espíritu? *"),
						b: String::from("¿A dónde huiré de tu presencia?")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Si subiere a los cielos, allí estás tú; *"),
						b: String::from("si en el abismo hiciere mi lecho, allí estás también.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Si tomare las alas del alba, *"),
						b: String::from("y habitare en el extremo del mar,")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Aun allí me guiará tu mano, *"),
						b: String::from("y me asirá tu diestra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Si dijere: \"Ciertamente las tinieblas me encubrirán, *"),
						b: String::from("y aun la luz se hará noche alrededor de mí\",")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Las tinieblas no son oscuras para ti; la noche resplandece como el día; *"),
						b: String::from("lo mismo te son las tinieblas que la luz;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Porque tú creaste mis entrañas; *"),
						b: String::from("me tejiste en el vientre de mi madre.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Te daré gracias, porque maravillosamente he sido *"),
						b: String::from("formado; admirables son tus obras, y bien lo sé.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("No fue encubierto de ti mi cuerpo, mientras que en oculto era formado, *"),
						b: String::from("y entretejido en lo más profundo de la tierra.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Tus ojos vieron mis miembros,\naún incompletos en el vientre;\ntodos estaban escritos en tu libro; *"),
						b: String::from("contados estaban mis días, antes que llegase el primero.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("¡Cuán profundos me son, oh Dios, tus pensamientos *"),
						b: String::from("¡Cuán inmensa es la suma de ellos!")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Si los contase, serían más que la arena; *"),
						b: String::from("para contarlos todos, tendría que ser eterno como tú.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("¡Oh Dios, ojalá matares al malvado! *"),
						b: String::from("¡Apártense de mí, oh sanguinarios!")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Blasfemias dicen contra ti; *"),
						b: String::from("tus enemigos toman tu Nombre en vano.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("¿No odio, oh Señor, a los que te odian? *"),
						b: String::from("¿No abomino a los que se levantan contra ti?")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Los aborrezco con odio extremo; *"),
						b: String::from("los tengo por mis enemigos.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Escudríñame, oh Dios, y conoce mi corazón; *"),
						b: String::from("pruébame, y conoce mis inquietudes.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Ve si hay en mí camino de perversidad, *"),
						b: String::from("y guíame en el camino eterno.")
					}
				]
			}
		]
	};
}