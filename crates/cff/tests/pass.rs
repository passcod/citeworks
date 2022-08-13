use std::fs::File;

use citeworks_cff::{
	from_reader,
	names::{EntityName, Name, NameMeta, PersonName},
	Cff, Date, License, Result, references::{Reference, RefType},
};

use pretty_assertions::assert_eq;
use spdx::Expression;
use url::Url;

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
			authors: vec![Name::Person(PersonName {
				family_names: Some("Haines".into()),
				given_names: Some("Robert".into()),
				..Default::default()
			})],
			..Cff::default()
		}
	);
}

#[test]
fn short() {
	assert_eq!(
		parse_file("short").unwrap(),
		Cff {
			message:
				"If you use this software in your work, please cite it using the following metadata"
					.into(),
			title: "Ruby CFF Library".into(),
			authors: vec![Name::Person(PersonName {
				family_names: Some("Haines".into()),
				given_names: Some("Robert".into()),
				affiliation: Some("The University of Manchester, UK".into()),
				..Default::default()
			})],
			keywords: vec!["ruby".into(), "credit".into(), "citation".into()],
			version: Some("0.4.0".into()),
			date_released: Some(Date {
				year: 2018,
				month: 7,
				day: 22
			}),
			license: Some(License::Single(Expression::parse("Apache-2.0").unwrap())),
			repository_artifact: Some(Url::parse("https://rubygems.org/gems/cff").unwrap()),
			..Cff::default()
		}
	);
}

#[test]
fn simple() {
	assert_eq!(
		parse_file("simple").unwrap(),
		Cff {
			message: "If you use this software, please cite it as below.".into(),
			title: "My Research Software".into(),
			authors: vec![Name::Person(PersonName {
				family_names: Some("Druskat".into()),
				given_names: Some("Stephan".into()),
				meta: NameMeta {
					orcid: Some(Url::parse("https://orcid.org/0000-0003-4925-7248").unwrap()),
					..Default::default()
				},
				..Default::default()
			})],
			version: Some("2.0.4".into()),
			date_released: Some(Date {
				year: 2017,
				month: 12,
				day: 18
			}),
			doi: Some("10.5281/zenodo.1234".into()),
			..Cff::default()
		}
	);
}

#[test]
fn binary() {
	assert_eq!(
		parse_file("binary").unwrap(),
		Cff {
			message: "If you use MRT, please cite the following.".into(),
			title: "My Research Tool Kickstarter".into(),
			authors: vec![Name::Person(PersonName {
				family_names: Some("Druskat".into()),
				given_names: Some("Stephan".into()),
				meta: NameMeta {
					orcid: Some(Url::parse("https://orcid.org/0000-0003-4925-7248").unwrap()),
					..Default::default()
				},
				..Default::default()
			})],
			version: Some("2.0.4".into()),
			date_released: Some(Date {
				year: 2017,
				month: 12,
				day: 18
			}),
			repository_artifact: Some(
				Url::parse("https://hu.berlin/nexus/mrt-kickstarter/2.0.4/mrt2-kickstarter.exe")
					.unwrap()
			),
			..Cff::default()
		}
	);
}

#[test]
fn closed_source() {
	assert_eq!(
		parse_file("closed-source").unwrap(),
		Cff {
			message: "If you dare use this commercial, closed-source, strangely versioned software in your research, please at least cite it as below.".into(),
			title: "Opaquity".into(),
			authors: vec![Name::Person(PersonName {
					family_names: Some("Vader".into()),
					name_suffix: Some("né Skywalker".into()),
					given_names: Some(r#"Anakin "Darth""#.into()),
					..Default::default()
			})],
			contact: vec![Name::Entity(EntityName {
				name: Some("Dark Side Software".into()),
				meta: NameMeta {
					address: Some("DS-1 Orbital Battle Station, near Scarif".into()),
					email: Some("father@imperial-empire.com".into()),
					tel: Some("+850 (0)123-45-666".into()),
					..Default::default()
				},
				..Default::default()
			})],
			version: Some("opq-1234-XZVF-ACME-RLY".into()),
			date_released: Some(Date {
				year: 2017,
				month: 2,
				day: 28
			}),
			url: Some(Url::parse("http://www.opaquity.com").unwrap()),
			..Cff::default()
		}
	);
}

#[test]
fn conference_paper() {
	assert_eq!(
		parse_file("conference-paper").unwrap(),
		Cff {
			message: "If you use this software, please cite the software and the paper.".into(),
			title: "My Research Tool".into(),
			authors: vec![Name::Person(PersonName {
				family_names: Some("Druskat".into()),
				given_names: Some("Stephan".into()),
				meta: NameMeta {
					orcid: Some(Url::parse("https://orcid.org/0000-0003-4925-7248").unwrap()),
					..Default::default()
				},
				..Default::default()
			})],
			version: Some("1.0.4".into()),
			date_released: Some(Date {
				year: 2017,
				month: 12,
				day: 18
			}),
			references: vec![Reference {
				work_type: RefType::ConferencePaper,
				authors: vec![Name::Person(PersonName {
					family_names: Some("Doe".into()),
					given_names: Some("Jane".into()),
					..Default::default()
				})],
				editors: vec![Name::Person(PersonName {
					family_names: Some("Kirk".into()),
					given_names: Some("James T.".into()),
					..Default::default()
				})],
				title: Some("Ultimate-accuracy syntax parsing with My Research Tool".into()),
				year: Some(2017),
				collection_title: Some("Proceedings of the 1st Conference on Wishful Thinking".into()),
				collection_doi: Some("10.5281/zenodo.123456".into()),
				conference: Some(EntityName {
					name: Some("1st Conference on Wishful Thinking".into()),
					date_start: Some(Date { year: 2017, month: 4, day: 1 }),
					date_end: Some(Date { year: 2017, month: 4, day: 1 }),
					meta: NameMeta {
						address: Some("123 Main St".into()),
						city: Some("Bielefeld".into()),
						country: Some("UM".into()),
						region: Some("Jarvis Island".into()),
						..Default::default()
					},
					..Default::default()
				}),
				start: Some(42),
				end: Some(45),
				doi: Some("10.5281/zenodo.1234".into()),
				..Default::default()
			}],
			doi: Some("10.5281/zenodo.1234".into()),
			..Cff::default()
		}
	);
}
