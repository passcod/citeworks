//! Citation Style Language serde types and implementations.
//!
//! This targets CSL 1.0.2 but may not support the entire specification.
//!
//! At the moment, only CSL-JSON items are covered.
//!
//! The top level API mimics [serde_json]'s:
//!
//! ```
//! let csl = citeworks_csl::from_str(r#"
//! [{
//!   "id": "example-id",
//!   "type": "report",
//!   "author": [
//!     {"given": "Jane", "family": "Roe"},
//!     {"literal": "John Doe"}
//!   ]
//! }]
//! "#).unwrap();
//!
//! assert_eq!(csl[0].author[0].given, Some("Jane".into()));
//! ```

use std::io::{Read, Write};

pub use serde_json::Result;

pub use items::Item;

pub mod dates;
pub mod items;
pub mod names;
pub mod ordinaries;

/// Deserialize CSL items from an IO stream of JSON.
pub fn from_reader<R>(rdr: R) -> Result<Vec<Item>>
where
	R: Read,
{
	serde_json::from_reader(rdr)
}

/// Deserialize CSL items from bytes of JSON text.
pub fn from_slice<'a>(v: &'a [u8]) -> Result<Vec<Item>> {
	serde_json::from_slice(v)
}

/// Deserialize CSL items from a string of JSON text.
pub fn from_str<'a>(s: &'a str) -> Result<Vec<Item>> {
	serde_json::from_str(s)
}

/// Serialize the given CSL items as a String of JSON.
pub fn to_string(value: &[Item]) -> Result<String> {
	serde_json::to_string(value)
}

/// Serialize the given CSL items as a pretty-printed String of JSON.
pub fn to_string_pretty(value: &[Item]) -> Result<String> {
	serde_json::to_string_pretty(value)
}

/// Serialize the given CSL items as a JSON byte vector.
pub fn to_vec(value: &[Item]) -> Result<Vec<u8>> {
	serde_json::to_vec(value)
}

/// Serialize the given CSL items as a pretty-printed JSON byte vector.
pub fn to_vec_pretty(value: &[Item]) -> Result<Vec<u8>> {
	serde_json::to_vec_pretty(value)
}

/// Serialize the given CSL items as JSON into the IO stream.
pub fn to_writer<W>(writer: W, value: &[Item]) -> Result<()> where
    W: Write
{
	serde_json::to_writer(writer, value)
}

/// Serialize the given CSL items as pretty-printed JSON into the IO stream.
pub fn to_writer_pretty<W>(writer: W, value: &[Item]) -> Result<()> where
    W: Write
{
	serde_json::to_writer_pretty(writer, value)
}
