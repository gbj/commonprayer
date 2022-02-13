use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_6: Psalm = Psalm {
		number: 6,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 489
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, ne in furore"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, no me reprendas en tu enojo, *"),
						b: String::from("ni me castigues con tu ira.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Ten misericordia de mí, oh Señor, porque estoy debilitado; *"),
						b: String::from("sáname, oh Señor, porque mis huesos se estremecen.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Mi alma también está muy turbada; *"),
						b: String::from("y tú, oh Señor, ¿hasta cuándo?")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Vuélvete, oh Señor, libra mi vida; *"),
						b: String::from("sálvame por tu misericordia;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Porque en la muerte no hay memoria de ti; *"),
						b: String::from("en el sepulcro, ¿quién te alabará?")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Me he consumido a fuerza de gemir; *"),
						b: String::from("todas las noches inundo de llanto mi lecho,\nriego mi cama con mis lágrimas.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Mis ojos están gastados de sufrir; *"),
						b: String::from("se han envejecido a causa de todos mis angustiadores.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Apártense de mí, todos los hacedores de iniquidad; *"),
						b: String::from("porque el Señor ha oído la voz de mi llanto.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("El Señor ha oído mi ruego; *"),
						b: String::from("ha recibido el Señor mi oración.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Se avergonzarán y se turbarán todos mis enemigos; *"),
						b: String::from("se volverán y serán avergonzados de repente.")
					}
				]
			}
		]
	};
}