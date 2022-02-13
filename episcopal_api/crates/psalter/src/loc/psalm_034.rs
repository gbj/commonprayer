use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_34: Psalm = Psalm {
		number: 34,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 526
				},
				local_name: String::from(""),
				latin_name: String::from("Benedicam Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Bendeciré al Señor en todo tiempo; *"),
						b: String::from("su alabanza estará siempre en mi boca.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("En el Señor me gloriaré; *"),
						b: String::from("lo oigan los mansos y se regocijen.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Proclamen conmigo la grandeza del Señor; *"),
						b: String::from("ensalcemos a una su Nombre.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Busqué al Señor y él me respondió, *"),
						b: String::from("y me libró de todos mis temores.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("A él miren y sean alumbrados, *"),
						b: String::from("y sus rostros no se avergüencen.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Este pobre clamó, y el Señor le oyó, *"),
						b: String::from("y lo libró de todas sus angustias.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("El ángel del Señor acampa en derredor de los que le temen,*"),
						b: String::from("y los libertará.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Gusten, y vean que es bueno el Señor; *"),
						b: String::from("dichosos los que en el confían.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Teman al Señor, ustedes sus santos, *"),
						b: String::from("pues nada falta a los que le temen.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los leoncillos necesitan, y tienen hambre, *"),
						b: String::from("pero los que buscan al Señor no tendrán falta\nde ningún bien.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Vengan, hijos, y escúchenme; *"),
						b: String::from("el temor del Señor les enseñaré.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("¿Hay alguien que ame la vida, *"),
						b: String::from("y desee muchos días para ver el bien ?")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Guarda tu lengua del mal, *"),
						b: String::from("y tus labios de hablar engaño.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Apártate del mal, y haz el bien; *"),
						b: String::from("busca la paz, y síguela.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Los ojos del Señor están sobre los justos, *"),
						b: String::from("y atentos sus oídos a su clamor.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("La ira del Señor contra los que mal hacen, *"),
						b: String::from("para borrar de la tierra su memoria.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Claman los justos, y el Señor escucha, *"),
						b: String::from("y los libra de todas sus angustias.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Cercano está el Señor a los quebrantados de corazón, *"),
						b: String::from("y salvará a los humildes de espíritu.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Muchos son las aflicciones de los justos, *"),
						b: String::from("pero de todas ellas les librará el Señor.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("El guarda todos sus huesos; *"),
						b: String::from("ni uno de ellos será quebrantado.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Matará al malo la maldad, *"),
						b: String::from("y los que aborrecen al justo serán condenados.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("El Señor redime la vida de sus siervos, *"),
						b: String::from("y no serán condenados los que en él confían.")
					}
				]
			}
		]
	};
}