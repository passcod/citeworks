use std::fs::File;

use citeworks_cff::{
	from_reader,
	names::{EntityName, Name, NameMeta, PersonName},
	references::{RefType, Reference},
	Cff, Date, License, Result,
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
				collection_title: Some(
					"Proceedings of the 1st Conference on Wishful Thinking".into()
				),
				collection_doi: Some("10.5281/zenodo.123456".into()),
				conference: Some(EntityName {
					name: Some("1st Conference on Wishful Thinking".into()),
					date_start: Some(Date {
						year: 2017,
						month: 4,
						day: 1
					}),
					date_end: Some(Date {
						year: 2017,
						month: 4,
						day: 1
					}),
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

#[test]
fn container() {
	assert_eq!(
		parse_file("container").unwrap(),
		Cff {
			message: "If you use the MRT Docker container, please cite the following.".into(),
			title: "mrt-iain-m-banks".into(),
			authors: vec![
				Name::Entity(EntityName {
					name: Some("Humboldt-Universität zu Berlin".into()),
					meta: NameMeta {
						website: Some(Url::parse("https://www.linguistik.hu-berlin.de/").unwrap()),
						..Default::default()
					},
					..Default::default()
				}),
				Name::Person(PersonName {
					family_names: Some("Doe".into()),
					given_names: Some(r"Jane".into()),
					..Default::default()
				}),
			],
			version: Some("1.0.4 (Iain M. Banks)".into()),
			date_released: Some(Date {
				year: 2017,
				month: 12,
				day: 18
			}),
			url: Some(
				Url::parse(
					"https://github.com/doe/docker-brew-mrt-core/blob/160d54f9e935/iain/Dockerfile"
				)
				.unwrap()
			),
			repository: Some(
				Url::parse("https://hub.docker.hu-berlin.de/_/mrt-iain-m-banks/").unwrap()
			),
			..Cff::default()
		}
	);
}

#[test]
fn mardyn() {
	assert_eq!(
		parse_file("mardyn").unwrap(),
		Cff {
			message: "This file contains CFF citation information, cf. https://citation-file-format.github.io/, for the ls1 mardyn molecular dynamics code developed by the Boltzmann-Zuse Society for Computational Molecular Engineering.".into(),
			title: "ls1 mardyn".into(),
			url: Some(Url::parse("http://www.ls1-mardyn.de/").unwrap()),
			repository_code: Some(Url::parse("https://projects.hlrs.de/projects/ls1/").unwrap()),
			license_url: Some(Url::parse("http://www.ls1-mardyn.de/license.html").unwrap()),
			authors: vec![Name::Entity(EntityName {
				name: Some("Boltzmann-Zuse Society for Computational Molecular Engineering".into()),
				meta: NameMeta {
					country: Some("DE".into()),
					..Default::default()
				},
				..Default::default()
			})],
			contact: vec![Name::Person(PersonName {
				given_names: Some("Philipp".into()),
				family_names: Some("Neumann".into()),
				..Default::default()
			})],
			version: Some("Internal development version, situated between release 1.1.1 and prospective future release 1.2".into()),
			commit: Some("Revision 6473.".into()),
			date_released: Some(Date {
				year: 2018,
				month: 9,
				day: 5
			}),
			abstract_text: Some("The molecular dynamics code ls1 mardyn (large systems 1: molecular dynamics), developed by the Boltzmann-Zuse Society for Computational Molecular Engineering, is a scalable massively-parallel molecular modelling and simulation code for classical-mechanical intermolecular pair potential models of low-molecular fluids.".into()),
			references: vec![Reference {
				work_type: RefType::Article,
				title: Some("ls1 mardyn: The massively parallel molecular dynamics code for large systems".into()),
				year: Some(2014),
				authors: vec![
					Name::Person(PersonName {
						given_names: Some("Christoph".into()),
						family_names: Some("Niethammer".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Stefan".into()),
						family_names: Some("Becker".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Martin".into()),
						family_names: Some("Bernreuther".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Martin".into()),
						family_names: Some("Buchholz".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Wolfgang".into()),
						family_names: Some("Eckhardt".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Alexander".into()),
						family_names: Some("Heinecke".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Stephan".into()),
						family_names: Some("Werth".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Hans-Joachim".into()),
						family_names: Some("Bungartz".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Colin W.".into()),
						family_names: Some("Glass".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Hans".into()),
						family_names: Some("Hasse".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Jadran".into()),
						family_names: Some("Vrabec".into()),
						..Default::default()
					}),
					Name::Person(PersonName {
						given_names: Some("Martin".into()),
						family_names: Some("Horsch".into()),
						..Default::default()
					}),
				],
				journal: Some("Journal of Chemical Theory and Computation".into()),
				volume: Some(10),
				issue: Some("10".into()),
				start: Some(4455),
				end: Some(4464),
				doi: Some("10.1021/ct500169q".into()),
				..Default::default()
			}],
			..Cff::default()
		}
	);
}
