//! Types and utilities for references to this or other works.

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{identifiers::Identifier, names::Name, Date, License};

/// A reference for a work.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Reference {
	/// The type of the referenced work.
	///
	/// This is required.
	#[serde(rename = "type")]
	pub work_type: RefType,

	/// The authors of the work.
	///
	/// This is required and must contain at least one author.
	pub authors: Vec<Name>,

	/// The abbreviation of a work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub abbreviation: Option<String>,

	/// The abstract of the work.
	///
	/// - If the work is a journal paper or other academic work,
	///   The abstract of the work.
	///
	/// - If the work is a film broadcast or similar,
	///   The synopsis of the work.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "abstract")]
	pub abstract_text: Option<String>,

	/// The DOI of a collection containing the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub collection_doi: Option<String>,

	/// The title of a collection or proceedings.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub collection_title: Option<String>,

	/// The type of a collection.
	///
	/// By convention this should be in lowercase.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub collection_type: Option<String>,

	/// The commit hash or revision number of the work, if it is software.
	///
	/// By convention:
	/// - if this is a Git hash, it should be bare lowercase hex, e.g.
	///   `1ff847d81f29c45a3a1a5ce73d38e45c2f319bba`;
	/// - if this is a decimal revision or build number, it should be preceded
	///   by a label, e.g. `Revision: 8612`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub commit: Option<String>,

	/// The conference where the work was presented.
	///
	/// This should specifically be a `Name::Entity`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub conference: Option<Name>,

	/// The contact person, group, company, etc. for a work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub contact: Vec<Name>,

	/// The copyright information pertaining to the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub copyright: Option<String>,

	/// The data type of a data set.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub data_type: Option<String>,

	/// The provider of the database where a work was accessed/is stored.
	///
	/// This should specifically be a `Name::Entity`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub database_provider: Option<Name>,

	/// The name of the database where a work was accessed/is stored.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub database: Option<String>,

	/// The date the work was accessed.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub date_accessed: Option<Date>,

	/// The date the work has been downloaded.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub date_downloaded: Option<Date>,

	/// The date the work has been published.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub date_published: Option<Date>,

	/// The date the work has been released.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub date_released: Option<Date>,

	/// The department where a work has been produced.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub department: Option<String>,

	/// The DOI of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub doi: Option<String>,

	/// The edition of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub edition: Option<String>,

	/// The editor(s) of a work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub editors: Vec<Name>,

	/// The editor(s) of a series in which the work has been published.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub editors_series: Vec<Name>,

	/// The start page of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub start: Option<u64>,

	/// The end page of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub end: Option<u64>,

	/// An entry in the collection that constitutes the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub entry: Option<String>,

	/// The name of the electronic file containing the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,

	/// The format in which a work is represented.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub format: Option<String>,

	/// The identifier(s) of the work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub identifiers: Vec<Identifier>,

	/// The institution where a work has been produced or published.
	///
	/// This should specifically be a `Name::Entity`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub institution: Option<Name>,

	/// The [ISBN] of the work.
	///
	/// The value is not validated.
	///
	/// [ISBN]: https://en.wikipedia.org/wiki/International_Standard_Book_Number
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub isbn: Option<String>,

	/// The [ISSN] of the work.
	///
	/// The value is not validated.
	///
	/// [ISSN]: https://en.wikipedia.org/wiki/International_Standard_Serial_Number
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issn: Option<String>,

	/// The issue of a periodical in which a work appeared.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issue: Option<String>,

	/// The publication date of the issue of a periodical in which a work appeared.
	///
	/// Note this is a freeform string.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issue_date: Option<String>,

	/// The name of the issue of a periodical in which the work appeared.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub issue_title: Option<String>,

	/// The name of the journal/magazine/newspaper/periodical where the work was published.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub journal: Option<String>,

	/// Keywords pertaining to the work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub keywords: Vec<String>,

	/// The language identifier(s) of the work.
	///
	/// These should be ISO639 strings in lowercase alpha-2 or alpha-3, but this
	/// library does not validate this.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub languages: Vec<String>,

	/// [SPDX][spdx] license expression(s).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub license: Option<License>,

	/// The URL of the license text under which the work is licensed.
	///
	/// This should only be used for non-standard licenses not included in the
	/// SPDX License List.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub license_url: Option<Url>,

	/// The line of code in the file where the work ends.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub loc_end: Option<u64>,

	/// The line of code in the file where the work starts.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub loc_start: Option<u64>,

	/// The location of the work.
	///
	/// This should specifically be a `Name::Entity`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub location: Option<Name>,

	/// The medium of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub medium: Option<String>,

	/// The month in which a work has been published.
	///
	/// Should be an integer in the range 1-12. Note this is not validated.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub month: Option<u8>,

	/// The [NIHMSID] of a work.
	///
	/// [NIHMSID]: https://web.archive.org/web/20210802210057/https://www.ncbi.nlm.nih.gov/pmc/about/public-access-info/
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub nihmsid: Option<String>,

	/// Notes pertaining to the work.
	///
	/// Note that this key should contain notes that may be picked up by some
	/// downstream tooling (e.g., reference managers), but not others
	/// (e.g., a software index).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub notes: Option<String>,

	/// The (library) [accession number] for a work.
	///
	/// [accession number]: https://en.wikipedia.org/wiki/Accession_number
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub number: Option<String>,

	/// The number of volumes making up the collection in which the work has
	/// been published.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub number_volumes: Option<u64>,

	/// The number of pages of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub pages: Option<u64>,

	/// The states for which a patent is granted.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub patent_states: Vec<String>,

	/// The [PMCID] of a work.
	///
	/// The value is not validated.
	///
	/// [PMCID]: https://web.archive.org/web/20210802210057/https://www.ncbi.nlm.nih.gov/pmc/about/public-access-info/
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub pmcid: Option<String>,

	/// The publisher who has published the work.
	///
	/// This should specifically be a `Name::Entity`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub publisher: Option<Name>,

	/// The recipient(s) of a personal communication.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub recipients: Vec<Name>,

	/// The URL of the work in a repository/archive.
	///
	/// This is to be used when the repository is neither a source code
	/// repository nor a build artifact repository. For source code, use the
	/// `repository_code` field; for binary releases or other built forms, use
	/// the `repository_artifact` field.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub repository: Option<Url>,

	/// The URL of the work in a build artifact/binary repository.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub repository_artifact: Option<Url>,

	/// The URL of the work in a source code repository.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub repository_code: Option<Url>,

	/// The scope of the reference, e.g., the section of the work it adheres to.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub scope: Option<String>,

	/// The section of a work that is referenced.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub section: Option<String>,

	/// The sender(s) of a personal communication.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub senders: Vec<Name>,

	/// The publication status of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub status: Option<PublicationStatus>,

	/// The term being referenced if the work is a dictionary or encyclopedia.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub term: Option<String>,

	/// The type of the thesis that is the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub thesis_type: Option<String>,

	/// The title of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,

	/// The translator(s) of a work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub translators: Vec<Name>,

	/// The URL of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub url: Option<Url>,

	/// The version of the work.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,

	/// The volume of the periodical in which a work appeared.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub volume: Option<String>,

	/// The title of the volume in which the work appeared.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub volume_title: Option<String>,

	/// The year in which a work has been published.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub year: Option<u64>,

	/// The year of the original publication.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub year_original: Option<i64>,
}

/// Publication statuses.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum PublicationStatus {
	Abstract,
	AdvanceOnline,
	InPreparation,
	InPress,
	Preprint,
	Submitted,
}

/// Types of referenced works.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum RefType {
	Art,
	Article,
	Audiovisual,
	Bill,
	Blog,
	Book,
	Catalogue,
	ConferencePaper,
	Conference,
	Data,
	Database,
	Dictionary,
	EditedWork,
	Encyclopedia,
	FilmBroadcast,
	Generic,
	GovernmentDocument,
	Grant,
	Hearing,
	HistoricalWork,
	LegalCase,
	LegalRule,
	MagazineArticle,
	Manual,
	Map,
	Multimedia,
	Music,
	NewspaperArticle,
	Pamphlet,
	Patent,
	PersonalCommunication,
	Proceedings,
	Report,
	Serial,
	Slides,
	SoftwareCode,
	SoftwareContainer,
	SoftwareExecutable,
	SoftwareVirtualMachine,
	Software,
	SoundRecording,
	Standard,
	Statute,
	Thesis,
	Unpublished,
	Video,
	Website,
}

impl Default for RefType {
	fn default() -> Self {
		Self::Generic
	}
}
