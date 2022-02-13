use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_9: Psalm = Psalm {
		number: 9,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 492
				},
				local_name: String::from(""),
				latin_name: String::from("Confitebor tibi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Te daré gracias, oh Señor, con todo mi corazón; *"),
						b: String::from("contaré todas tus maravillas.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Me alegraré y me regocijaré en ti; *"),
						b: String::from("cantaré a tu Nombre, oh Altísimo.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Mis enemigos volvieron atrás; *"),
						b: String::from("cayeron y perecieron delante de ti;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque has mantenido mi derecho y mi causa; *"),
						b: String::from("te has sentado en el trono juzgando con justicia.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Reprendiste a los impíos, destruiste a los malos, *"),
						b: String::from("borraste el nombre de ellos eternamente y para siempre.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Los enemigos han perecido;\nhan quedado desolados para siempre; *"),
						b: String::from("y las ciudades que derribaste, su memoria pereció con ellas.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Pero el Señor reina para siempre; *"),
						b: String::from("ha dispuesto su trono para juicio.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El juzgará al mundo con justicia, *"),
						b: String::from("y a los pueblos con rectitud.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("El Señor será refugio de los oprimidos, *"),
						b: String::from("refugio para el tiempo de angustia.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("En ti confiarán los que conocen tu Nombre, *"),
						b: String::from("por cuanto tú, oh Señor, no desamparas a los\nque te buscan.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Canten al Señor, que habita en Sión; *"),
						b: String::from("publiquen entre los pueblos sus obras.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("El que se venga de la sangre se acordará de ellos; *"),
						b: String::from("no se olvidará del clamor de los afligidos.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Ten misericordia de mí, oh Señor; *"),
						b: String::from("mira mi aflicción que padezco a causa de los\nque me aborrecen,\ntú que me levantas de las puertas de la muerte;")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Para que cuente yo todas tus alabanzas y me goce en tu salvación, *"),
						b: String::from("en las puertas de la ciudad de Sión.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Se hundieron los impíos en el hoyo que hicieron; *"),
						b: String::from("en la red que escondieron fue tomado su pie.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("El Señor se ha hecho conocer en el juicio que ejecutó; *"),
						b: String::from("en la obra de sus manos fue enlazado el malo.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Los malos serán entregados al sepulcro, *"),
						b: String::from("todas las gentes que se olvidan de Dios;")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Porque no para siempre será olvidado el menesteroso, *"),
						b: String::from("ni la esperanza de los pobres perecerá perpetuamente.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Levántate, oh Señor; que no triunfe el impío; *"),
						b: String::from("sean juzgados los impíos delante de ti.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Pon, oh Señor, temor en ellos; *"),
						b: String::from("conozcan los impíos que no son sino mortales.")
					}
				]
			}
		]
	};
}