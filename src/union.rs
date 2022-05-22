use crate::Expression;

use std::marker::PhantomData;

/// The relational algebra operation for taking the union of two relations.
/// Note that this is operation uses bag semantics and so duplicates can
// appear in the result.
pub struct Union<S: Clone, E>
where
    E: Expression<S>,
{
    pub left_expression: E,
    pub right_expression: E,
    phantom: PhantomData<S>,
}

impl<S: Clone, E> Union<S, E>
where
    E: Expression<S>,
{
    pub fn new(left_expression: E, right_expression: E) -> Self {
        Self {
            left_expression,
            right_expression,
            phantom: PhantomData,
        }
    }
}

impl<S: Clone, E> Expression<S> for Union<S, E>
where
    E: Expression<S, Output = S>,
{
    type Output = S;

    fn eval(&self) -> Vec<S> {
        let mut left_result = (self.left_expression).eval();
        let mut right_result = (self.right_expression).eval();

        left_result.append(&mut right_result);
        left_result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Terminal;

    #[test]
    fn union_right_identity() {
        let values = &[(1, "test string", 123.4), (2, "another string", 25.6)];

        assert_eq!(
            Union::new(Terminal::new(values), Terminal::new(&[])).eval(),
            values
        );
    }

    #[test]
    fn union_left_identity() {
        let values = &[(1, "test string", 123.4), (2, "another string", 25.6)];

        assert_eq!(
            Union::new(Terminal::new(&[]), Terminal::new(values)).eval(),
            values
        );
    }

    #[test]
    fn union_multiple() {
        let values1 = &[(1, "test string", 123.4), (2, "another string", 25.6)];
        let values2 = &[(3, "test string", 123.4), (4, "another string", 25.6)];

        let expected_result = &[
            (1, "test string", 123.4),
            (2, "another string", 25.6),
            (3, "test string", 123.4),
            (4, "another string", 25.6),
        ];

        assert_eq!(
            Union::new(Terminal::new(values1), Terminal::new(values2)).eval(),
            expected_result
        )
    }
}
