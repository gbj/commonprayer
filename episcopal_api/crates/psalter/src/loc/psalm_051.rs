use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_51: Psalm = Psalm {
		number: 51,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 553
				},
				local_name: String::from(""),
				latin_name: String::from("Miserere mei, Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Ten misericordia de mí, oh Dios, conforme a tu bondad; *"),
						b: String::from("conforme a tu inmensa compasión borra mis rebeliones.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Lávame más y más de mi maldad, *"),
						b: String::from("y límpiame de mi pecado;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque reconozco mis rebeliones, *"),
						b: String::from("y mi pecado está siempre delante de mí.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Contra ti, contra ti sólo he pecado, *"),
						b: String::from("y he hecho lo malo delante de tus ojos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Por tanto eres reconocido justo en tu sentencia, *"),
						b: String::from("y tenido por puro en tu juicio.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("He aquí, he sido malo desde mi nacimiento, *"),
						b: String::from("pecador desde el vientre de mi madre;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque he aquí, amas la verdad más que la astucia o el saber oculto; *"),
						b: String::from("por tanto, enséñame sabiduría.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Límpiame de mi pecado, y seré puro; *"),
						b: String::from("lávame, y seré más blanco que la nieve.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Hazme oír canciones de gozo y alegría, *"),
						b: String::from("y se regocijará el cuerpo que has abatido.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Esconde tu rostro de mis pecados, *"),
						b: String::from("y borra todas mis maldades.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Crea en mí, oh Dios, un corazón limpio, *"),
						b: String::from("y renueva un espíritu firme dentro de mí.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("No me eches de tu presencia, *"),
						b: String::from("y no quites de mí tu santo Espíritu.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Dame otra vez el gozo de tu salvación; *"),
						b: String::from("y que tu noble Espíritu me sustente.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Enseñaré a los rebeldes tus caminos, *"),
						b: String::from("y los pecadores se convertirán a ti.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Líbrame de la muerte, oh Dios, *"),
						b: String::from("y cantará mi lengua tu justicia,\noh Dios mi Salvador.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Soberano mío, abre mis labios, *"),
						b: String::from("y mi boca proclamará tu alabanza;")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Porque no quieres tú sacrificio, que yo daría; *"),
						b: String::from("no te complaces en holocausto.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("El sacrificio que más te agrada es el espíritu quebrantado; *"),
						b: String::from("al corazón contrito y humillado no despreciarás tú,\noh Dios.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Haz bien con tu benevolencia a Sión; *"),
						b: String::from("reconstruye los muros de Jerusalén.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Entonces aceptarás los sacrificios requeridos, holocausto y oblación; *"),
						b: String::from("entonces ofrecerán becerros sobre tu altar.")
					}
				]
			}
		]
	};
}