use ra_evaluator::*;

#[test]
fn identity_transformation() {
    let values = &[1];

    let result = ExpressionBuilder::new(Terminal::new(values)).eval();

    assert_eq!(result, values);
}

#[test]
fn type_preserved_transformations() {
    let values1 = &[(1, "a"), (2, "b"), (3, "c")];
    let values2 = &[(4, "d")];
    let values3 = &[(3, "c"), (4, "d"), (5, "e")];

    let expected_result = &[(3, "c"), (4, "d")];

    let result = ExpressionBuilder::new(Terminal::new(values1))
        .select(|x| x.0 > 1)
        .union(values2)
        .intersect(values3)
        .eval();

    assert_eq!(result, expected_result);
}

#[test]
fn type_changing_transformations() {
    let values1 = &[(1, "a"), (2, "b"), (3, "c")];
    let values2 = &[("Concatenated", 1), ("Concatenated", 2)];
    let values3 = &["x", "y"];

    let expected_result = &[
        (1, "Concatenated", "x"),
        (1, "Concatenated", "y"),
        (2, "Concatenated", "x"),
        (2, "Concatenated", "y"),
    ];

    let result = ExpressionBuilder::new(Terminal::new(values1))
        .project(|x| x.0)
        .join(values2, |x, y| *x == y.1, |x, y| (*x, y.0))
        .cartesian_product(values3, |x, y| (x.0, x.1, *y))
        .eval();

    assert_eq!(result, expected_result);
}

#[test]
fn complex_transformations() {
    let values1 = &[(1, "a"), (2, "b"), (3, "c")];
    let values2 = &[1, 2];
    let values3 = &[(1, "Join1"), (2, "Join2")];
    let values4 = &[("d", 3, "Union")];
    let values5 = &[
        ("c", 1, "Join1"),
        ("c", 2, "Join2"),
        ("d", 3, "Union"),
        ("e", 4, "Removed"),
    ];

    let expected_result = &[("c", 1, "Join1"), ("c", 2, "Join2"), ("d", 3, "Union")];

    let result = ExpressionBuilder::new(Terminal::new(values1))
        .select(|x| x.0 > 1)
        .project(|x| x.1)
        .cartesian_product(values2, |x, y| (*x, *y))
        .join(values3, |x, y| x.1 == y.0, |x, y| (x.0, y.0, y.1))
        .union(values4)
        .intersect(values5)
        .eval();

    assert_eq!(result, expected_result);
}
