use crate::Expression;

/// The relational algebra operation responsible for picking a subset of columns.
/// This is technically a generalized projection in that arbitrary mappings
/// can be performed over the columns through the mapper function.
pub struct Projection<S: Clone, T: Clone, E>
where
    E: Expression<S>,
{
    pub expression: E,
    pub mapper: fn(&S) -> T
}

impl<S: Clone, T: Clone, E> Projection<S, T, E>
where
    E: Expression<S>, {
    fn new(expression: E, mapper: fn(&S) -> T) -> Self {
        Self {
            expression,
            mapper
        }
    }
}

impl<S: Clone, T: Clone, E> Expression<T> for Projection<S, T, E>
where
    E: Expression<S, Output = S>,
{
    type Output = T;

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
            (1, "test string", 123.4),
            (2, "another string", 25.6)
        ];

        assert_eq!(Projection::new(Terminal::new(values), |x| *x).eval(), values);
    }

    #[test]
    fn keep_zero_columns() {
        let values = &[
            (1, "test string", 123.4),
            (2, "another string", 25.6)
        ];

        assert_eq!(Projection::new(Terminal::new(values), |_| ()).eval(), &[(), ()]);
    }

    #[test]
    fn keep_some_columns() {
        let values = &[
            (1, "test string", 123.4),
            (2, "another string", 25.6)
        ];

        let expected_result = &[
            ("test string", 123.4),
            ("another string", 25.6)
        ];

        assert_eq!(Projection::new(Terminal::new(values), |x| (x.1, x.2)).eval(), expected_result);
    }
}