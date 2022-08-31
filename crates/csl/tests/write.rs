use std::fs::File;

use citeworks_csl::{
	dates::{Circa, Date, DateMeta, DateParts, Season},
	items::{ItemType, ItemValue},
	names::Name,
	ordinaries::OrdinaryValue,
	to_vec, Item,
};

use pretty_assertions::assert_eq;

fn json_file(name: &str) -> serde_json::Value {
	let file = File::open(format!("tests/csl-json/{name}.json")).unwrap();
	serde_json::from_reader(file).unwrap()
}

fn json_item(item: Item) -> serde_json::Value {
	let json = to_vec(&[item]).unwrap();
	serde_json::from_slice(&json).unwrap()
}

#[test]
fn author2() {
	assert_eq!(
		json_file("author2"),
		json_item(Item {
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
		})
	);
}

#[test]
fn single_date() {
	assert_eq!(
		json_file("single-date"),
		json_item(Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Single {
				date: DateParts {
					year: 2000,
					month: Some(1),
					day: Some(1)
				},
				meta: Default::default(),
			}),
			..Default::default()
		})
	);
}

#[test]
fn date_range() {
	assert_eq!(
		json_file("date-range"),
		json_item(Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Range {
				start: DateParts {
					year: 2000,
					month: Some(1),
					day: Some(1)
				},
				end: DateParts {
					year: 2010,
					month: Some(10),
					day: Some(10)
				},
				meta: Default::default(),
			}),
			..Default::default()
		})
	);
}

#[test]
fn raw_date() {
	assert_eq!(
		json_file("raw-date"),
		json_item(Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Raw {
				date: "1st January 2000".into(),
				meta: Default::default(),
			}),
			..Default::default()
		})
	);
}

#[test]
fn edtf() {
	assert_eq!(
		json_file("edtf"),
		json_item(Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Edtf {
				date: "2000-01-01/2010-10-10".into(),
				meta: Default::default(),
			}),
			..Default::default()
		})
	);
}

#[test]
fn complex_date() {
	assert_eq!(
		json_file("complex-date"),
		json_item(Item {
			id: "example-id".into(),
			item_type: ItemType::Report,
			issued: Some(Date::Single {
				date: DateParts {
					year: 2000,
					month: Some(1),
					day: Some(1)
				},
				meta: DateMeta {
					season: Some(Season::Winter),
					circa: Some(Circa::Year(2001)),
					..Default::default()
				}
			}),
			..Default::default()
		})
	);
}

#[test]
fn extra() {
	assert_eq!(
		json_file("extra"),
		json_item(Item {
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
		})
	);
}
