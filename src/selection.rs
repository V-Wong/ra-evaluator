use crate::{Expression, Relation};

/// Filters the tuples in ``expression`` that satisfy ``predicate``.
#[derive(Clone)]
pub struct Selection<S, E>
where
    S: Relation,
    E: Expression<S>,
{
    expression: E,
    predicate: fn(&S) -> bool
}

impl<S, E> Selection<S, E>
where
    S: Relation,
    E: Expression<S>, 
{
    pub fn new(expression: E, predicate: fn(&S) -> bool) -> Self {
        Self {
            expression,
            predicate
        }
    }
}

impl<S, E> Expression<S> for Selection<S, E>
where
    S: Relation,
    E: Expression<S>,
{
    fn eval(&self) -> Vec<S> {
        self.expression.eval().into_iter().filter(self.predicate).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Terminal;

    #[test]
    fn keep_all_rows() {
        let values = &[
            (1, "test string", 123),
            (2, "another string", 25)
        ];

        assert_eq!(Selection::new(Terminal::new(values), |_| true).eval(), values);
    }

    #[test]
    fn keep_zero_rows() {
        let values = &[
            (1, "test string", 123),
            (2, "another string", 25)
        ];

        assert_eq!(Selection::new(Terminal::new(values), |_| false).eval(), &[]);
    }

    #[test]
    fn keep_some_rows() {
        let values = &[
            (1, "test string", 123),
            (2, "another string", 25),
            (3, "yes another string", -50)
        ];

        assert_eq!(
            Selection::new(
                Terminal::new(values), 
                |x| x.0 >= 2 && x.2 > 0
            ).eval(),
            &[(2, "another string", 25)]
        );
    }
}