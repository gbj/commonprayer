use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_100: Psalm = Psalm {
		number: 100,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 622
				},
				local_name: String::from(""),
				latin_name: String::from("Jubilate Deo"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Regocíjense en el Señor, pueblos todos; *"),
						b: String::from("sirvan al Señor con alegría;\nvengan ante su presencia con cánticos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Sepan que el Señor es Dios; *"),
						b: String::from("él nos hizo y somos suyos,\nsu pueblo y ovejas de su rebaño.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Entren por sus puertas con acción de gracias, en sus atrios con alabanza; *"),
						b: String::from("denle gracias, y bendigan su Nombre;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque el Señor es bueno;\npara siempre es su misericordia; *"),
						b: String::from("su fidelidad perdura de generación en generación.")
					}
				]
			}
		]
	};
}