use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_70: Psalm = Psalm {
		number: 70,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 578
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, in adjutorium"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dígnate, oh Dios, librarme; *"),
						b: String::from("Señor, apresúrate a socorrerme.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Sean avergonzados y confundidos a una, los que buscan mi vida; *"),
						b: String::from("vuelvan atrás y averguéncense, los que mi ruina desean.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Vuélvanse atrás, avergonzados, *"),
						b: String::from("los que con malicia me dicen: \"¡Ajá!\"")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Gócense y alégrense en ti todos los que te buscan; *"),
						b: String::from("digan siempre los que aman tu salvación:\n\"¡Grande es el Señor!\"")
					},
					PsalmVerse {
						number: 5,
						a: String::from("En cuanto a mí, estoy afligido y en necesidad; *"),
						b: String::from("apresúrate y ven a mí, oh Dios.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Mi ayuda y mi libertador eres tú; *"),
						b: String::from("no te tardes, oh Señor.")
					}
				]
			}
		]
	};
}