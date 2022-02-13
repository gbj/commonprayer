use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_20: Psalm = Psalm {
		number: 20,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 507
				},
				local_name: String::from(""),
				latin_name: String::from("Exaudiat te Dominus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Que Dios te escuche en el día de asedio, *"),
						b: String::from("el Nombre del Dios de Jacob sea tu baluarte;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Te envíe ayuda desde su santuario, *"),
						b: String::from("y te sostenga desde Sión;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Haga memoria de todas tus ofrendas, *"),
						b: String::from("y acepte tu holocausto;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Te dé conforme al deseo de tu corazón, *"),
						b: String::from("y cumpla todos tus designios.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Nos alegraremos en tu victoria,\ny alzaremos pendón en Nombre de nuestro Dios; *"),
						b: String::from("que el Señor conceda todas tus peticiones.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Ahora sé que el Señor ha dado la victoria a su ungido; *"),
						b: String::from("lo ha escuchado desde su santo cielo\ncon la fuerza victoriosa de su diestra.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Unos confían en carros de guerra, y otros en caballos, *"),
						b: String::from("mas nosotros invocaremos el Nombre del Señor\nnuestro Dios.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Ellos se hunden y caen, *"),
						b: String::from("mas nosotros nos levantamos y estamos de pie.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Otorga victoria al rey, oh Señor, *"),
						b: String::from("y escúchanos cuando te invocamos.")
					}
				]
			}
		]
	};
}