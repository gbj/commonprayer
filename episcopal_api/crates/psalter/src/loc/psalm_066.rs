use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_66: Psalm = Psalm {
		number: 66,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 569
				},
				local_name: String::from(""),
				latin_name: String::from("Jubilate Deo"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Aclamen a Dios, toda la tierra; *"),
						b: String::from("canten la gloria de su Nombre; canten la gloria de su alabanza.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Digan a Dios: \"¡Cuán asombrosas tus obras! *"),
						b: String::from("Por la grandeza de tu poder se someten a ti\ntus enemigos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Toda la tierra te adora; *"),
						b: String::from("te canta, canta tu Nombre\".")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Vengan, y vean las obras de Dios, *"),
						b: String::from("¡cuán temibles sus proezas para el género humano!")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Convirtió el mar en tierra seca,\npara que atravesaran el agua a pie; *"),
						b: String::from("y allí nos alegramos en él.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("En su poder él se enseñorea eternamente; sus ojos atalayan sobre las naciones; *"),
						b: String::from("que no se subleven los rebeldes.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Bendigan, pueblos, a nuestro Dios; *"),
						b: String::from("hagan oír la voz de su alabanza.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El es quien preserva a nuestra alma en vida; *"),
						b: String::from("y no permite que nuestros pies resbalen;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Porque tú, oh Dios, nos probaste; *"),
						b: String::from("nos refinaste como refinan la plata.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Nos metiste en la red; *"),
						b: String::from("pusiste sobre nuestros lomos pesada carga.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Hiciste cabalgar enemigos sobre nuestra cabeza; atravesamos por fuego y agua; *"),
						b: String::from("pero nos sacaste a un lugar de abundancia.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Entraré a tu casa con holocaustos, y te pagaré mis votos, *"),
						b: String::from("que pronunciaron mis labios,\ny habló mi boca, cuando estaba angustiado.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Te ofreceré holocaustos de animales cebados, con sahumerios de carneros; *"),
						b: String::from("inmolaré bueyes y cabros.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Vengan, oigan, cuantos temen a Dios, *"),
						b: String::from("y les contaré lo que ha hecho conmigo.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("A él clamé con mi boca, *"),
						b: String::from("y lo ensalzó mi lengua.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Si yo tuviese maldad en mi corazón, *"),
						b: String::from("mi Soberano no me habría escuchado;")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Mas ciertamente me escuchó Dios, *"),
						b: String::from("y atendió a la voz de mi súplica.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Bendito sea Dios, que no rechazó mi oración, *"),
						b: String::from("ni me retiró su favor.")
					}
				]
			}
		]
	};
}