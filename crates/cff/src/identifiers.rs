//! Types and utilities for identifiers e.g. DOIs.

use serde::{Deserialize, Serialize};
use url::Url;

/// An identifier for a work.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Identifier {
	/// DOI
	Doi {
		/// The value of the DOI.
		/// This is not parsed or validated in any way.
		///
		/// It should be the bare value of the DOI, not its URL or URI.
		/// E.g. `10.5281/zenodo.1003149`.
		value: String,

		/// Optional description.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		description: Option<String>,
	},

	/// URL
	Url {
		/// The value of the URL.
		value: Url,

		/// Optional description.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		description: Option<String>,
	},

	/// Software Heritage identifier
	Swh {
		/// The value of the Software Heritage identifier.
		/// This is not parsed or validated in any way.
		///
		/// E.g. `swh:1:dir:bc286860f423ea7ced246ba7458eef4b4541cf2d`.
		value: String,

		/// Optional description.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		description: Option<String>,
	},

	/// Some other identifier.
	Other {
		/// The value of the identifier.
		/// This is not parsed or validated in any way.
		///
		/// E.g. `arXiv:2103.06681`.
		value: String,

		/// Optional description.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		description: Option<String>,
	},
}
