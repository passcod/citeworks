//! [Citation File Format](https://citation-file-format.github.io) serde types and implementations.
//!
//! This targets CFF 1.2.0 but may not support the entire specification.
//!
//! The top level API mimics [serde_yaml]'s:
//!
//! ```
//! let cff = citeworks_cff::from_str(r#"
//! cff-version: 1.2.0
//! message:
//!   If you dare use this commercial, closed-source, strangely versioned
//!   software in your research, please at least cite it as below.
//! authors:
//!   - family-names: Vader
//!     name-suffix: né Skywalker
//!     given-names: 'Anakin "Darth"'
//!   - name: anonymous
//! title: Opaquity
//! version: opq-1234-XZVF-ACME-RLY
//! date-released: 2017-02-28
//! url: http://www.opaquity.com/
//! contact:
//!   - name: Dark Side Software
//!     address: DS-1 Orbital Battle Station, near Scarif
//!     email: father@imperial-empire.com
//!     tel: +850 (0)123-45-666
//! "#).unwrap();
//!
//! assert_eq!(
//!     cff
//!         .authors[0]
//!         .as_person()
//!         .and_then(|his| his.family_names.as_deref()),
//!     Some("Vader")
//! );
//! ```
#![warn(clippy::unwrap_used, missing_docs)]
#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

use std::io::{Read, Write};

pub use serde_yaml::Result;

#[doc(inline)]
pub use cff::{Cff, WorkType};
#[doc(inline)]
pub use date::Date;
#[doc(inline)]
pub use license::License;

mod cff;
mod date;
pub mod identifiers;
mod license;
pub mod names;
pub mod references;

/// Deserialize CFF from an IO stream of YAML.
pub fn from_reader<R>(rdr: R) -> Result<Cff>
where
	R: Read,
{
	serde_yaml::from_reader(rdr)
}

/// Deserialize CFF from bytes of YAML text.
pub fn from_slice<'a>(v: &'a [u8]) -> Result<Cff> {
	serde_yaml::from_slice(v)
}

/// Deserialize CFF from a string of YAML text.
pub fn from_str<'a>(s: &'a str) -> Result<Cff> {
	serde_yaml::from_str(s)
}

/// Serialize the given CFF as a String of YAML.
pub fn to_string(value: &Cff) -> Result<String> {
	serde_yaml::to_string(value)
}

/// Serialize the given CFF as a YAML byte vector.
pub fn to_vec(value: &Cff) -> Result<Vec<u8>> {
	serde_yaml::to_string(value).map(|v| v.into_bytes())
}

/// Serialize the given CFF as YAML into the IO stream.
pub fn to_writer<W>(writer: W, value: &Cff) -> Result<()>
where
	W: Write,
{
	serde_yaml::to_writer(writer, value)
}
