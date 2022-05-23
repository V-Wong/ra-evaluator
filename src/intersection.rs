use crate::Expression;

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;

/// The relational algebra operation for taking the intersection of two relations.
/// Note that this is operation uses bag semantics and so duplicates can
// appear in the result.
#[derive(Clone)]
pub struct Intersection<S: Clone, E1, E2>
where
    E1: Expression<S>,
    E2: Expression<S>,
{
    pub left_expression: E1,
    pub right_expression: E2,
    phantom: PhantomData<S>,
}

impl<S, E1, E2> Intersection<S, E1, E2>
where
    S: Clone + Eq + PartialEq + Hash,
    E1: Expression<S>,
    E2: Expression<S>,
{
    pub fn new(left_expression: E1, right_expression: E2) -> Self {
        Self {
            left_expression,
            right_expression,
            phantom: PhantomData,
        }
    }
}

impl<S, E1, E2> Expression<S> for Intersection<S, E1, E2>
where
    S: Clone + Eq + PartialEq + Hash,
    E1: Expression<S, Output = S>,
    E2: Expression<S, Output = S>,
{
    type Output = S;

    fn eval(&self) -> Vec<S> {
        let left_result = (self.left_expression).eval();
        let right_result: HashSet<S> = HashSet::from_iter((self.right_expression).eval());

        let mut result = Vec::new();

        for row in left_result {
            if right_result.contains(&row) {
                result.push(row);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Terminal;

    #[test]
    fn empty_intersection() {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        struct Tuple(i32, String, i32);

        let values1 = &[
            Tuple(1, String::from("test string"), 123),
            Tuple(2, String::from("another string"), 25),
        ];
        let values2 = &[
            Tuple(3, String::from("test string"), 123),
            Tuple(4, String::from("another string"), 25),
        ];

        assert_eq!(
            Intersection::new(Terminal::new(values1), Terminal::new(values2)).eval(),
            &[]
        );
    }

    #[test]
    fn full_intersection() {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        struct Tuple(i32, String, i32);

        let values1 = &[
            Tuple(1, String::from("test string"), 123),
            Tuple(2, String::from("another string"), 25),
        ];
        let values2 = &[
            Tuple(1, String::from("test string"), 123),
            Tuple(2, String::from("another string"), 25),
        ];

        assert_eq!(
            Intersection::new(Terminal::new(values1), Terminal::new(values2)).eval(),
            values1
        );
    }

    #[test]
    fn some_intersection() {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        struct Tuple(i32, String, i32);

        let values1 = &[
            Tuple(1, String::from("test string"), 123),
            Tuple(2, String::from("another string"), 25),
            Tuple(3, String::from("another string"), 25),
        ];
        let values2 = &[
            Tuple(123, String::from("test string"), 123),
            Tuple(2, String::from("another string"), 25),
            Tuple(3, String::from("another string"), 25),
        ];

        let expected_result = &[
            Tuple(2, String::from("another string"), 25),
            Tuple(3, String::from("another string"), 25),
        ];

        assert_eq!(
            Intersection::new(Terminal::new(values1), Terminal::new(values2)).eval(),
            expected_result
        );
    }
}
