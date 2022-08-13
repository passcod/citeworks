use std::{
	fmt::{Debug, Display},
	str::FromStr,
};

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// A date.
///
/// In CFF this is a string in `YYYY-MM-DD` format.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Date {
	/// Year, in the Gregorian calendar
	pub year: i64,

	/// Month, starting from 1
	pub month: u8,

	/// Day of the month, starting from 1
	pub day: u8,
}

impl Display for Date {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Self { year, month, day } = self;
		write!(f, "{year:04}-{month:02}-{day:02}")
	}
}

impl FromStr for Date {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let err = || -> String { format!("expected YYYY-MM-DD, got: {s:?}") };

		let [year, month, day]: [&str; 3] = s
			.splitn(3, '-')
			.collect::<Vec<_>>()
			.try_into()
			.map_err(|_| err())?;

		if year.len() != 4 || month.len() != 2 || day.len() != 2 {
			Err(err())
		} else {
			let date = Self {
				year: year.parse().map_err(|_| err())?,
				month: month.parse().map_err(|_| err())?,
				day: day.parse().map_err(|_| err())?,
			};

			if date.month == 0 || date.month > 12 {
				Err(format!(
					"month should be in range 1-12, got: {}",
					date.month
				))
			} else if date.day == 0 || date.day > 31 {
				Err(format!("day should be in range 1-31, got: {}", date.day))
			} else {
				Ok(date)
			}
		}
	}
}

impl Serialize for Date {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let s = self.to_string();
		s.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for Date {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Date::from_str(&s).map_err(D::Error::custom)
	}
}
