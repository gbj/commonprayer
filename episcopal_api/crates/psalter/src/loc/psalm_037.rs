use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_37: Psalm = Psalm {
		number: 37,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 531
				},
				local_name: String::from("Parte I"),
				latin_name: String::from("Noli aemulari"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("No te impacientes a causa de los malignos, *"),
						b: String::from("ni tengas celos de los que hacen mal")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Porque como hierba pronto se marchitarán, *"),
						b: String::from("y como césped se agotarán.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Confía en el Señor, y haz el bien; *"),
						b: String::from("habita en la tierra, y aliméntate de sus caudales.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Deléitate en el Señor, *"),
						b: String::from("y él te dará las peticiones de tu corazón.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Encomienda al Señor tu camino; *"),
						b: String::from("confía en él, y él actuará.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Exhibirá tu justicia como la luz, *"),
						b: String::from("y tu rectitud como el mediodía.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Guarda silencio ante el Señor, *"),
						b: String::from("y espera en él con paciencia.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("No te impacientes del que medra, *"),
						b: String::from("del que tiene éxito en sus maldades.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Deja la ira, desecha el enojo; *"),
						b: String::from("la impaciencia sólo conduce al mal;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Porque los malignos serán arrancados, *"),
						b: String::from("pero los que invocan al Señor,\nhe aquí heredarán la tierra.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Pues dentro de poco no existirán los malos; *"),
						b: String::from("observarás su lugar, y no estarán allí.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Mas los mansos heredarán la tierra, *"),
						b: String::from("y se recrearán con abundancia de paz.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("El maligno trama contra el justo; *"),
						b: String::from("y cruje sobre él sus dientes.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Mi Soberano se reirá de ellos, *"),
						b: String::from("porque ve que viene su día.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Los malos desenvainan espada, y atesan su arco para derribar al pobre y al menesteroso, *"),
						b: String::from("para matar a los de recto proceder.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Su espada entrará en su propio corazón, *"),
						b: String::from("y su arco será quebrado.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Mejor es lo poco del justo, *"),
						b: String::from("que la riqueza grande de los malos;")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 532
				},
				local_name: String::from("Parte II"),
				latin_name: String::from("Novit Dominus"),
				verses: vec![
					PsalmVerse {
						number: 18,
						a: String::from("Porque el poder de los malos será quebrado, *"),
						b: String::from("mas el Señor sostendrá a los justos.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("El Señor vela por las sendas de los honrados, *"),
						b: String::from("y la heredad de ellos será para siempre.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("No serán avergonzados en el mal tiempo, *"),
						b: String::from("y en los días de hambre serán hartos.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("En cuanto a los malos, perecerán, *"),
						b: String::from("y los enemigos del Señor, como las flores del prado,\nse disiparán, se disiparán como el humo.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("El malo toma prestado, y no paga; *"),
						b: String::from("mas el justo es generoso y dadivoso.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Los benditos de Dios heredarán la tierra, *"),
						b: String::from("mas los malditos por él serán destruidos.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("Los pasos de los mortales son dirigidos por el Señor, *"),
						b: String::from("y fortalece a aquéllos en cuyos caminos él se deleita.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Si tropiezan, no caerán, *"),
						b: String::from("porque el Señor los lleva de la mano.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Joven fui, y he envejecido, *"),
						b: String::from("y no he visto a justo desamparado,\nni que su descendencia mendigue pan.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("En todo tiempo los justos son generosos y prestan, *"),
						b: String::from("y su descendencia es para ellos bendición.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Apártate del mal, y haz el bien, *"),
						b: String::from("y habitarás en la tierra para siempre;")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Porque el Señor ama la justicia, *"),
						b: String::from("y no desampara a sus santos.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Para siempre serán guardados, *"),
						b: String::from("mas los hijos de los malos serán destruidos.")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Los justos heredarán la tierra, *"),
						b: String::from("y vivirán para siempre sobre ella.")
					},
					PsalmVerse {
						number: 32,
						a: String::from("La boca del justo profiere sabiduría, *"),
						b: String::from("y su lengua habla lo que es recto.")
					},
					PsalmVerse {
						number: 33,
						a: String::from("La ley de su Dios está en su corazón; *"),
						b: String::from("por tanto, sus pies no resbalarán.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("Acecha el malo al justo, *"),
						b: String::from("y busca ocasión para matarle.")
					},
					PsalmVerse {
						number: 35,
						a: String::from("El Señor no lo entregará en sus manos, *"),
						b: String::from("ni permitirá que sea declarado culpable en su juicio.")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Espera en el Señor, y guarda su camino; *"),
						b: String::from("él te exaltará para heredar la tierra;\ncuando sean destruidos los malos, lo verás.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("He visto al malo medrando, *"),
						b: String::from("floreciente como árbol lleno de hojas;")
					},
					PsalmVerse {
						number: 38,
						a: String::from("Pero yo pasé, y he aquí, ya no estaba; *"),
						b: String::from("busqué, y no lo hallé.")
					},
					PsalmVerse {
						number: 39,
						a: String::from("Mira al honrado; observa al justo; *"),
						b: String::from("porque hay futuro para el que es pacífico.")
					},
					PsalmVerse {
						number: 40,
						a: String::from("Mas los transgresores serán todos a una destruidos; *"),
						b: String::from("el futuro del malo será acortado.")
					},
					PsalmVerse {
						number: 41,
						a: String::from("La salvación de los justos es del Señor; *"),
						b: String::from("él es su fortaleza en tiempo de angustia.")
					},
					PsalmVerse {
						number: 42,
						a: String::from("El Señor los ayudará, y los librará; *"),
						b: String::from("los librará de los malignos, y los salvará,\npor cuanto en él se refugian.")
					}
				]
			}
		]
	};
}