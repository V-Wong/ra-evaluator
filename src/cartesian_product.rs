use crate::{Expression, Join};

/// The relational algebra operation for taking the Cartesian product of two relations.
/// 
/// This is equivalent to unconditionally joining the two relations.
#[derive(Clone)]
pub struct CartesianProduct<L, R, Res, EL, ER>
where
    L: Clone + Eq + PartialEq,
    R: Clone + Eq + PartialEq,
    Res: Clone + Eq + PartialEq,
    EL: Expression<L>,
    ER: Expression<R>,
{
    pub joiner: Join<L, R, Res, EL, ER>,
}

impl<L, R, Res, EL, ER> CartesianProduct<L, R, Res, EL, ER>
where
    L: Clone + Eq + PartialEq,
    R: Clone + Eq + PartialEq,
    Res: Clone + Eq + PartialEq,
    EL: Expression<L>,
    ER: Expression<R>,
{
    pub fn new(left_expression: EL, right_expression: ER, mapper: fn(&L, &R) -> Res) -> Self {
        Self {
            joiner: Join::new(left_expression, right_expression, |_, _| true, mapper),
        }
    }
}

impl<L, R, Res, EL, ER> Expression<Res> for CartesianProduct<L, R, Res, EL, ER>
where
    L: Clone + Eq + PartialEq,
    R: Clone + Eq + PartialEq,
    Res: Clone + Eq + PartialEq,
    EL: Expression<L>,
    ER: Expression<R>,
{
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
        let values1 = &[(1, "test string", 123), (2, "another string", 25)];
        let values2 = &[(1, "test string", 123), (2, "another string", 25)];

        let expected_result = &[
            (1, "test string", 123, 1, "test string", 123),
            (1, "test string", 123, 2, "another string", 25),
            (2, "another string", 25, 1, "test string", 123),
            (2, "another string", 25, 2, "another string", 25),
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
        let values1 = &[(1, "test string", 123), (2, "another string", 25)];
        let values2 = &[("a", 1), ("b", 2)];

        let expected_result = &[
            (1, "test string", 123, "a", 1),
            (1, "test string", 123, "b", 2),
            (2, "another string", 25, "a", 1),
            (2, "another string", 25, "b", 2),
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
