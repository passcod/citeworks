//! Types and utilities for names complex values.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Name complex field type.
///
/// Contains information about one person.
///
/// Should have at least the `family` field (for personyms) or the `literal`
/// field (for institutions). People using mononyms can have _just_ the `family`
/// field as their sole name.
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Name {
	/// Represents the familial name that a person inherits.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub family: Option<String>,

	/// Represents the name a person has been given or has chosen for themselves.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub given: Option<String>,

	/// Elements before the given name.
	///
	/// For example, in "Rev. Martin Luther Jr.", "Rev." is the dropping
	/// particle.
	///
	/// It's also acceptable to include these particles directly as part of the
	/// `given` field.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub dropping_particle: Option<String>,

	/// Elements before the family name.
	///
	/// For example, in "Bartolomé de las Casas", "de las" are the non-dropping
	/// particles.
	///
	/// It's also acceptable to include these particles directly as part of the
	/// `family` field.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub non_dropping_particle: Option<String>,

	/// Elements after the family name.
	///
	/// For example, in "Rev. Martin Luther Jr.", "Jr." is the suffix.
	///
	/// Multiple suffixes may be given, e.g. "Jr., Ph.D.".
	///
	/// It's also acceptable to include these suffixes directly as part of the
	/// `family` field.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub suffix: Option<String>,

	/// Name of an institution, or whole name of a person.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub literal: Option<String>,

	/// Name fields not defined above.
	#[serde(flatten)]
	pub extra: BTreeMap<String, String>,
}
