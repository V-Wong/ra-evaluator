use crate::{Expression, Join};

/// The relational algebra for taking the Cartesian product of two relations.
/// This is equivalent to unconditionally joining the two relations.
pub struct CartesianProduct<L: Clone, R: Clone, Res: Clone, EL, ER>
where
    EL: Expression<L>,
    ER: Expression<R>,
{
    pub joiner: Join<L, R, Res, EL, ER>,
}

impl<L: Clone, R: Clone, Res: Clone, EL, ER> CartesianProduct<L, R, Res, EL, ER>
where
    EL: Expression<L>,
    ER: Expression<R>,
{
    pub fn new(left_expression: EL, right_expression: ER, mapper: fn(&L, &R) -> Res) -> Self {
        Self {
            joiner: Join::new(left_expression, right_expression, |_, _| true, mapper),
        }
    }
}

impl<L: Clone, R: Clone, Res: Clone, EL, ER> Expression<Res> for CartesianProduct<L, R, Res, EL, ER>
where
    EL: Expression<L, Output = L>,
    ER: Expression<R, Output = R>,
{
    type Output = Res;

    fn eval(&self) -> Vec<Res> {
        self.joiner.eval()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Terminal;

    #[test]
    fn cartesian_product_homogenous_types() {
        let values1 = &[(1, "test string", 123.4), (2, "another string", 25.6)];
        let values2 = &[(1, "test string", 123.4), (2, "another string", 25.6)];

        let expected_result = &[
            (1, "test string", 123.4, 1, "test string", 123.4),
            (1, "test string", 123.4, 2, "another string", 25.6),
            (2, "another string", 25.6, 1, "test string", 123.4),
            (2, "another string", 25.6, 2, "another string", 25.6),
        ];

        assert_eq!(
            CartesianProduct::new(Terminal::new(values1), Terminal::new(values2), |x, y| (
                x.0, x.1, x.2, y.0, y.1, y.2
            ),)
            .eval(),
            expected_result
        );
    }

    #[test]
    fn cartesian_product_heterogenous_types() {
        let values1 = &[(1, "test string", 123.4), (2, "another string", 25.6)];
        let values2 = &[("a", 1), ("b", 2)];

        let expected_result = &[
            (1, "test string", 123.4, "a", 1),
            (1, "test string", 123.4, "b", 2),
            (2, "another string", 25.6, "a", 1),
            (2, "another string", 25.6, "b", 2),
        ];

        assert_eq!(
            CartesianProduct::new(Terminal::new(values1), Terminal::new(values2), |x, y| (
                x.0, x.1, x.2, y.0, y.1
            ),)
            .eval(),
            expected_result
        );
    }
}
