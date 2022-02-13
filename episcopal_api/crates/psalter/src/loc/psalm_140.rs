use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_140: Psalm = Psalm {
		number: 140,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 686
				},
				local_name: String::from(""),
				latin_name: String::from("Eripe me, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Líbrame, oh Señor, de los malhechores; *"),
						b: String::from("guárdame de los violentos,")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Que maquinan males en su corazón, *"),
						b: String::from("y todo el día provocan contiendas.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Han aguzado su lengua como la serpiente; *"),
						b: String::from("veneno de víboras hay en sus labios.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Guárdame, oh Señor, de manos del malvado; *"),
						b: String::from("protégeme del hombre violento,\nque está resuelto a hacerme tropezar.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Me han escondido trampas los soberbios, y han extendido una red de cuerdas; *"),
						b: String::from("por el camino me han tendido lazos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("He dicho al Señor: \"Tú eres mi Dios; *"),
						b: String::from("atiende, oh Señor, a mis súplicas.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Oh Señor Dios, fortaleza de mi salvación, *"),
						b: String::from("tú cubriste mi cabeza el día de la batalla.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("No concedas, oh Señor, al malvado sus deseos, *"),
						b: String::from("ni des éxito a sus proyectos, oh Altísimo.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Que no levanten la cabeza los que me rodean; *"),
						b: String::from("que el veneno de sus labios los anegue.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Caigan sobre ellos brasas encendidas; *"),
						b: String::from("sean echados en el cieno, de donde no salgan jamás\".")
					},
					PsalmVerse {
						number: 11,
						a: String::from("El difamador no se afirmará en la tierra; *"),
						b: String::from("al forajido lo cazará el mal.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Yo sé que el Señor protegerá la causa del afligido, *"),
						b: String::from("y defenderá el derecho del necesitado.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Ciertamente los justos alabarán tu Nombre, *"),
						b: String::from("y los rectos morarán en tu presencia.")
					}
				]
			}
		]
	};
}