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
/// this library this is checked at serialisation time, but unrecognised fields
/// are not errors when deserialised.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
	/// Unique ID of this item within the CSL document.
	pub id: String,

	/// Type of the resource.
	#[serde(rename = "type")]
	pub item_type: ItemType,

	/// Author(s) of the item.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub author: Vec<Name>,

	/// Date the item was issued on.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issued: Option<Date>,

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
