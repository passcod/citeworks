//! Types and utilities for ordinary values.

use std::{fmt::Display, hash::Hash};

use decorum::{cmp::FloatEq, hash::FloatHash};
use serde::{Deserialize, Serialize};

/// An ordinary value can either be numerical or a string.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrdinaryValue {
	/// Numeric values (floating)
	Float(f64),

	/// Numeric values (integers)
	Integer(i64),

	/// String values
	String(String),
}

impl OrdinaryValue {
	/// If the [OrdinaryValue] is a string, return it.
	pub fn as_str(&self) -> Option<&str> {
		if let Self::String(str) = self {
			Some(str.as_ref())
		} else {
			None
		}
	}

	/// If the [OrdinaryValue] is an integer, return it.
	pub fn as_i64(&self) -> Option<i64> {
		if let Self::Integer(num) = self {
			Some(*num)
		} else {
			None
		}
	}

	/// If the [OrdinaryValue] is a float, return it.
	pub fn as_f64(&self) -> Option<f64> {
		if let Self::Float(num) = self {
			Some(*num)
		} else {
			None
		}
	}
}

impl PartialEq for OrdinaryValue {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
			(Self::Float(l0), Self::Float(r0)) => l0.float_eq(r0),
			(Self::String(l0), Self::String(r0)) => l0 == r0,
			_ => false,
		}
	}
}

impl Eq for OrdinaryValue {}

impl Hash for OrdinaryValue {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		core::mem::discriminant(self).hash(state);
		match self {
			OrdinaryValue::Float(f) => f.float_hash(state),
			OrdinaryValue::Integer(i) => i.hash(state),
			OrdinaryValue::String(s) => s.hash(state),
		}
	}
}

impl Display for OrdinaryValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Float(n) => write!(f, "{}", n),
			Self::Integer(i) => write!(f, "{}", i),
			Self::String(s) => write!(f, "{}", s),
		}
	}
}
