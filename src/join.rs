use crate::Expression;

/// The relational algebra operation responsible for joining two relations.
/// The mapper function is required in order to provide typing for
/// the joined result as Rust does not have a way to take the product of
/// two types without nesting types.
#[derive(Clone)]
pub struct Join<L: Clone, R: Clone, Res: Clone, EL, ER>
where
    EL: Expression<L>,
    ER: Expression<R>,
{
    pub left_expression: EL,
    pub right_expression: ER,
    pub predicate: fn(&L, &R) -> bool,
    pub mapper: fn(&L, &R) -> Res,
}

impl<L: Clone, R: Clone, Res: Clone, EL, ER> Join<L, R, Res, EL, ER>
where
    EL: Expression<L>,
    ER: Expression<R>,
{
    pub fn new(
        left_expression: EL,
        right_expression: ER,
        predicate: fn(&L, &R) -> bool,
        mapper: fn(&L, &R) -> Res,
    ) -> Self {
        Self {
            left_expression,
            right_expression,
            predicate,
            mapper,
        }
    }
}

impl<L: Clone, R: Clone, Res: Clone, EL, ER> Expression<Res>
    for Join<L, R, Res, EL, ER>
where
    EL: Expression<L, Output = L>,
    ER: Expression<R, Output = R>,
{
    type Output = Res;

    fn eval(&self) -> Vec<Res> {
        let left_result = self.left_expression.eval();
        let right_result = self.right_expression.eval();

        let mut result = Vec::new();

        for row1 in &left_result {
            for row2 in &right_result {
                if (self.predicate)(&row1, &row2) {
                    result.push((self.mapper)(&row1, &row2));
                }
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
    fn cartesian_product_homogenous_types() {
        let values1 = &[(1, "test string", 123.4), (2, "another string", 25.6)];
        let values2 = &[(1, "test string", 123.4), (2, "another string", 25.6)];

        let expected_result = &[
            (1, "test string", 123.4, 1, "test string", 123.4),
            (1, "test string", 123.4, 2, "another string", 25.6),
            (2, "another string", 25.6, 1, "test string", 123.4),
            (2, "another string", 25.6, 2, "another string", 25.6)
        ];

        assert_eq!(
            Join::new(
                Terminal::new(values1), 
                Terminal::new(values2), 
                |_, _| true,
                |x, y| (x.0, x.1, x.2, y.0, y.1, y.2),
            ).eval(),
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
            (2, "another string", 25.6, "b", 2)
        ];

        assert_eq!(
            Join::new(
                Terminal::new(values1), 
                Terminal::new(values2), 
                |_, _| true,
                |x, y| (x.0, x.1, x.2, y.0, y.1),
            ).eval(),
            expected_result
        );
    }

    #[test]
    fn conditional_join_on_key() {
        let values1 = &[(1, "test string", 123.4), (2, "another string", 25.6)];
        let values2 = &[("a", 1), ("b", 2)];

        let expected_result = &[
            (1, "test string", 123.4, "a", 1),
            (2, "another string", 25.6, "b", 2)
        ];

        assert_eq!(
            Join::new(
                Terminal::new(values1), 
                Terminal::new(values2), 
                |x, y| x.0 == y.1,
                |x, y| (x.0, x.1, x.2, y.0, y.1),
            ).eval(),
            expected_result
        );
    }

    #[test]
    fn empty_join() {
        let values1 = &[(1, "test string", 123.4), (2, "another string", 25.6)];
        let values2 = &[("a", 1), ("b", 2)];

        assert_eq!(
            Join::new(
                Terminal::new(values1), 
                Terminal::new(values2), 
                |x, y| false,
                |x, y| (x.0, x.1, x.2, y.0, y.1),
            ).eval(),
            &[]
        );
    }
}
