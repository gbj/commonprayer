use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_58: Psalm = Psalm {
		number: 58,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 561
				},
				local_name: String::from(""),
				latin_name: String::from("si vere utique"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh gobernantes, en verdad, ¿dictan sentencias justas? *"),
						b: String::from("¿Hacen verdadera justicia?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("No, traman maldad en su corazón; *"),
						b: String::from("y sus manos reparten violencia en la tierra.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Los malvados se pervierten desde el vientre; *"),
						b: String::from("los mentirosos se extravían desde que nacen.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Son venenosos como serpiente; *"),
						b: String::from("son como el áspid sordo que cierra su oído,")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Para no oír la voz del encantador, *"),
						b: String::from("por más hábil que éste sea.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Oh Dios, rómpeles los dientes en la boca; *"),
						b: String::from("arráncales los colmillos a los leones, oh Señor.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Que se disipen como agua que se escurre; *"),
						b: String::from("que se marchiten como hierba pisoteada.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Que se deslía como el caracol en su baba; *"),
						b: String::from("como abortado que no llega a ver el sol.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Antes de que den fruto, sean cortados como la zarza; *"),
						b: String::from("como cardos y ortigas sean barridos.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Se alegrarán los justos cuando vean la venganza; *"),
						b: String::from("lavarán sus pies en la sangre de los malvados.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Entonces dirá la gente:\n\"Ciertamente para el justo hay galardón; *"),
						b: String::from("ciertamente hay un Dios que gobierna en la tierra\".")
					}
				]
			}
		]
	};
}