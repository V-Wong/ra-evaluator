//! A simple type-safe [relational algebra] evaluator.
//! 
//! Relational algebra provides the theoretical foundation for relational databases
//! and the SQL language. This library provides a means to build expressions in the
//! language of relational algebra and evaluate them to obtain a concrete result.
//! 
//! [relational algebra]: https://en.wikipedia.org/wiki/Relational_algebra
//! 
//! # Sample Usage
//! 
//! We can build relational algebra expressions using the ``ExpressionBuilder`` struct:
//! 
//! ```rust
//! use ra_evaluator::{ExpressionBuilder, Terminal};
//! 
//! let query = ExpressionBuilder::new(Terminal::new(&[(1, "a"), (2, "b"), (3, "c")]))
//!     .select(|x| x.0 > 1)
//!     .project(|x| x.1)
//!     .cartesian_product(&[1, 2], |x, y| (*x, *y))
//!     .join(&[(1, "Join1"), (2, "Join2")], |x, y| x.1 == y.0, |x, y| (x.0, y.0, y.1))
//!     .union(&[("d", 3, "Union")])
//!     .intersect(&[
//!         ("c", 1, "Join1"),
//!         ("c", 2, "Join2"),
//!         ("d", 3, "Union"),
//!         ("e", 4, "Removed"),
//!     ]);
//! 
//! // Results in ``[("c", 1, "Join1"), ("c", 2, "Join2"), ("d", 3, "Union")]``
//! println!("{:?}", query.eval());
//! 
//! ```

/// Any type that is cloneable and can be compared by equality is
/// a valid relation that can be operated on by relational algebra operations.
pub trait Relation: Clone + Eq + PartialEq {}
impl<T> Relation for T where T: Clone + Eq + PartialEq {}

mod expression;
mod expression_builder;
mod projection;
mod selection;
mod terminal;
mod join;
mod union;
mod intersection;
mod cartesian_product;

pub use expression::*;
pub use expression_builder::*;
pub use projection::*;
pub use selection::*;
pub use terminal::*;
pub use join::*;
pub use union::*;
pub use intersection::*;
pub use cartesian_product::*;