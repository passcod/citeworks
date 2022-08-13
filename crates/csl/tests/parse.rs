use std::fs::File;

use citeworks_csl::{
	dates::{Circa, Date, DateMeta, DateParts, Season},
	from_reader,
	items::{ItemType, ItemValue},
	names::Name,
	ordinaries::OrdinaryValue,
	Item,
};

use pretty_assertions::assert_eq;

#[test]
fn author2() {
	let mut file = File::open("tests/csl-json/author2.json").unwrap();
	let csl = from_reader(&mut file).unwrap();
	assert_eq!(
		csl,
		vec![Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			author: vec![
				Name {
					given: Some("Jane".into()),
					family: Some("Roe".into()),
					..Default::default()
				},
				Name {
					literal: Some("John Doe".into()),
					..Default::default()
				}
			],
			..Default::default()
		}]
	);
}

#[test]
fn single_date() {
	let mut file = File::open("tests/csl-json/single-date.json").unwrap();
	let csl = from_reader(&mut file).unwrap();
	assert_eq!(
		csl,
		vec![Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Single {
				date: DateParts {
					year: 2000,
					month: 1,
					day: 1
				},
				meta: Default::default(),
			}),
			..Default::default()
		}]
	);
}

#[test]
fn date_range() {
	let mut file = File::open("tests/csl-json/date-range.json").unwrap();
	let csl = from_reader(&mut file).unwrap();
	assert_eq!(
		csl,
		vec![Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Range {
				start: DateParts {
					year: 2000,
					month: 1,
					day: 1
				},
				end: DateParts {
					year: 2010,
					month: 10,
					day: 10
				},
				meta: Default::default(),
			}),
			..Default::default()
		}]
	);
}

#[test]
fn raw_date() {
	let mut file = File::open("tests/csl-json/raw-date.json").unwrap();
	let csl = from_reader(&mut file).unwrap();
	assert_eq!(
		csl,
		vec![Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Raw {
				date: "1st January 2000".into(),
				meta: Default::default(),
			}),
			..Default::default()
		}]
	);
}

#[test]
fn edtf() {
	let mut file = File::open("tests/csl-json/edtf.json").unwrap();
	let csl = from_reader(&mut file).unwrap();
	assert_eq!(
		csl,
		vec![Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Edtf {
				date: "2000-01-01/2010-10-10".into(),
				meta: Default::default(),
			}),
			..Default::default()
		}]
	);
}

#[test]
fn complex_date() {
	let mut file = File::open("tests/csl-json/complex-date.json").unwrap();
	let csl = from_reader(&mut file).unwrap();
	assert_eq!(
		csl,
		vec![Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Single {
				date: DateParts {
					year: 2000,
					month: 1,
					day: 1
				},
				meta: DateMeta {
					season: Some(Season::Winter),
					circa: Some(Circa::Year(2001)),
					..Default::default()
				}
			}),
			..Default::default()
		}]
	);
}

#[test]
fn extra() {
	let mut file = File::open("tests/csl-json/extra.json").unwrap();
	let csl = from_reader(&mut file).unwrap();
	assert_eq!(
		csl,
		vec![Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			fields: [
				(
					String::from("not-a-csl-key"),
					ItemValue::Ordinary(OrdinaryValue::String("extra".into()))
				),
				(
					String::from("not-a-csl-date"),
					ItemValue::Date(Date::Raw {
						date: "1/2/3456".into(),
						meta: Default::default(),
					})
				),
				(
					String::from("not-a-csl-name"),
					ItemValue::Names(vec![Name {
						literal: Some("surplus".into()),
						..Default::default()
					}])
				),
			]
			.into_iter()
			.collect(),
			..Default::default()
		}]
	);
}
