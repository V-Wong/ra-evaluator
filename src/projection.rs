use crate::Expression;

/// The relational algebra operation responsible for picking a subset of columns.
/// This is technically a generalized projection in that arbitrary mappings
/// can be performed over the columns through the mapper function.
#[derive(Clone)]
pub struct Projection<S, T, E>
where
    S: Clone + Eq + PartialEq,
    T: Clone + Eq + PartialEq,
    E: Expression<S>,
{
    pub expression: E,
    pub mapper: fn(&S) -> T
}

impl<S, T, E> Projection<S, T, E>
where
    S: Clone + Eq + PartialEq,
    T: Clone + Eq + PartialEq,
    E: Expression<S>, 
{
    pub fn new(expression: E, mapper: fn(&S) -> T) -> Self {
        Self {
            expression,
            mapper
        }
    }
}

impl<S, T, E> Expression<T> for Projection<S, T, E>
where
    S: Clone + Eq + PartialEq,
    T: Clone + Eq + PartialEq,
    E: Expression<S>,
{
    fn eval(&self) -> Vec<T> {
        self.expression.eval().iter().map(self.mapper).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Terminal;

    #[test]
    fn keep_all_columns() {
        let values = &[
            (1, "test string", 123),
            (2, "another string", 25)
        ];

        assert_eq!(Projection::new(Terminal::new(values), |x| *x).eval(), values);
    }

    #[test]
    fn keep_zero_columns() {
        let values = &[
            (1, "test string", 123),
            (2, "another string", 25)
        ];

        assert_eq!(Projection::new(Terminal::new(values), |_| ()).eval(), &[(), ()]);
    }

    #[test]
    fn keep_some_columns() {
        let values = &[
            (1, "test string", 123),
            (2, "another string", 25)
        ];

        let expected_result = &[
            ("test string", 123),
            ("another string", 25)
        ];

        assert_eq!(Projection::new(Terminal::new(values), |x| (x.1, x.2)).eval(), expected_result);
    }
}