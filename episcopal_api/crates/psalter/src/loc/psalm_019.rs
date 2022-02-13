use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_19: Psalm = Psalm {
		number: 19,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 505
				},
				local_name: String::from(""),
				latin_name: String::from("Caeli enarrant"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Los cielos proclaman la gloria de Dios, *"),
						b: String::from("y la bóveda celeste pregona las obras de sus manos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Un día emite palabra al otro día, *"),
						b: String::from("y una noche a la otra noche imparte sabiduría.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Aunque no hay palabras, ni lenguaje, *"),
						b: String::from("ni son oídas sus voces,")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Por toda la tierra salió su sonido, *"),
						b: String::from("y hasta el extremo del mundo su mensaje.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("En el mar puso tabernáculo para el sol, *"),
						b: String::from("y éste, como esposo que sale de su alcoba,\nse alegra cual paladín para correr su camino.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("De un extremo de los cielos es su salida, y su curso hasta el término de ellos; *"),
						b: String::from("nada hay que se esconda de su calor.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("La ley del Señor es perfecta,\nque aviva el alma; *"),
						b: String::from("el testimonio del Señor es fiel,\nque hace sabio al sencillo.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Los mandamientos del Señor son rectos,\nque alegran el corazón; *"),
						b: String::from("el precepto del Señor es claro,\nque alumbra los ojos.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("El temor del Señor es limpio,\nque permanece para siempre; *"),
						b: String::from("los juicios del Señor son verdad, completamente justos.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Deseables son, más que el oro,\nmás que oro fino; *"),
						b: String::from("dulce más que miel,\nque la que destila del panal.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Tu siervo es además por ellos alumbrado, *"),
						b: String::from("y al guardarlos hay grande galardón.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("¿Quién podrá entender sus propios errores? *"),
						b: String::from("Líbrame de los que me son ocultos.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Preserva también a tu siervo de las soberbias, que no se enseñoreen de mí; *"),
						b: String::from("entonces seré íntegro,\ny estaré limpio del gran pecado.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Sean gratos los dichos de mi boca\ny la meditación de mi corazón delante de ti, *"),
						b: String::from("oh Señor, Roca mía y Redentor mío.")
					}
				]
			}
		]
	};
}