use std::hash::Hash;

use serde::{Deserialize, Serialize};
use spdx::Expression;

/// License field value.
///
/// This may either be a single SPDX license expression, or a list of licenses
/// or expressions.
///
/// A list should be interpreted as being a single expression with members
/// joined with `OR`; this library does no such interpretation immediately, so
/// as to keep the format of the original document. However, the
/// [`License::to_expression`] method does this for convenience.
///
/// Note that `Hash`, `PartialEq`, and `Eq` are implemented in term of the
/// original strings for the expression. That is, the list of `Apache-2.0` and
/// `MIT` may not be equal or hash to the same as `Apache-2.0 OR MIT`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, try_from = "ExprInternal", into = "ExprInternal")]
pub enum License {
	/// A single SPDX license expression.
	Single(Expression),

	/// A set of SPDX license expressions (interpreted as joined by `OR`).
	AnyOf(Vec<Expression>),
}

impl License {
	/// Get a single SPDX expression for this License value.
	pub fn to_expression(&self) -> Expression {
		match self {
			Self::Single(exp) => exp.clone(),
			Self::AnyOf(exps) => Expression::parse(
				&exps
					.iter()
					.map(|exp| format!("({exp})"))
					.collect::<Vec<_>>()
					.join(" OR "),
			)
			.expect("if the original expressions parsed, this one will too"),
		}
	}
}

impl Hash for License {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.to_expression().to_string().hash(state)
	}
}

impl PartialEq for License {
	fn eq(&self, other: &Self) -> bool {
		self.to_expression().eq(&other.to_expression())
	}
}

impl Eq for License {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ExprInternal {
	Single(String),
	AnyOf(Vec<String>),
}

impl TryFrom<ExprInternal> for License {
	type Error = spdx::ParseError;

	fn try_from(value: ExprInternal) -> Result<Self, Self::Error> {
		match value {
			ExprInternal::Single(expr) => {
				let expr = Expression::parse(&expr)?;
				Ok(Self::Single(expr))
			}
			ExprInternal::AnyOf(exprs) => {
				let mut exps = Vec::with_capacity(exprs.len());
				for exp in exprs {
					exps.push(Expression::parse(&exp)?);
				}
				Ok(Self::AnyOf(exps))
			}
		}
	}
}

impl From<License> for ExprInternal {
	fn from(license: License) -> Self {
		match license {
			License::Single(exp) => Self::Single(exp.to_string()),
			License::AnyOf(exps) => Self::AnyOf(exps.into_iter().map(|e| e.to_string()).collect()),
		}
	}
}
