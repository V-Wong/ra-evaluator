use crate::Expression;

/// The most basic operation for a relation; performs no transformations.
/// This is intended to be a basic and transparent wrapper of a relation
/// in order to start expression building.
pub struct Terminal<S: Clone> {
    rows: Vec<S>,
}

impl<S: Clone> Terminal<S> {
    pub fn new(rows: &[S]) -> Self {
        Self {
            rows: rows.to_vec(),
        }
    }
}

impl<S: Clone> Expression<S> for Terminal<S> {
    type Output = S;

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
            (1, "test string", 123.4),
            (2, "another string", 25.6)
        ];

        assert_eq!((Terminal::new(values)).eval(), values);
    }
}
