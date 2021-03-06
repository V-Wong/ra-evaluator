use crate::Expression;

/// An identity expression to mark the end of an expression chain.
/// 
/// This is intended to be a basic and transparent wrapper of a relation
/// in order to start expression building.
#[derive(Clone)]
pub struct Terminal<S>
where
    S: Clone + Eq + PartialEq,
{
    rows: Vec<S>,
}

impl<S> Terminal<S> 
where
    S: Clone + Eq + PartialEq,
{
    pub fn new(rows: &[S]) -> Self {
        Self {
            rows: rows.to_vec(),
        }
    }
}

impl<S> Expression<S> for Terminal<S>
where 
    S: Clone + Eq + PartialEq,
{
    fn eval(&self) -> Vec<S> {
        self.rows.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_value() {
        assert_eq!((Terminal::new(&[1])).eval(), &[1]);
    }

    #[test]
    fn multiple_values() {
        let values = &[1, 2, 3, 4, 5, 6];

        assert_eq!((Terminal::new(values)).eval(), values);
    }

    #[test]
    fn complex_type() {
        let values = &[
            (1, "test string", 123),
            (2, "another string", 25)
        ];

        assert_eq!((Terminal::new(values)).eval(), values);
    }
}
