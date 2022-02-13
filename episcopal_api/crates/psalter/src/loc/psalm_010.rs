use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_10: Psalm = Psalm {
		number: 10,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 494
				},
				local_name: String::from(""),
				latin_name: String::from("Ut quid, Domine?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¿Por qué estás tan lejos, oh Señor, *"),
						b: String::from("y te escondes en el tiempo de la tribulación?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Con arrogancia el malo persigue al pobre; *"),
						b: String::from("será atrapado en las trampas que ha ideado;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque el malo se jacta del deseo de su corazón; *"),
						b: String::from("en su codicia blasfema y desprecia al Señor.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("El malo, por la altivez de su rostro, no tiene cuidado; *"),
						b: String::from("no hay Dios en ninguno de sus pensamientos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Sus caminos son torcidos en todo tiempo; tus juicios los tiene muy lejos de su vista; *"),
						b: String::from("a todos sus adversarios desafía.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Dice en su corazón: \"No seré movido jamás; *"),
						b: String::from("nunca me alcanzará el infortunio\".")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Llena está su boca de maldición, de engaños y de fraude; *"),
						b: String::from("debajo de su lengua hay vejación y maldad.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Se sienta al acecho en los rincones de las plazas; en escondrijos mata al inocente; *"),
						b: String::from("sus ojos espían al desvalido.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Acecha en oculto, como el león desde su cueva; acecha para arrebatar al humilde; *"),
						b: String::from("arrebata al humilde trayéndolo a su red.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Se encoje, se agacha, *"),
						b: String::from("y caen en sus fuertes garras muchos desdichados.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Dice el malo en su corazón: \"Dios ha olvidado: *"),
						b: String::from("ha encubierto su rostro; nunca lo verá\".")
					},
					PsalmVerse {
						number: 12,
						a: String::from("¡Levántate, oh Señor; alza tu mano, oh Dios; *"),
						b: String::from("no te olvides de los afligidos!")
					},
					PsalmVerse {
						number: 13,
						a: String::from("¿Por qué desprecia el malo a Dios? *"),
						b: String::from("¿Por qué dice en su corazón: \"Tú no le pedirás cuentas?\"")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Tú lo has visto; porque miras el trabajo y la vejación, *"),
						b: String::from("para dar la recompensa con tu mano.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("A ti se acoje el desvalido; *"),
						b: String::from("tú eres el amparo del huérfano.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Quebranta tú el poder del inicuo, *"),
						b: String::from("y persigue la maldad del malo,\nhasta que no halles ninguna.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("El Señor es Rey eternamente y para siempre; *"),
						b: String::from("de su tierra perecerán los impíos.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("El deseo de los humildes seguramente escucharás, oh Señor; *"),
						b: String::from("tú animas su corazón, y haces atento tu oído,")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Para dar justicia al huérfano y al oprimido, *"),
						b: String::from("a fin de que el terrígeno no vuelva a sembrar su terror.")
					}
				]
			}
		]
	};
}