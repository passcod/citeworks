//! Types and utilities for names e.g. of authors.

use serde::{Deserialize, Serialize};
use url::Url;

/// Information about a person or entity.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Name {
	/// A human person.
	Person {
		/// The name information of the person.
		#[serde(flatten)]
		name: PersonName,

		/// Common author metadata fields.
		#[serde(flatten)]
		meta: NameMeta,
	},

	/// An entity, e.g. research institution, company, co-op...
	Entity {
		/// The name of the entity.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		name: Option<String>,

		/// The entity's starting date.
		///
		/// For example, a conference.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		date_start: Option<String>,

		/// The entity's ending date.
		///
		/// For example, a conference.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		date_end: Option<String>,

		/// Common author metadata fields.
		#[serde(flatten)]
		meta: NameMeta,
	},

	/// A truly anonymous author.
	///
	/// This is the entry `- name: anonymous`.
	Anonymous,
}

/// Fields common to both types of names (persons and entities).
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NameMeta {
	/// [ORCID] identifier.
	///
	/// [ORCID]: https://orcid.org
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub orcid: Option<Url>,

	/// Physical or postal address.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub address: Option<String>,

	/// Alias or pseudonym.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub alias: Option<String>,

	/// City.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,

	/// Country.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,

	/// Email address.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,

	/// Post code.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub post_code: Option<String>,

	/// Region.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,

	/// Telephone number.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub tel: Option<String>,

	/// Fax number.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub fax: Option<String>,

	/// Website.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub website: Option<Url>,
}

/// The name of a person.
///
/// At least one field must be provided.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PersonName {
	/// Family names.
	///
	/// This includes combinations of given and patronymic forms, such as
	/// _Guðmundsdóttir_ or _bin Osman_; double names with or without hyphen,
	/// such as _Leutheusser-Schnarrenberger_ or _Sánchez Vicario_. It can
	/// potentially also specify names that include prepositions or (nobiliary)
	/// particles, especially if they occur in between family names such as in
	/// Spanish- or Portuguese-origin names, such as _Fernández de Córdoba_.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub family_names: Option<String>,

	/// Given or chosen names.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub given_names: Option<String>,

	/// The person's name particle.
	///
	/// For example, a [nobiliary] particle, or a preposition meaning _of_ or
	/// _from_ (for example _von_ in _Alexander von Humboldt_).
	///
	/// This may also be called the "non-dropping particle".
	///
	/// [nobiliary]: https://en.wikipedia.org/wiki/Nobiliary_particle
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name_particle: Option<String>,

	/// The person's name suffix.
	///
	/// For example, _Jr._ for _Sammy Davis Jr._ or _III_ for _Frank Edwin
	/// Wright III_.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name_suffix: Option<String>,

	/// Affiliation (e.g. organisation membership).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub affiliation: Option<String>,
}