use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_132: Psalm = Psalm {
		number: 132,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 676
				},
				local_name: String::from(""),
				latin_name: String::from("Memento, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Acuérdate, oh Señor, de David, *"),
						b: String::from("y de todas sus aflicciones;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("De cómo juró al Señor, *"),
						b: String::from("e hizo voto al Poderoso de Jacob:")
					},
					PsalmVerse {
						number: 3,
						a: String::from("\"No entraré bajo el techo de mi casa, *"),
						b: String::from("ni subiré a mi lecho;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("No daré sueño a mis ojos, *"),
						b: String::from("ni a mis párpados adormecimiento;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Hasta que halle un lugar para el Señor, *"),
						b: String::from("una morada para el Poderoso de Jacob\".")
					},
					PsalmVerse {
						number: 6,
						a: String::from("\"¡El arca! Oímos que estaba en Efrata, *"),
						b: String::from("la hallamos en el campo de Jaar.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Vayamos a la habitación de Dios; *"),
						b: String::from("postrémonos ante el estrado de sus pies\".")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Levántate, oh Señor, al lugar de tu reposo, *"),
						b: String::from("tú, y el arca de tu poder.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Que se vistan tus sacerdotes de justicia, *"),
						b: String::from("que tus fieles canten de júbilo.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Por amor a David tu siervo, *"),
						b: String::from("no vuelvas el rostro de tu Ungido.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("El Señor ha jurado a David un juramento, *"),
						b: String::from("y seguramente no se retractará:")
					},
					PsalmVerse {
						number: 12,
						a: String::from("\"A uno de los hijos de tu cuerpo *"),
						b: String::from("pondré sobre tu trono.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Si tus hijos guardaren mi pacto,\ny mis testimonios que yo les enseñaré, *"),
						b: String::from("sus hijos también se sentarán sobre tu trono para siempre\";")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Porque el Señor ha elegido a Sión; *"),
						b: String::from("la ha deseado para su habitación.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("\"Esta es para siempre mi lugar de reposo; *"),
						b: String::from("aquí habitaré, porque en ella está mi deleite.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Bendeciré abundantemente sus provisiones; *"),
						b: String::from("a sus pobres los saciaré de pan.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Vestiré de salvación a sus sacerdotes, *"),
						b: String::from("y sus fieles cantarán con júbilo.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Allí haré florecer el poder de David; *"),
						b: String::from("he dispuesto una lámpara para mi Ungido.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("En cuanto a sus enemigos, los vestiré de vergüenza,*"),
						b: String::from("mas sobre él brillará su corona\".")
					}
				]
			}
		]
	};
}