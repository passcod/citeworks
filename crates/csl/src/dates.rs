//! Types and utilities for dates complex values.

use std::collections::HashMap;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DateInternal {
	#[serde(skip_serializing_if = "Vec::is_empty")]
	date_parts: Vec<Date>,

	#[serde(skip_serializing_if = "Option::is_none")]
	season: Option<Season>,

	#[serde(skip_serializing_if = "Option::is_none")]
	circa: Option<Circa>,

	#[serde(skip_serializing_if = "Option::is_none")]
	literal: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	raw: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	edtf: Option<String>,

	#[serde(flatten)]
	extra: HashMap<String, OrdinaryValue>,
}

impl DateValue {
	pub fn meta(&self) -> &DateMeta {
		match self {
			Self::Single { meta, .. }
			| Self::Range { meta, .. }
			| Self::Raw { meta, .. }
			| Self::Edtf { meta, .. } => meta,
		}
	}
}

impl Serialize for DateValue {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let meta = self.meta().clone();
		let mut internal = DateInternal {
			season: meta.season,
			circa: meta.circa,
			literal: meta.literal,
			extra: meta.extra,
			..Default::default()
		};

		match self {
			Self::Single { date, .. } => {
				internal.date_parts = vec![date.clone()];
			}
			Self::Range { start, end, .. } => {
				internal.date_parts = vec![start.clone(), end.clone()];
			}
			Self::Raw { date, .. } => {
				internal.raw = Some(date.clone());
			}
			Self::Edtf { date, .. } => {
				internal.edtf = Some(date.clone());
			}
		}

		internal.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for DateValue {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
		D::Error: serde::de::Error,
	{
		let internal = DateInternal::deserialize(deserializer)?;

		if internal.date_parts.len() == 1 {
			Ok(Self::Single {
				date: internal.date_parts[0].clone(),
				meta: DateMeta::from_internal(internal),
			})
		} else if internal.date_parts.len() == 2 {
			Ok(Self::Range {
				start: internal.date_parts[0].clone(),
				end: internal.date_parts[1].clone(),
				meta: DateMeta::from_internal(internal),
			})
		} else if let Some(date) = &internal.edtf {
			Ok(Self::Edtf {
				date: date.clone(),
				meta: DateMeta::from_internal(internal),
			})
		} else if let Some(date) = &internal.raw {
			Ok(Self::Edtf {
				date: date.clone(),
				meta: DateMeta::from_internal(internal),
			})
		} else {
			Err(D::Error::custom("unknown date format".to_string()))
		}
	}
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

impl DateMeta {
	fn from_internal(internal: DateInternal) -> Self {
		Self {
			season: internal.season,
			circa: internal.circa,
			literal: internal.literal,
			extra: internal.extra,
		}
	}
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
