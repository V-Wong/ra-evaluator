use crate::*;

use std::marker::PhantomData;

/// Is a utility struct for building complex ``Expression``s.
/// 
/// Different relational algebra expressions can be "chained together"
/// to build more complex relational algebra expressions.
/// The ``eval`` method can then be called at the end to evaluate the relational
/// algebra transformations sequentially and obtain a result.
pub struct ExpressionBuilder<S, E>
where
    S: Relation,
    E: Expression<S>,
{
    expression: E,
    phantom: PhantomData<S>,
}

impl<S, E> ExpressionBuilder<S, E>
where
    S: Relation,
    E: Expression<S>,
{
    pub fn new<T>(expression: E) -> ExpressionBuilder<T, E>
    where
        T: Relation,
        E: Expression<T>
    {
        ExpressionBuilder {
            expression,
            phantom: PhantomData,
        }
    }

    pub fn project<T: Clone>(
        &self,
        mapper: fn(&S) -> T,
    ) -> ExpressionBuilder<T, Projection<S, T, E>>
    where
        T: Relation
    {
        ExpressionBuilder {
            expression: Projection::new(self.expression.clone(), mapper),
            phantom: PhantomData,
        }
    }

    pub fn select(&self, predicate: fn(&S) -> bool) -> ExpressionBuilder<S, Selection<S, E>> {
        ExpressionBuilder {
            expression: Selection::new(self.expression.clone(), predicate),
            phantom: PhantomData,
        }
    }

    pub fn join<R, Res>(
        &self,
        right_relation: &[R],
        predicate: fn(&S, &R) -> bool,
        mapper: fn(&S, &R) -> Res,
    ) -> ExpressionBuilder<Res, Join<S, R, Res, E, Terminal<R>>>
    where
        R: Relation,
        Res: Relation,
    {
        ExpressionBuilder {
            expression: Join::new(
                self.expression.clone(),
                Terminal::new(right_relation.clone()),
                predicate,
                mapper,
            ),
            phantom: PhantomData,
        }
    }

    pub fn union(&self, right_relation: &[S]) -> ExpressionBuilder<S, Union<S, E, Terminal<S>>> {
        ExpressionBuilder {
            expression: Union::new(self.expression.clone(), Terminal::new(right_relation)),
            phantom: PhantomData,
        }
    }

    pub fn intersect(
        &self,
        right_relation: &[S],
    ) -> ExpressionBuilder<S, Intersection<S, E, Terminal<S>>> {
        ExpressionBuilder {
            expression: Intersection::new(self.expression.clone(), Terminal::new(right_relation)),
            phantom: PhantomData,
        }
    }

    pub fn cartesian_product<R, Res>(
        &self,
        right_relation: &[R],
        mapper: fn(&S, &R) -> Res,
    ) -> ExpressionBuilder<Res, CartesianProduct<S, R, Res, E, Terminal<R>>>
    where
        R: Relation,
        Res: Relation,
    {
        ExpressionBuilder {
            expression: CartesianProduct::new(
                self.expression.clone(),
                Terminal::new(right_relation),
                mapper
            ),
            phantom: PhantomData,
        }
    }

    pub fn eval(&self) -> Vec<S> {
        self.expression.eval()
    }
}
