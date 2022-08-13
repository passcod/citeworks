//! Types and utilities for bibliography items.

use std::{
	collections::HashMap,
	hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use crate::{dates::Date, names::Name, ordinaries::OrdinaryValue};

/// An item carries the details of a single unique bibliographic resource.
///
/// The set of fields that an item may have is determined by the item type; in
/// this library this is not checked: known fields have their field type defined
/// and checked, but unrecognised fields are not errors when deserialised and
/// go in the generic `fields` map.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Item {
	/// Unique ID of this item within the CSL document.
	pub id: String,

	/// Type of the resource.
	#[serde(rename = "type")]
	pub item_type: ItemType,

	/// Author(s).
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub author: Vec<Name>,

	/// Contributor(s) to the item.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub contributor: Vec<Name>,

	/// Date the item was issued on.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issued: Option<Date>,

	/// Date the item was last updated.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub updated: Option<Date>,

	/// Date the item was published on.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub published: Option<Date>,

	/// Date the item was accessed (for citations).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub accessed: Option<Date>,

	/// Category (scientific field or type of study)
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub category: Option<OrdinaryValue>,

	/// ISSN.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "ISSN")]
	pub issn: Option<OrdinaryValue>,

	/// EISSN.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "EISSN")]
	pub eissn: Option<OrdinaryValue>,

	/// ISSNL.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "ISSNL")]
	pub issnl: Option<OrdinaryValue>,

	/// DOI.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "DOI")]
	pub doi: Option<OrdinaryValue>,

	/// URL.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "URL")]
	pub url: Option<OrdinaryValue>,

	/// Title.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub title: Option<OrdinaryValue>,

	/// Short title.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub title_short: Option<OrdinaryValue>,

	/// Summary.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub summary: Option<OrdinaryValue>,

	/// Abstract.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "abstract")]
	pub abstract_text: Option<OrdinaryValue>,

	/// Name of the issuing publication.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub container_title: Option<OrdinaryValue>,

	/// Abbreviated name of the issuing publication.
	///
	/// This has non-standard casing: `journalAbbreviation`.
	#[serde(
		default,
		skip_serializing_if = "Option::is_none",
		rename = "journalAbbreviation"
	)]
	pub journal_abbrevation: Option<OrdinaryValue>,

	/// Volume number of the issuing publication.
	///
	/// This can be a numerical value but is often a string of a number.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub volume: Option<OrdinaryValue>,

	/// Issue number of the issuing publication.
	///
	/// This can be a numerical value but is often a string of a number.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issue: Option<OrdinaryValue>,

	/// Page number or page range in the issuing publication.
	///
	/// This can be a numerical value but is often a string of a number, or of
	/// the range in `N-M` format.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub page: Option<OrdinaryValue>,

	/// Language code.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub language: Option<OrdinaryValue>,

	/// Plain source name.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub source: Option<OrdinaryValue>,

	/// Copyright statement.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub rights: Option<OrdinaryValue>,

	/// License statement.
	///
	/// Sometimes used as a synonym of `rights` rather than actual licensing.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub license: Option<OrdinaryValue>,

	/// Note for extra details that are important to include in the citation but
	/// don't have a standard field.
	///
	/// May be structured or semi-structured data, but as there is no convention
	/// processors shouldn't make assumptions unless they can assert meaning.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub note: Option<OrdinaryValue>,

	/// Any field that is not directly supported by name.
	#[serde(flatten)]
	pub fields: HashMap<String, ItemValue>,
}

/// Any of the possible value types of an item's fields.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemValue {
	/// Ordinary fields containing string or numeric values.
	///
	/// In ordinary fields, the processor recognizes a limited set of [HTML-like
	/// tags][html-tags] for visual formatting.
	///
	/// One common ordinary field is `title`, which identifies the title of the
	/// citation item.
	///
	/// [html-tags]: https://citeproc-js.readthedocs.io/en/latest/csl-json/markup.html#html-like-formatting-tags
	Ordinary(OrdinaryValue),

	/// Date fields containing dates or ranges of dates.
	///
	/// A date field is a complex field that expresses a date or a range of
	/// dates, for example `issued`, which identifies the date an item was
	/// issued or published.
	Date(Date),

	/// Names fields containing lists of names.
	///
	/// A names field is a complex field that lists persons as authors,
	/// contributors, or creators, etc. Each field is an array of objects, with
	/// each object containing information about one person.
	Names(Vec<Name>),
}

impl Hash for Item {
	/// Hashes this value, considering only the ID field.
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

/// The type of the bibliographic resource.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum ItemType {
	// CSL
	Article,
	ArticleJournal,
	ArticleMagazine,
	ArticleNewspaper,
	Bill,
	Book,
	Broadcast,
	Chapter,
	Classic,
	Collection,
	Dataset,
	Document,
	Entry,
	EntryDictionary,
	EntryEncyclopedia,
	Figure,
	Graphic,
	Hearing,
	Interview,
	LegalCase,
	Legislation,
	Manuscript,
	Map,
	MotionPicture,
	MusicalScore,
	Pamphlet,
	PaperConference,
	Patent,
	Performance,
	Periodical,
	PersonalCommunication,
	Post,
	PostWeblog,
	Regulation,
	Report,
	Review,
	ReviewBook,
	Software,
	Song,
	Speech,
	Standard,
	Thesis,
	Treaty,
	Webpage,

	// CSL-M additional types
	Gazette,
	Video,
	LegalCommentary,
}

impl Default for ItemType {
	fn default() -> Self {
		Self::Article
	}
}
