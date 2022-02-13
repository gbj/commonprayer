use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_33: Psalm = Psalm {
		number: 33,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 524
				},
				local_name: String::from(""),
				latin_name: String::from("Exultate, justi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Alégrense, justos, en el Señor; *"),
						b: String::from("a los rectos es conveniente la alabanza,")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Celebren al Señor con arpa; *"),
						b: String::from("táñanle con salterio y decacordio.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Cántenle canción nueva; *"),
						b: String::from("toquen la trompeta con destreza;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque recta es la palabra del Señor, *"),
						b: String::from("y toda su obra es hecha con fidelidad.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("El ama justicia y juicio; *"),
						b: String::from("de la misericordia del Señor está llena la tierra")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Por la palabra del Señor fueron hechos los cielos, *"),
						b: String::from("y el ejército de los cielos por el aliento de su boca.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("El junta como en un odre las aguas de la mar; *"),
						b: String::from("él pone en depósitos los abismos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Tema al Señor toda la tierra; *"),
						b: String::from("teman delante de él todos los habitantes del mundo")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Porque él dijo, y fue hecho; *"),
						b: String::from("él mandó, y existió.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("El Señor hace nula la voluntad de las gentes, *"),
						b: String::from("y frustra las maquinaciones de los pueblos.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Pero la voluntad del Señor permanece para siempre, *"),
						b: String::from("los designios de su corazón por todas las generaciones.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Bienaventurada la nación cuyo Dios es el Señor; *"),
						b: String::from("bienaventurado el pueblo que él escogió para sí.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Desde el cielo mira el Señor, *"),
						b: String::from("y ve a todos los seres humanos.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Desde el lugar de su morada observa *"),
						b: String::from("a todos los moradores de la tierra.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("El formó el corazón de todos ellos; *"),
						b: String::from("atento está a todas sus obras.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("El rey no se salva por la multitud del ejército, *"),
						b: String::from("ni escapa el valiente por la mucha fuerza.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Vano para salvar es el caballo; *"),
						b: String::from("la grandeza de su fuerza a nadie podrá librar.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("He aquí el ojo del Señor sobre los que le temen, *"),
						b: String::from("sobre los que esperan en su misericordia;")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Para arrancar sus vidas de la muerte, *"),
						b: String::from("y para sustentarles en tiempo de hambre.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Nuestra alma espera al Señor; *"),
						b: String::from("nuestra ayuda y nuestro escudo es él.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Por tanto en él se alegra nuestro corazón, *"),
						b: String::from("porque en su santo Nombre confiamos.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Sea tu misericordia, oh Señor, sobre nosotros, *"),
						b: String::from("según ponemos nuestra confianza en ti.")
					}
				]
			}
		]
	};
}