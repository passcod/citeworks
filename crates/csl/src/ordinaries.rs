//! Types and utilities for ordinary values.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrdinaryValue {
	/// String values
	String(String),

	/// Numeric values
	Number(serde_json::Number),
}

impl OrdinaryValue {
	pub fn as_str(&self) -> Option<&str> {
		if let Self::String(str) = self {
			Some(str.as_ref())
		} else {
			None
		}
	}

	pub fn as_number(&self) -> Option<&serde_json::Number> {
		if let Self::Number(num) = self {
			Some(num)
		} else {
			None
		}
	}

	pub fn as_i64(&self) -> Option<i64> {
		self.as_number().and_then(|f| f.as_i64())
	}

	pub fn as_u64(&self) -> Option<u64> {
		self.as_number().and_then(|f| f.as_u64())
	}

	pub fn as_f64(&self) -> Option<f64> {
		self.as_number().and_then(|f| f.as_f64())
	}
}
