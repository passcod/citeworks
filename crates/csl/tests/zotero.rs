use std::{collections::HashMap, fs::File};

use citeworks_csl::from_reader;

use pretty_assertions::assert_eq;

#[test]
fn parse_export() {
	let mut file = File::open("tests/csl-json/zotero-export.json").unwrap();
	let csl = from_reader(&mut file).unwrap();

	assert_eq!(csl.len(), 3);
	for item in &csl {
		assert_eq!(item.fields, HashMap::new());
	}
}
