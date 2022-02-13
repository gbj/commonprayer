use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_95: Psalm = Psalm {
		number: 95,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 618
				},
				local_name: String::from(""),
				latin_name: String::from("Venite, exultemus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Vengan, cantemos alegremente al Señor; *"),
						b: String::from("aclamemos con júbilo a la Roca que nos salva.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Lleguemos ante su presencia con alabanza, *"),
						b: String::from("vitoreándole con cánticos;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque el Señor es Dios grande, *"),
						b: String::from("y Rey grande sobre todos los dioses.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("En su mano están las profundidades de la tierra, *"),
						b: String::from("y las alturas de los montes son suyas.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Suyo el mar, pues él lo hizo, *"),
						b: String::from("y sus manos formaron la tierra seca.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Vengan, adoremos y postrémonos; *"),
						b: String::from("arrodillémonos delante del Señor nuestro Hacedor;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque él es nuestro Dios;\nnosotros el pueblo de su dehesa, y ovejas de su mano. *"),
						b: String::from("¡Ojalá escuchen hoy su voz!")
					},
					PsalmVerse {
						number: 8,
						a: String::from("No endurezcan su corazón,\ncomo en Meribá, y en el día de Masá en el desierto, *"),
						b: String::from("donde me tentaron sus antepasados.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Me pusieron a prueba, *"),
						b: String::from("aunque habían visto mis obras.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Durante cuarenta años aborrecí aquella generación, y dije: *"),
						b: String::from("\"Es un pueblo que divaga de corazón; no reconoce mis caminos\".")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Por tanto, juré en mi furor: *"),
						b: String::from("\"No entrarán en mi reposo\".")
					}
				]
			}
		]
	};
}