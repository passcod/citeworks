use std::fs::File;

use citeworks_cff::{
	from_reader,
	names::{Name, PersonName},
	Cff, Result,
};

fn parse_file(name: &str) -> Result<Cff> {
	let file = File::open(format!("tests/pass/{name}.cff")).unwrap();
	from_reader(file)
}

#[test]
fn minimal() {
	assert_eq!(
		parse_file("minimal").unwrap(),
		Cff {
			message:
				"If you use this software in your work, please cite it using the following metadata"
					.into(),
			title: "Ruby CFF Library".into(),
			authors: vec![Name::Person {
				name: PersonName {
					family_names: Some("Haines".into()),
					given_names: Some("Robert".into()),
					..Default::default()
				},
				meta: Default::default(),
			}],
			..Cff::default()
		}
	);
}
