use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_111: Psalm = Psalm {
		number: 111,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 646
				},
				local_name: String::from(""),
				latin_name: String::from("Confitebor tibi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nDaré gracias al Señor de todo corazón, *"),
						b: String::from("en la asamblea de los rectos, en la congregación.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("¡Grandes son las obras del Señor! *"),
						b: String::from("Son dignas de estudio para los que las aman.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Su obra está llena de esplendor y majestad, *"),
						b: String::from("y su benevolencia permanece para siempre.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Ha hecho memorables sus maravillas; *"),
						b: String::from("clemente y compasivo es el Señor.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Da alimento a los que le veneran; *"),
						b: String::from("para siempre se acuerda de su pacto.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El poder de sus obras manifestó a su pueblo, *"),
						b: String::from("dándoles la heredad de las naciones.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Las obras de sus manos son verdad y juicio; *"),
						b: String::from("fidedignos son todos sus mandamientos,")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Afirmados eternamente y para siempre, *"),
						b: String::from("hechos en verdad y en rectitud.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Redención envió a su pueblo; para siempre ordenó su pacto; *"),
						b: String::from("santo y temible es su Nombre.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Principio de la sabiduría es el temor del Señor; tienen buen juicio los que lo practican; *"),
						b: String::from("su loor permanece para siempre.")
					}
				]
			}
		]
	};
}