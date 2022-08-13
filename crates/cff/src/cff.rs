use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
	date::Date, identifiers::Identifier, license::License, names::Name, references::Reference,
};

/// The top-level CFF document.
///
/// This contains the citation metadata for a project, and may also contain
/// reference information (the project's bibligraphy).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Cff {
	/// Version of the CFF specification this document conforms to.
	///
	/// This is required and must be non-empty.
	pub cff_version: Version,

	/// What to do with the citation metadata, in a human-readable message.
	///
	/// This is required and must be non-empty.
	///
	/// # Examples
	///
	/// - "If you use this software, please cite it using the metadata from this file."
	/// - "Please cite this software using these metadata."
	/// - "Please cite this software using the metadata from 'preferred-citation'."
	/// - "If you use this dataset, please cite it using the metadata from this file."
	/// - "Please cite this dataset using these metadata."
	/// - "Please cite this dataset using the metadata from 'preferred-citation'."
	pub message: String,

	/// The name of the work.
	///
	/// This is required and must be non-empty.
	pub title: String,

	/// The type of the work.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
	pub work_type: Option<WorkType>,

	/// Version of the work.
	///
	/// There is no requirement that this be semver.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,

	/// The commit hash or revision number of the software version.
	///
	/// By convention:
	/// - if this is a Git hash, it should be bare lowercase hex, e.g.
	///   `1ff847d81f29c45a3a1a5ce73d38e45c2f319bba`;
	/// - if this is a decimal revision or build number, it should be preceded
	///   by a label, e.g. `Revision: 8612`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub commit: Option<String>,

	/// The date the work has been released.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub date_released: Option<Date>,

	/// A description of the work.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "abstract")]
	pub abstract_text: Option<String>,

	/// Keywords that describe the work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub keywords: Vec<String>,

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

	/// [SPDX][spdx] license expression(s).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub license: Option<License>,

	/// The URL of the license text under which the work is licensed.
	///
	/// This should only be used for non-standard licenses not included in the
	/// SPDX License List.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub license_url: Option<Url>,

	/// The authors of the work.
	///
	/// This is required and must contain at least one author.
	pub authors: Vec<Name>,

	/// The contact person, group, company, etc. for the work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub contact: Vec<Name>,

	/// The DOI for the work.
	///
	/// Use this if the work has a single DOI. Otherwise, use the `identifiers`
	/// field.
	///
	/// Note that the DOI is not parsed or validated in any way.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub doi: Option<String>,

	/// The identifiers for the work.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub identifiers: Vec<Identifier>,

	/// A reference to another work that should be cited instead of the work.
	///
	/// Note that the principles of [software citation] and [data citation]
	/// require that software should be cited on the same basis as any other
	/// research product such as a paper or a book. Adding a different preferred
	/// citation may result in a violation of the respective primary principle,
	/// "Importance", when others cite this work.
	///
	/// [software citation]: https://doi.org/10.7717/peerj-cs.86
	/// [data citation]: https://doi.org/10.25490/a97f-egyk
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub preferred_citation: Option<Reference>,

	/// Reference(s) to other creative works.
	///
	/// Similar to a list of references in a paper, references of the software
	/// or dataset may include other software (dependencies), or other research
	/// products that the software or dataset builds on, but not work describing
	/// the software or dataset.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub references: Vec<Reference>,
}

impl Default for Cff {
	fn default() -> Self {
		Self {
			cff_version: Version::new(1, 2, 0),
			message: String::from("Please cite this software using these metadata."),
			title: Default::default(),
			work_type: Default::default(),
			version: Default::default(),
			commit: Default::default(),
			date_released: Default::default(),
			abstract_text: Default::default(),
			keywords: Default::default(),
			repository: Default::default(),
			repository_artifact: Default::default(),
			repository_code: Default::default(),
			license: Default::default(),
			license_url: Default::default(),
			authors: Default::default(),
			contact: Default::default(),
			doi: Default::default(),
			identifiers: Default::default(),
			preferred_citation: Default::default(),
			references: Default::default(),
		}
	}
}

/// Types of works recognised by CFF.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkType {
	/// A software project.
	Software,

	/// A dataset.
	Dataset,
}
