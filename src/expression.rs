/// Defines relational algebraic expressions over generic tuple-based relations.
/// 
/// Implemented by operations such as Projection, Selection, Join, etc.
/// Expressions are intended to be recursive and contain sub-expressions.

use crate::Relation;

pub trait Expression<Output>: Clone 
where
    Output: Relation,
{
    /// Calls evaluation on any sub-expressions before performing its 
    /// own transformation and returning the (unwrapped) result.
    fn eval(&self) -> Vec<Output>;
}