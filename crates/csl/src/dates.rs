//! Types and utilities for dates complex values.

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

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
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Date {
	/// Structured single date
	Single {
		/// Date as a [year, month, day] array
		date: DateParts,

		/// Additional date (meta)data
		meta: DateMeta,
	},

	/// Structured date range
	Range {
		/// Start date as a [year, month, day] array
		start: DateParts,

		/// End date as a [year, month, day] array
		end: DateParts,

		/// Additional date (meta)data
		meta: DateMeta,
	},

	/// Raw
	Raw {
		/// Date as a string in arbitrary format
		date: String,

		/// Additional date (meta)data
		meta: DateMeta,
	},

	/// EDTF
	Edtf {
		/// Date in EDTF string format
		date: String,

		/// Additional date (meta)data
		meta: DateMeta,
	},
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DateInternal {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	date_parts: Vec<DateParts>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	season: Option<Season>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	circa: Option<Circa>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	literal: Option<String>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	raw: Option<String>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	edtf: Option<String>,

	#[serde(flatten)]
	extra: HashMap<String, OrdinaryValue>,
}

impl Date {
	/// Get the [DateMeta] of any variant.
	pub fn meta(&self) -> &DateMeta {
		match self {
			Self::Single { meta, .. }
			| Self::Range { meta, .. }
			| Self::Raw { meta, .. }
			| Self::Edtf { meta, .. } => meta,
		}
	}
}

impl Serialize for Date {
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

impl<'de> Deserialize<'de> for Date {
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
			Ok(Self::Raw {
				date: date.clone(),
				meta: DateMeta::from_internal(internal),
			})
		} else {
			Err(D::Error::custom("unknown date format".to_string()))
		}
	}
}

/// The core "date-parts" of a date complex type.
///
/// In CSL-JSON this is an array `[year, month, day]`.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "DatePartsInternal", into = "DatePartsInternal")]
pub struct DateParts {
	/// Year, in the Gregorian calendar
	pub year: i64,

	/// Month, starting from 1
	pub month: u8,

	/// Day of the month, starting from 1
	pub day: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
struct DatePartsInternal(StrumI64, StrumU8, StrumU8);

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum StrumI64 {
	String(String),
	Num(i64),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum StrumU8 {
	String(String),
	Num(u8),
}

impl TryFrom<StrumI64> for i64 {
	type Error = ParseIntError;

	fn try_from(value: StrumI64) -> Result<Self, Self::Error> {
		match value {
			StrumI64::String(s) => s.parse(),
			StrumI64::Num(t) => Ok(t),
		}
	}
}

impl TryFrom<StrumU8> for u8 {
	type Error = ParseIntError;

	fn try_from(value: StrumU8) -> Result<Self, Self::Error> {
		match value {
			StrumU8::String(s) => s.parse(),
			StrumU8::Num(t) => Ok(t),
		}
	}
}

impl TryFrom<DatePartsInternal> for DateParts {
	type Error = ParseIntError;

	fn try_from(
		DatePartsInternal(year, month, day): DatePartsInternal,
	) -> Result<Self, Self::Error> {
		Ok(Self {
			year: year.try_into()?,
			month: month.try_into()?,
			day: day.try_into()?,
		})
	}
}

impl From<DateParts> for DatePartsInternal {
	fn from(parts: DateParts) -> Self {
		Self(
			StrumI64::Num(parts.year),
			StrumU8::Num(parts.month),
			StrumU8::Num(parts.day),
		)
	}
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// Date metadata or less-precise fields.
pub struct DateMeta {
	/// A season.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub season: Option<Season>,

	/// Unprecise date.
	///
	/// A number will be considered a year.
	///
	/// Can also take a boolean to mark the enclosing date as approximate.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub circa: Option<Circa>, // String, number, bool

	/// Full date in whatever format.
	#[serde(default, skip_serializing_if = "Option::is_none")]
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

impl Hash for DateMeta {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.season.hash(state);
		self.circa.hash(state);
		self.literal.hash(state);
		for (k, v) in &self.extra {
			(k, v).hash(state);
		}
	}
}

/// Circa field of date metadata
///
/// This has multiple uses:
/// - it can be a year or arbitrary string, interpreted like `ca. 2008`; or
/// - it can be a boolean (generally only `true`) to indicate that the
///   containing date is itself approximate.
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
	/// If the [Circa] is an arbitrary string, return it.
	pub fn as_arbitrary(&self) -> Option<&str> {
		if let Self::Arbitrary(str) = self {
			Some(str.as_ref())
		} else {
			None
		}
	}

	/// If the [Circa] is a numerical year, return it.
	pub fn as_year(&self) -> Option<i64> {
		if let Self::Year(num) = self {
			Some(*num)
		} else {
			None
		}
	}

	/// If the [Circa] is a boolean, return it.
	pub fn as_bool(&self) -> Option<bool> {
		if let Self::Bool(b) = self {
			Some(*b)
		} else {
			None
		}
	}
}

/// Season value for approximate dates.
///
/// This does not contain information as to where the season is, e.g.
/// Winter in the north hemisphere could be Summer in the south.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Season {
	/// Spring season, or `season-01` in CSL.
	Spring,

	/// Summer season, or `season-02` in CSL.
	Summer,

	/// Autumn season, or `season-03` in CSL.
	Autumn,

	/// Winter season, or `season-04` in CSL.
	Winter,
}

impl Display for Season {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Spring => "spring",
				Self::Summer => "summer",
				Self::Autumn => "autumn",
				Self::Winter => "winter",
			}
		)
	}
}

impl FromStr for Season {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"spring" | "season-01" => Ok(Self::Spring),
			"summer" | "season-02" => Ok(Self::Summer),
			"autumn" | "season-03" => Ok(Self::Autumn),
			"winter" | "season-04" => Ok(Self::Winter),
			other => Err(format!("unknown season: {other:?}")),
		}
	}
}

impl Serialize for Season {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let s = self.to_string();
		s.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for Season {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Season::from_str(&s).map_err(D::Error::custom)
	}
}
