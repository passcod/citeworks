//! Types and utilities for dates complex values.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ordinaries::OrdinaryValue;

/// Date formats.
///
/// Date fields can be expressed in different forms.
///
/// The first serialises as an array format, containing either a single date in
/// a double-nested array keyed under the `date-parts` field, or a date range as
/// two inner arrays in the `date-parts` outer array. In this library, array
/// singles and array ranges are represented separately as `Single` and `Range`.
///
/// The second form is a field named `raw` with a string representation of the
/// date in arbitrary or human formats, which citation software may attempt to
/// recognise. This library doesn't attempt to parse raw dates.
///
/// [EDTF] (Extended Date/Time Format) is a structured string format for dates,
/// datetimes, and ranges established by the United States of America's Library
/// of Congress.
///
/// All forms may also have any of the [metadata or less-precise fields][meta].
///
/// [EDTF]: https://www.librarianshipstudies.com/2016/05/extended-date-time-format-edtf.html
/// [meta]: DateMeta
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DateValue {
	/// Structured single date
	Single { date: Date, meta: DateMeta },

	/// Structured date range
	Range {
		start: Date,
		end: Date,
		meta: DateMeta,
	},

	/// Raw
	Raw { date: String, meta: DateMeta },

	/// EDTF
	Edtf { date: String, meta: DateMeta },
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Date {
	pub year: i64,
	pub month: u32,
	pub day: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// Date metadata or less-precise fields.
pub struct DateMeta {
	/// A season.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub season: Option<Season>,

	/// Unprecise date.
	///
	/// A number will be considered a year.
	///
	/// Can also take a boolean to mark the enclosing date as approximate.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub circa: Option<Circa>, // String, number, bool

	/// Full date in whatever format.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub literal: Option<String>,

	/// Date fields not defined above.
	#[serde(flatten)]
	pub extra: HashMap<String, OrdinaryValue>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Circa {
	/// Arbitrary string for the circa value.
	Arbitrary(String),

	/// Approximate year.
	Year(i64),

	/// Whether the date itself is approximate.
	Bool(bool),
}

impl Circa {
	pub fn as_arbitrary(&self) -> Option<&str> {
		if let Self::Arbitrary(str) = self {
			Some(str.as_ref())
		} else {
			None
		}
	}

	pub fn as_year(&self) -> Option<i64> {
		if let Self::Year(num) = self {
			Some(*num)
		} else {
			None
		}
	}

	pub fn as_bool(&self) -> Option<bool> {
		if let Self::Bool(b) = self {
			Some(*b)
		} else {
			None
		}
	}
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum Season {
	Spring,
	Summer,
	Autumn,
	Winter,
}
